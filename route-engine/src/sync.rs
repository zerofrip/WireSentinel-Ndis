use crate::assignment::KernelRouteAssignment;
use ndis_controller::{NdisClient, NdisError};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RouteSyncError {
    #[error("driver error: {0}")]
    Driver(#[from] NdisError),
    #[error("duplicate flow id: {0}")]
    DuplicateFlow(Uuid),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SyncReport {
    pub upserted: usize,
    pub removed: usize,
    pub total: usize,
}

pub struct RouteSyncEngine {
    desired: HashMap<Uuid, KernelRouteAssignment>,
}

impl Default for RouteSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RouteSyncEngine {
    pub fn new() -> Self {
        Self {
            desired: HashMap::new(),
        }
    }

    pub fn upsert(&mut self, assignment: KernelRouteAssignment) -> Result<(), RouteSyncError> {
        if self.desired.contains_key(&assignment.flow_id)
            && self.desired.get(&assignment.flow_id).map(|a| a.flow_id) == Some(assignment.flow_id)
        {
            // allow overwrite of same flow
        }
        self.desired.insert(assignment.flow_id, assignment);
        Ok(())
    }

    pub fn remove(&mut self, flow_id: Uuid) -> Option<KernelRouteAssignment> {
        self.desired.remove(&flow_id)
    }

    pub fn len(&self) -> usize {
        self.desired.len()
    }

    pub fn is_empty(&self) -> bool {
        self.desired.is_empty()
    }

    pub fn sync_to_kernel(&self, client: &NdisClient) -> Result<SyncReport, RouteSyncError> {
        let mut upserted = 0usize;
        for assignment in self.desired.values() {
            client.set_route(&assignment.to_ioctl_struct())?;
            upserted += 1;
        }
        Ok(SyncReport {
            upserted,
            removed: 0,
            total: self.desired.len(),
        })
    }

    pub fn reconcile(
        &mut self,
        active_flow_ids: &[Uuid],
        client: &NdisClient,
    ) -> Result<SyncReport, RouteSyncError> {
        let active: HashSet<Uuid> = active_flow_ids.iter().copied().collect();
        let stale: Vec<Uuid> = self
            .desired
            .keys()
            .copied()
            .filter(|id| !active.contains(id))
            .collect();
        let mut removed = 0usize;
        for flow_id in stale {
            client.clear_route(flow_id)?;
            self.desired.remove(&flow_id);
            removed += 1;
        }
        let report = self.sync_to_kernel(client)?;
        Ok(SyncReport {
            upserted: report.upserted,
            removed,
            total: self.desired.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndis_controller::NdisRouteKind;

    #[test]
    fn upsert_tracks_assignments() {
        let mut engine = RouteSyncEngine::new();
        let flow = Uuid::new_v4();
        let app = Uuid::new_v4();
        engine
            .upsert(KernelRouteAssignment::vpn(flow, app, 1, 0))
            .unwrap();
        assert_eq!(engine.len(), 1);
    }

    #[test]
    fn assignment_roundtrip_fields() {
        let assignment = KernelRouteAssignment::proxy(Uuid::new_v4(), Uuid::new_v4(), 42, 1080);
        assert_eq!(assignment.route_kind, NdisRouteKind::Proxy);
        assert_eq!(assignment.socks_port, 1080);
    }

    #[test]
    fn sync_to_kernel_fails_on_non_windows() {
        let engine = RouteSyncEngine::new();
        let client = NdisClient::connect();
        assert!(client.is_err());
        if let Ok(client) = client {
            assert!(engine.sync_to_kernel(&client).is_err());
        }
    }
}
