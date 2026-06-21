use ndis_controller::{uuid_to_bytes, NdisRouteAssignmentV2, NdisRouteKind};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelRouteAssignment {
    pub flow_id: Uuid,
    pub app_id: Uuid,
    pub route_kind: NdisRouteKind,
    pub profile_id: u64,
    pub interface_luid: u64,
    pub socks_port: u16,
    pub protocol: u16,
}

impl KernelRouteAssignment {
    pub fn vpn(flow_id: Uuid, app_id: Uuid, profile_id: u64, interface_luid: u64) -> Self {
        Self {
            flow_id,
            app_id,
            route_kind: NdisRouteKind::Vpn,
            profile_id,
            interface_luid,
            socks_port: 0,
            protocol: 0,
        }
    }

    pub fn proxy(flow_id: Uuid, app_id: Uuid, profile_id: u64, socks_port: u16) -> Self {
        Self {
            flow_id,
            app_id,
            route_kind: NdisRouteKind::Proxy,
            profile_id,
            interface_luid: 0,
            socks_port,
            protocol: 0,
        }
    }

    pub fn to_ioctl_struct(&self) -> NdisRouteAssignmentV2 {
        NdisRouteAssignmentV2 {
            version: ndis_controller::NDIS_ROUTE_ASSIGNMENT_VERSION,
            flow_id: uuid_to_bytes(self.flow_id),
            app_id: uuid_to_bytes(self.app_id),
            route_kind: self.route_kind as u32,
            profile_id: self.profile_id,
            interface_luid: self.interface_luid,
            socks_port: self.socks_port,
            protocol: self.protocol,
            reserved: [0; 4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_ioctl_layout() {
        let flow = Uuid::new_v4();
        let app = Uuid::new_v4();
        let assignment = KernelRouteAssignment::vpn(flow, app, 7, 0x1234);
        let ioctl = assignment.to_ioctl_struct();
        let profile_id = unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(ioctl.profile_id)) };
        let route_kind = unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(ioctl.route_kind)) };
        assert_eq!(profile_id, 7);
        assert_eq!(route_kind, NdisRouteKind::Vpn as u32);
        assert_eq!(ioctl.flow_id, uuid_to_bytes(flow));
    }
}
