//! Client-side IOCTL security policy — mirrors `NdisValidateIoctlBuffer` in the LWF driver.

use crate::ioctl::*;
use crate::types::*;
use std::mem::size_of;

pub const NDIS_MAX_IOCTL_BUFFER: usize = 65536;
pub const NDIS_MAX_SYNC_ROUTES: u32 = 512;
pub const NDIS_MAX_REDIRECT_RULES: u32 = 256;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SecurityError {
    #[error("IOCTL payload exceeds maximum buffer size ({NDIS_MAX_IOCTL_BUFFER} bytes)")]
    PayloadTooLarge,
    #[error("invalid device request: {0:#010x}")]
    InvalidDeviceRequest(u32),
    #[error("invalid input buffer size")]
    InvalidInputSize,
    #[error("invalid output buffer size")]
    InvalidOutputSize,
    #[error("invalid structure version: expected {expected}, got {actual}")]
    InvalidVersion { expected: u32, actual: u32 },
    #[error("invalid sync count: {0}")]
    InvalidSyncCount(u32),
    #[error("caller verification failed")]
    CallerVerificationFailed,
}

/// Validates NDIS IOCTL buffers before they reach the kernel driver.
pub struct KernelSecurityPolicy {
    service_pid: Option<u32>,
}

impl Default for KernelSecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl KernelSecurityPolicy {
    pub fn new() -> Self {
        Self {
            service_pid: std::process::id().into(),
        }
    }

    pub fn with_service_pid(pid: u32) -> Self {
        Self {
            service_pid: Some(pid),
        }
    }

    /// Stub caller verification — on Windows ensures IOCTLs originate from the service PID.
    pub fn verify_caller(&self) -> Result<(), SecurityError> {
        #[cfg(windows)]
        {
            if let Some(expected) = self.service_pid {
                if std::process::id() != expected {
                    return Err(SecurityError::CallerVerificationFailed);
                }
            }
        }
        Ok(())
    }

    /// Returns the minimum required output length for read IOCTLs.
    pub fn validate_ioctl_buffer(
        &self,
        io_control_code: u32,
        input: Option<&[u8]>,
        output_len: usize,
    ) -> Result<usize, SecurityError> {
        let input_len = input.map(|b| b.len()).unwrap_or(0);
        if input_len > NDIS_MAX_IOCTL_BUFFER || output_len > NDIS_MAX_IOCTL_BUFFER {
            return Err(SecurityError::PayloadTooLarge);
        }

        let required_output = match io_control_code {
            IOCTL_NDIS_GET_DRIVER_STATE => {
                require_output(output_len, size_of::<NdisDriverStateV2>())?;
                size_of::<NdisDriverStateV2>()
            }
            IOCTL_NDIS_SET_ROUTE => {
                let input = require_input(input, size_of::<NdisRouteAssignmentV2>())?;
                check_u32_version(input, NDIS_ROUTE_ASSIGNMENT_VERSION)?;
                0
            }
            IOCTL_NDIS_CLEAR_ROUTE => {
                require_input(input, 16)?;
                0
            }
            IOCTL_NDIS_SYNC_ROUTES => {
                let input = require_input(input, 8)?;
                let route_count = read_u32(input, 4)?;
                if route_count > NDIS_MAX_SYNC_ROUTES {
                    return Err(SecurityError::InvalidSyncCount(route_count));
                }
                let needed = 8 + route_count as usize * size_of::<NdisRouteAssignmentV2>();
                if input.len() < needed {
                    return Err(SecurityError::InvalidInputSize);
                }
                for idx in 0..route_count as usize {
                    let offset = 8 + idx * size_of::<NdisRouteAssignmentV2>();
                    check_u32_version(&input[offset..], NDIS_ROUTE_ASSIGNMENT_VERSION)?;
                }
                0
            }
            IOCTL_NDIS_SET_TRANSFORM_PROFILE => {
                let input = require_input(input, size_of::<NdisTransformProfileV2>())?;
                check_u32_version(input, NDIS_TRANSFORM_PROFILE_VERSION)?;
                0
            }
            IOCTL_NDIS_SET_COVER_TRAFFIC => {
                let input = require_input(input, size_of::<NdisCoverTrafficProfileV2>())?;
                check_u32_version(input, NDIS_COVER_TRAFFIC_VERSION)?;
                0
            }
            IOCTL_NDIS_SYNC_REDIRECT => {
                let input = require_input(input, 8)?;
                let rule_count = read_u32(input, 4)?;
                if rule_count > NDIS_MAX_REDIRECT_RULES {
                    return Err(SecurityError::InvalidSyncCount(rule_count));
                }
                let needed = 8 + rule_count as usize * size_of::<NdisRedirectRuleV2>();
                if input.len() < needed {
                    return Err(SecurityError::InvalidInputSize);
                }
                for idx in 0..rule_count as usize {
                    let offset = 8 + idx * size_of::<NdisRedirectRuleV2>();
                    check_u32_version(&input[offset..], NDIS_REDIRECT_RULE_VERSION)?;
                }
                0
            }
            IOCTL_NDIS_GET_TELEMETRY_SUMMARY => {
                require_output(output_len, size_of::<NdisTelemetrySummaryV2>())?;
                size_of::<NdisTelemetrySummaryV2>()
            }
            IOCTL_NDIS_DRAIN_TELEMETRY => {
                require_output(output_len, 8)?;
                8
            }
            IOCTL_NDIS_CLASSIFY_PACKET => {
                require_input(input, 8)?;
                0
            }
            _ => return Err(SecurityError::InvalidDeviceRequest(io_control_code)),
        };

        Ok(required_output)
    }
}

fn require_input<'a>(
    input: Option<&'a [u8]>,
    min_len: usize,
) -> Result<&'a [u8], SecurityError> {
    let input = input.ok_or(SecurityError::InvalidInputSize)?;
    if input.len() < min_len {
        return Err(SecurityError::InvalidInputSize);
    }
    Ok(input)
}

fn require_output(output_len: usize, min_len: usize) -> Result<(), SecurityError> {
    if output_len < min_len {
        return Err(SecurityError::InvalidOutputSize);
    }
    Ok(())
}

fn read_u32(input: &[u8], offset: usize) -> Result<u32, SecurityError> {
    if input.len() < offset + 4 {
        return Err(SecurityError::InvalidInputSize);
    }
    Ok(u32::from_le_bytes(
        input[offset..offset + 4]
            .try_into()
            .map_err(|_| SecurityError::InvalidInputSize)?,
    ))
}

fn check_u32_version(input: &[u8], expected: u32) -> Result<(), SecurityError> {
    let actual = read_u32(input, 0)?;
    if actual != expected {
        return Err(SecurityError::InvalidVersion { expected, actual });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_ioctl() {
        let policy = KernelSecurityPolicy::new();
        let err = policy
            .validate_ioctl_buffer(0xDEAD_BEEF, None, 0)
            .unwrap_err();
        assert!(matches!(err, SecurityError::InvalidDeviceRequest(_)));
    }

    #[test]
    fn rejects_oversized_payload() {
        let policy = KernelSecurityPolicy::new();
        let huge = vec![0u8; NDIS_MAX_IOCTL_BUFFER + 1];
        let err = policy
            .validate_ioctl_buffer(IOCTL_NDIS_SET_ROUTE, Some(&huge), 0)
            .unwrap_err();
        assert_eq!(err, SecurityError::PayloadTooLarge);
    }

    #[test]
    fn validates_set_route_version() {
        let policy = KernelSecurityPolicy::new();
        let mut route = NdisRouteAssignmentV2::new_vpn(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            1,
            42,
        );
        route.version = 99;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                (&route as *const NdisRouteAssignmentV2) as *const u8,
                size_of::<NdisRouteAssignmentV2>(),
            )
        };
        let err = policy
            .validate_ioctl_buffer(IOCTL_NDIS_SET_ROUTE, Some(bytes), 0)
            .unwrap_err();
        assert!(matches!(
            err,
            SecurityError::InvalidVersion {
                expected: NDIS_ROUTE_ASSIGNMENT_VERSION,
                actual: 99
            }
        ));
    }

    #[test]
    fn validates_driver_state_output_size() {
        let policy = KernelSecurityPolicy::new();
        let err = policy
            .validate_ioctl_buffer(IOCTL_NDIS_GET_DRIVER_STATE, None, 4)
            .unwrap_err();
        assert_eq!(err, SecurityError::InvalidOutputSize);
    }

    #[test]
    fn caller_verification_accepts_service_pid() {
        let pid = std::process::id();
        let policy = KernelSecurityPolicy::with_service_pid(pid);
        assert!(policy.verify_caller().is_ok());
    }

    #[test]
    #[cfg(windows)]
    fn caller_verification_rejects_foreign_pid() {
        let policy = KernelSecurityPolicy::with_service_pid(1);
        let err = policy.verify_caller().unwrap_err();
        assert_eq!(err, SecurityError::CallerVerificationFailed);
    }
}
