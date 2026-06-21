use super::device_impl::NdisDevice;
use crate::ioctl::*;
use crate::types::*;
use std::mem::size_of;

#[derive(Debug, thiserror::Error)]
pub enum NdisError {
    #[error("failed to open NDIS device")]
    DeviceOpenFailed,
    #[error("windows API error: {0}")]
    Windows(#[from] windows::core::Error),
    #[error("ioctl returned insufficient bytes: {0}")]
    ShortRead(u32),
    #[error("invalid driver state version")]
    InvalidVersion,
    #[error("security policy violation: {0}")]
    Security(#[from] crate::security::SecurityError),
}

pub struct NdisClient {
    device: NdisDevice,
}

impl NdisClient {
    pub fn connect() -> Result<Self, NdisError> {
        Ok(Self {
            device: NdisDevice::open()?,
        })
    }

    pub fn driver_state(&self) -> Result<NdisDriverStateV2, NdisError> {
        let mut buf = [0u8; NdisDriverStateV2::SIZE];
        let n = self
            .device
            .ioctl(IOCTL_NDIS_GET_DRIVER_STATE, None, &mut buf)?;
        if n < NdisDriverStateV2::SIZE as u32 {
            return Err(NdisError::ShortRead(n));
        }
        let state = unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const NdisDriverStateV2) };
        if state.version != NDIS_DRIVER_STATE_VERSION {
            return Err(NdisError::InvalidVersion);
        }
        Ok(state)
    }

    pub fn set_route(&self, route: &NdisRouteAssignmentV2) -> Result<(), NdisError> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                (route as *const NdisRouteAssignmentV2) as *const u8,
                size_of::<NdisRouteAssignmentV2>(),
            )
        };
        self.device
            .ioctl(IOCTL_NDIS_SET_ROUTE, Some(bytes), &mut [])?;
        Ok(())
    }

    pub fn clear_route(&self, flow_id: uuid::Uuid) -> Result<(), NdisError> {
        let id = uuid_to_bytes(flow_id);
        self.device
            .ioctl(IOCTL_NDIS_CLEAR_ROUTE, Some(&id), &mut [])?;
        Ok(())
    }

    pub fn set_transform_profile(&self, profile: &NdisTransformProfileV2) -> Result<(), NdisError> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                (profile as *const NdisTransformProfileV2) as *const u8,
                size_of::<NdisTransformProfileV2>(),
            )
        };
        self.device
            .ioctl(IOCTL_NDIS_SET_TRANSFORM_PROFILE, Some(bytes), &mut [])?;
        Ok(())
    }

    pub fn set_cover_traffic(&self, profile: &NdisCoverTrafficProfileV2) -> Result<(), NdisError> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                (profile as *const NdisCoverTrafficProfileV2) as *const u8,
                size_of::<NdisCoverTrafficProfileV2>(),
            )
        };
        self.device
            .ioctl(IOCTL_NDIS_SET_COVER_TRAFFIC, Some(bytes), &mut [])?;
        Ok(())
    }

    pub fn telemetry_summary(&self) -> Result<NdisTelemetrySummaryV2, NdisError> {
        let mut buf = [0u8; size_of::<NdisTelemetrySummaryV2>()];
        let n = self
            .device
            .ioctl(IOCTL_NDIS_GET_TELEMETRY_SUMMARY, None, &mut buf)?;
        if n < size_of::<NdisTelemetrySummaryV2>() as u32 {
            return Err(NdisError::ShortRead(n));
        }
        Ok(unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const NdisTelemetrySummaryV2) })
    }
}
