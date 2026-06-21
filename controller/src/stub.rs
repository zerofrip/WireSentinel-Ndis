use crate::types::*;

#[derive(Debug, thiserror::Error)]
pub enum NdisError {
    #[error("NDIS driver requires Windows")]
    NotWindows,
}

pub struct NdisDevice;

impl NdisDevice {
    pub fn open() -> Result<Self, NdisError> {
        Err(NdisError::NotWindows)
    }
}

pub struct NdisClient;

impl NdisClient {
    pub fn connect() -> Result<Self, NdisError> {
        Err(NdisError::NotWindows)
    }

    pub fn driver_state(&self) -> Result<NdisDriverStateV2, NdisError> {
        Err(NdisError::NotWindows)
    }

    pub fn set_route(&self, _route: &NdisRouteAssignmentV2) -> Result<(), NdisError> {
        Err(NdisError::NotWindows)
    }

    pub fn clear_route(&self, _flow_id: uuid::Uuid) -> Result<(), NdisError> {
        Err(NdisError::NotWindows)
    }

    pub fn set_transform_profile(
        &self,
        _profile: &NdisTransformProfileV2,
    ) -> Result<(), NdisError> {
        Err(NdisError::NotWindows)
    }

    pub fn telemetry_summary(&self) -> Result<NdisTelemetrySummaryV2, NdisError> {
        Err(NdisError::NotWindows)
    }
}
