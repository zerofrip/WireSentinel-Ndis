pub mod ioctl;
pub mod security;
pub mod types;

#[cfg(windows)]
mod client_impl;
#[cfg(windows)]
mod device_impl;

#[cfg(not(windows))]
mod stub;

#[cfg(windows)]
pub use client_impl::{NdisClient, NdisError};
#[cfg(windows)]
pub use device_impl::NdisDevice;

#[cfg(not(windows))]
pub use stub::{NdisClient, NdisDevice, NdisError};

pub use security::{KernelSecurityPolicy, SecurityError};
pub use types::*;
