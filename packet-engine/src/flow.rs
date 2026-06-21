use crate::packet::IpProtocol;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowDirection {
    Outbound,
    Inbound,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlowKey {
    pub protocol: IpProtocol,
    pub src_ip: IpAddr,
    pub dst_ip: IpAddr,
    pub src_port: u16,
    pub dst_port: u16,
    pub direction: FlowDirection,
}

#[derive(Debug, Clone)]
pub struct FlowRecord {
    pub key: FlowKey,
    pub flow_id: Uuid,
    pub app_id: Option<Uuid>,
    pub route_hint: Option<u64>,
    pub packets: u64,
    pub bytes: u64,
    pub first_seen: Instant,
    pub last_seen: Instant,
}

pub struct FlowTracker {
    flows: HashMap<FlowKey, FlowRecord>,
    idle_timeout: Duration,
}

impl FlowTracker {
    pub fn new(idle_timeout: Duration) -> Self {
        Self {
            flows: HashMap::new(),
            idle_timeout,
        }
    }

    pub fn touch(&mut self, key: FlowKey, payload_len: usize) -> FlowRecord {
        let now = Instant::now();
        self.evict_idle(now);
        let flow_id = self
            .flows
            .get(&key)
            .map(|f| f.flow_id)
            .unwrap_or_else(Uuid::new_v4);
        self.flows
            .entry(key.clone())
            .and_modify(|record| {
                record.packets += 1;
                record.bytes += payload_len as u64;
                record.last_seen = now;
            })
            .or_insert_with(|| FlowRecord {
                key: key.clone(),
                flow_id,
                app_id: None,
                route_hint: None,
                packets: 1,
                bytes: payload_len as u64,
                first_seen: now,
                last_seen: now,
            });
        self.flows.get(&key).expect("flow inserted").clone()
    }

    pub fn bind_app(&mut self, key: &FlowKey, app_id: Uuid) {
        if let Some(record) = self.flows.get_mut(key) {
            record.app_id = Some(app_id);
        }
    }

    pub fn set_route_hint(&mut self, key: &FlowKey, profile_id: u64) {
        if let Some(record) = self.flows.get_mut(key) {
            record.route_hint = Some(profile_id);
        }
    }

    pub fn active_flows(&self) -> usize {
        self.flows.len()
    }

    pub fn get(&self, key: &FlowKey) -> Option<&FlowRecord> {
        self.flows.get(key)
    }

    fn evict_idle(&mut self, now: Instant) {
        self.flows
            .retain(|_, record| now.duration_since(record.last_seen) <= self.idle_timeout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn tracks_flow_counters() {
        let mut tracker = FlowTracker::new(Duration::from_secs(60));
        let key = FlowKey {
            protocol: IpProtocol::Tcp,
            src_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            dst_ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
            src_port: 50000,
            dst_port: 443,
            direction: FlowDirection::Outbound,
        };
        tracker.touch(key.clone(), 100);
        tracker.touch(key.clone(), 50);
        let record = tracker.get(&key).unwrap();
        assert_eq!(record.packets, 2);
        assert_eq!(record.bytes, 150);
    }
}
