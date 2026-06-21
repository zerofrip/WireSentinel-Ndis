use crate::flow::{FlowDirection, FlowKey, FlowTracker};
use crate::packet::{IpProtocol, ParsedPacket};
use ndis_controller::{NdisRouteKind, ObfuscationPreset};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteHint {
    Direct,
    Vpn { profile_id: u64 },
    Proxy { profile_id: u64, socks_port: u16 },
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassificationResult {
    pub flow_id: Uuid,
    pub app_id: Option<Uuid>,
    pub protocol: IpProtocol,
    pub route: RouteHint,
    pub obfuscation: ObfuscationPreset,
}

pub struct PacketClassifier {
    app_by_process: HashMap<u32, Uuid>,
    app_routes: HashMap<Uuid, RouteHint>,
    app_obfuscation: HashMap<Uuid, ObfuscationPreset>,
    flows: FlowTracker,
}

impl Default for PacketClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketClassifier {
    pub fn new() -> Self {
        Self {
            app_by_process: HashMap::new(),
            app_routes: HashMap::new(),
            app_obfuscation: HashMap::new(),
            flows: FlowTracker::new(Duration::from_secs(120)),
        }
    }

    pub fn register_process(&mut self, pid: u32, app_id: Uuid) {
        self.app_by_process.insert(pid, app_id);
    }

    pub fn set_app_route(&mut self, app_id: Uuid, route: RouteHint) {
        self.app_routes.insert(app_id, route);
    }

    pub fn set_app_obfuscation(&mut self, app_id: Uuid, preset: ObfuscationPreset) {
        self.app_obfuscation.insert(app_id, preset);
    }

    pub fn classify_ipv4(
        &mut self,
        raw: &[u8],
        pid: u32,
        direction: FlowDirection,
    ) -> Option<ClassificationResult> {
        let packet = ParsedPacket::parse_ipv4(raw)?;
        self.classify_parsed(packet, pid, direction)
    }

    pub fn classify_ipv6(
        &mut self,
        raw: &[u8],
        pid: u32,
        direction: FlowDirection,
    ) -> Option<ClassificationResult> {
        let packet = ParsedPacket::parse_ipv6(raw)?;
        self.classify_parsed(packet, pid, direction)
    }

    pub fn active_flow_count(&self) -> usize {
        self.flows.active_flows()
    }

    fn classify_parsed(
        &mut self,
        packet: ParsedPacket,
        pid: u32,
        direction: FlowDirection,
    ) -> Option<ClassificationResult> {
        let key = FlowKey {
            protocol: packet.protocol,
            src_ip: packet.src_ip,
            dst_ip: packet.dst_ip,
            src_port: packet.src_port,
            dst_port: packet.dst_port,
            direction,
        };
        let mut record = self.flows.touch(key.clone(), packet.payload_len);
        let app_id = self.app_by_process.get(&pid).copied().or(record.app_id);
        if let Some(app) = app_id {
            self.flows.bind_app(&key, app);
            record.app_id = Some(app);
        }
        let route = app_id
            .and_then(|id| self.app_routes.get(&id).copied())
            .unwrap_or(RouteHint::Direct);
        let obfuscation = app_id
            .and_then(|id| self.app_obfuscation.get(&id).copied())
            .unwrap_or(ObfuscationPreset::Disabled);
        Some(ClassificationResult {
            flow_id: record.flow_id,
            app_id,
            protocol: packet.protocol,
            route,
            obfuscation,
        })
    }
}

impl RouteHint {
    pub fn to_route_kind(&self) -> NdisRouteKind {
        match self {
            RouteHint::Direct => NdisRouteKind::Direct,
            RouteHint::Vpn { .. } => NdisRouteKind::Vpn,
            RouteHint::Proxy { .. } => NdisRouteKind::Proxy,
            RouteHint::Blocked => NdisRouteKind::Blocked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_udp_packet() -> Vec<u8> {
        let mut pkt = vec![0u8; 28];
        pkt[0] = 0x45;
        pkt[9] = 17;
        pkt[12..16].copy_from_slice(&[192, 168, 1, 10]);
        pkt[16..20].copy_from_slice(&[1, 1, 1, 1]);
        pkt[20..22].copy_from_slice(&4444u16.to_be_bytes().as_ref());
        pkt[22..24].copy_from_slice(&853u16.to_be_bytes().as_ref());
        pkt
    }

    #[test]
    fn classifies_process_app_and_route() {
        let mut classifier = PacketClassifier::new();
        let app_id = Uuid::new_v4();
        classifier.register_process(4242, app_id);
        classifier.set_app_route(
            app_id,
            RouteHint::Vpn {
                profile_id: 99,
            },
        );
        classifier.set_app_obfuscation(app_id, ObfuscationPreset::Balanced);
        let result = classifier
            .classify_ipv4(&sample_udp_packet(), 4242, FlowDirection::Outbound)
            .expect("classified");
        assert_eq!(result.app_id, Some(app_id));
        assert_eq!(
            result.route,
            RouteHint::Vpn {
                profile_id: 99
            }
        );
        assert_eq!(result.obfuscation, ObfuscationPreset::Balanced);
        assert_eq!(result.protocol, IpProtocol::Udp);
        assert_eq!(classifier.active_flow_count(), 1);
    }

    #[test]
    fn defaults_to_direct_without_app_binding() {
        let mut classifier = PacketClassifier::new();
        let result = classifier
            .classify_ipv4(&sample_udp_packet(), 1, FlowDirection::Inbound)
            .expect("classified");
        assert_eq!(result.route, RouteHint::Direct);
        assert_eq!(result.obfuscation, ObfuscationPreset::Disabled);
    }
}
