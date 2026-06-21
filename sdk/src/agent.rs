use crate::{
    ClassificationResult, FlowDirection, KernelRouteAssignment, KernelTelemetryV2,
    KernelTransformEngine, NdisClient, ObfuscationPreset, PacketClassifier, RouteHint,
    RouteSyncEngine, SyncReport, TelemetryRecord,
};
use ndis_controller::TelemetryEventKind;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum NdisAgentError {
    #[error("driver unavailable: {0}")]
    Driver(#[from] ndis_controller::NdisError),
    #[error("route sync failed: {0}")]
    RouteSync(#[from] route_engine::RouteSyncError),
}

#[derive(Debug, Clone, Default)]
pub struct AgentSnapshot {
    pub active_flows: usize,
    pub route_count: usize,
    pub classify_count: u64,
}

pub struct NdisAgent {
    classifier: PacketClassifier,
    routes: RouteSyncEngine,
    transform: KernelTransformEngine,
    telemetry: KernelTelemetryV2,
}

impl Default for NdisAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl NdisAgent {
    pub fn new() -> Self {
        Self {
            classifier: PacketClassifier::new(),
            routes: RouteSyncEngine::new(),
            transform: KernelTransformEngine::from_preset(ObfuscationPreset::Disabled),
            telemetry: KernelTelemetryV2::new(),
        }
    }

    pub fn bind_process(&mut self, pid: u32, app_id: Uuid) {
        self.classifier.register_process(pid, app_id);
    }

    pub fn set_route(&mut self, app_id: Uuid, route: RouteHint) {
        self.classifier.set_app_route(app_id, route);
    }

    pub fn set_obfuscation(&mut self, app_id: Uuid, preset: ObfuscationPreset) {
        self.classifier.set_app_obfuscation(app_id, preset);
        self.transform = KernelTransformEngine::from_preset(preset);
    }

    pub fn classify_ipv4(
        &mut self,
        raw: &[u8],
        pid: u32,
        direction: FlowDirection,
    ) -> Option<ClassificationResult> {
        let result = self.classifier.classify_ipv4(raw, pid, direction)?;
        self.telemetry.record(TelemetryRecord {
            kind: TelemetryEventKind::Classify,
            flow_id: result.flow_id,
            process_id: pid,
            protocol: result.protocol.number() as u32,
            bytes: raw.len() as u64,
            result_code: 0,
            timestamp_100ns: 0,
        });
        if let (Some(app_id), RouteHint::Vpn { profile_id }) = (result.app_id, result.route) {
            let _ = self.routes.upsert(KernelRouteAssignment::vpn(
                result.flow_id,
                app_id,
                profile_id,
                0,
            ));
        }
        Some(result)
    }

    pub fn snapshot(&self) -> AgentSnapshot {
        AgentSnapshot {
            active_flows: self.classifier.active_flow_count(),
            route_count: self.routes.len(),
            classify_count: self.telemetry.summary().classify_count,
        }
    }

    pub fn sync_routes(&self, client: &NdisClient) -> Result<SyncReport, NdisAgentError> {
        Ok(self.routes.sync_to_kernel(client)?)
    }

    pub fn transform_payload(&self, payload: &[u8]) -> Vec<u8> {
        self.transform.transform(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_udp_packet() -> Vec<u8> {
        let mut pkt = vec![0u8; 28];
        pkt[0] = 0x45;
        pkt[9] = 17;
        pkt[12..16].copy_from_slice(&[10, 0, 0, 5]);
        pkt[16..20].copy_from_slice(&[8, 8, 4, 4]);
        pkt[20..22].copy_from_slice(&1000u16.to_be_bytes().as_ref());
        pkt[22..24].copy_from_slice(&443u16.to_be_bytes().as_ref());
        pkt
    }

    #[test]
    fn agent_classifies_and_tracks_snapshot() {
        let mut agent = NdisAgent::new();
        let app = Uuid::new_v4();
        agent.bind_process(900, app);
        agent.set_route(app, RouteHint::Vpn { profile_id: 3 });
        agent.set_obfuscation(app, ObfuscationPreset::Basic);
        let result = agent
            .classify_ipv4(&sample_udp_packet(), 900, FlowDirection::Outbound)
            .expect("classified");
        assert_eq!(result.app_id, Some(app));
        let snap = agent.snapshot();
        assert_eq!(snap.active_flows, 1);
        assert_eq!(snap.route_count, 1);
        assert_eq!(snap.classify_count, 1);
    }
}
