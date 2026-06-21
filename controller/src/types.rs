//! Binary layouts — mirrors `shared/structures/*.h`.

use std::mem::size_of;

pub const NDIS_DRIVER_STATE_VERSION: u32 = 2;
pub const NDIS_ROUTE_ASSIGNMENT_VERSION: u32 = 2;
pub const NDIS_TELEMETRY_VERSION: u32 = 2;
pub const NDIS_TRANSFORM_PROFILE_VERSION: u32 = 2;
pub const NDIS_COVER_TRAFFIC_VERSION: u32 = 2;
pub const NDIS_REDIRECT_RULE_VERSION: u32 = 2;
pub const NDIS_MAX_TRANSFORM_MODULES: usize = 8;
pub const NDIS_TELEMETRY_RING_CAPACITY: u32 = 256;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NdisLifecycleState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Paused = 3,
    Failed = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NdisRouteKind {
    Direct = 0,
    Vpn = 1,
    Tailnet = 2,
    Tor = 3,
    Anonymous = 4,
    Blocked = 5,
    Proxy = 6,
    Chain = 7,
    Katzenpost = 8,
    Loopix = 9,
    FederatedMixnet = 10,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObfuscationPreset {
    Disabled = 0,
    Basic = 1,
    Balanced = 2,
    Aggressive = 3,
    Lwo = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformModuleKind {
    Padding = 0,
    Jitter = 1,
    Fragment = 2,
    Camouflage = 3,
    Lwo = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverTrafficMode {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectAction {
    Pass = 0,
    Inject = 1,
    Drop = 2,
    Loopback = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryEventKind {
    Classify = 0,
    Redirect = 1,
    Transform = 2,
    CoverTraffic = 3,
    Error = 4,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NdisDriverStateV2 {
    pub version: u32,
    pub version_major: u32,
    pub version_minor: u32,
    pub version_patch: u32,
    pub build_stamp: u32,
    pub lifecycle_state: u32,
    pub filter_attached: u32,
    pub active_route_count: u32,
    pub active_redirect_count: u32,
    pub last_error: i32,
    pub reserved: [u8; 12],
}

impl NdisDriverStateV2 {
    pub const SIZE: usize = size_of::<Self>();
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NdisRouteAssignmentV2 {
    pub version: u32,
    pub flow_id: [u8; 16],
    pub app_id: [u8; 16],
    pub route_kind: u32,
    pub profile_id: u64,
    pub interface_luid: u64,
    pub socks_port: u16,
    pub protocol: u16,
    pub reserved: [u8; 4],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NdisTransformModuleV2 {
    pub kind: u32,
    pub parameter0: u32,
    pub parameter1: u32,
    pub reserved: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NdisTransformProfileV2 {
    pub version: u32,
    pub preset: u32,
    pub module_count: u32,
    pub modules: [NdisTransformModuleV2; NDIS_MAX_TRANSFORM_MODULES],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NdisCoverTrafficProfileV2 {
    pub version: u32,
    pub mode: u32,
    pub min_interval_ms: u32,
    pub max_interval_ms: u32,
    pub min_payload_bytes: u32,
    pub max_payload_bytes: u32,
    pub burst_count: u32,
    pub enabled: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NdisRedirectRuleV2 {
    pub version: u32,
    pub flow_id: [u8; 16],
    pub action: u32,
    pub target_interface_luid: u64,
    pub target_port: u16,
    pub protocol: u16,
    pub reserved: [u8; 4],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NdisTelemetryEventV2 {
    pub version: u32,
    pub kind: u32,
    pub timestamp_100ns: u64,
    pub flow_id: [u8; 16],
    pub process_id: u32,
    pub protocol: u32,
    pub bytes: u64,
    pub result_code: i32,
    pub reserved: [u8; 4],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NdisTelemetrySummaryV2 {
    pub version: u32,
    pub classify_count: u64,
    pub redirect_count: u64,
    pub transform_count: u64,
    pub cover_traffic_count: u64,
    pub error_count: u64,
    pub dropped_count: u64,
    pub avg_classify_latency_100ns: u64,
    pub max_classify_latency_100ns: u64,
    pub ring_head: u32,
    pub ring_tail: u32,
    pub ring_capacity: u32,
    pub pending_events: u32,
}

pub fn uuid_to_bytes(id: uuid::Uuid) -> [u8; 16] {
    *id.as_bytes()
}

impl NdisRouteAssignmentV2 {
    pub fn new_vpn(flow_id: uuid::Uuid, app_id: uuid::Uuid, profile_id: u64, interface_luid: u64) -> Self {
        Self {
            version: NDIS_ROUTE_ASSIGNMENT_VERSION,
            flow_id: uuid_to_bytes(flow_id),
            app_id: uuid_to_bytes(app_id),
            route_kind: NdisRouteKind::Vpn as u32,
            profile_id,
            interface_luid,
            socks_port: 0,
            protocol: 0,
            reserved: [0; 4],
        }
    }

    pub fn new_proxy(flow_id: uuid::Uuid, app_id: uuid::Uuid, profile_id: u64, socks_port: u16) -> Self {
        Self {
            version: NDIS_ROUTE_ASSIGNMENT_VERSION,
            flow_id: uuid_to_bytes(flow_id),
            app_id: uuid_to_bytes(app_id),
            route_kind: NdisRouteKind::Proxy as u32,
            profile_id,
            interface_luid: 0,
            socks_port,
            protocol: 0,
            reserved: [0; 4],
        }
    }
}

impl NdisTransformProfileV2 {
    pub fn from_preset(preset: ObfuscationPreset) -> Self {
        let modules = preset_modules(preset);
        let mut buf = [NdisTransformModuleV2 {
            kind: 0,
            parameter0: 0,
            parameter1: 0,
            reserved: 0,
        }; NDIS_MAX_TRANSFORM_MODULES];
        for (idx, module) in modules.iter().enumerate() {
            buf[idx] = *module;
        }
        Self {
            version: NDIS_TRANSFORM_PROFILE_VERSION,
            preset: preset as u32,
            module_count: modules.len() as u32,
            modules: buf,
        }
    }
}

pub fn preset_modules(preset: ObfuscationPreset) -> Vec<NdisTransformModuleV2> {
    let mk = |kind: TransformModuleKind| NdisTransformModuleV2 {
        kind: kind as u32,
        parameter0: 0,
        parameter1: 0,
        reserved: 0,
    };
    match preset {
        ObfuscationPreset::Disabled => Vec::new(),
        ObfuscationPreset::Basic => vec![mk(TransformModuleKind::Padding)],
        ObfuscationPreset::Balanced => vec![
            mk(TransformModuleKind::Padding),
            mk(TransformModuleKind::Jitter),
        ],
        ObfuscationPreset::Aggressive => vec![
            mk(TransformModuleKind::Fragment),
            mk(TransformModuleKind::Padding),
            mk(TransformModuleKind::Jitter),
            mk(TransformModuleKind::Camouflage),
        ],
        ObfuscationPreset::Lwo => vec![mk(TransformModuleKind::Lwo)],
    }
}

impl NdisCoverTrafficProfileV2 {
    pub fn from_mode(mode: CoverTrafficMode) -> Self {
        let (min_ms, max_ms, min_payload, max_payload, burst, enabled) = match mode {
            CoverTrafficMode::Off => (0, 0, 0, 0, 0, 0),
            CoverTrafficMode::Low => (500, 2000, 64, 256, 1, 1),
            CoverTrafficMode::Medium => (250, 1000, 128, 512, 2, 1),
            CoverTrafficMode::High => (100, 500, 256, 1024, 4, 1),
        };
        Self {
            version: NDIS_COVER_TRAFFIC_VERSION,
            mode: mode as u32,
            min_interval_ms: min_ms,
            max_interval_ms: max_ms,
            min_payload_bytes: min_payload,
            max_payload_bytes: max_payload,
            burst_count: burst,
            enabled,
            reserved: [0; 3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_module_counts_match_wire_sentinel() {
        assert!(preset_modules(ObfuscationPreset::Disabled).is_empty());
        assert_eq!(preset_modules(ObfuscationPreset::Basic).len(), 1);
        assert_eq!(preset_modules(ObfuscationPreset::Balanced).len(), 2);
        assert_eq!(preset_modules(ObfuscationPreset::Aggressive).len(), 4);
        assert_eq!(preset_modules(ObfuscationPreset::Lwo).len(), 1);
    }

    #[test]
    fn route_assignment_layout_size_is_stable() {
        assert_eq!(size_of::<NdisRouteAssignmentV2>(), 64);
    }
}
