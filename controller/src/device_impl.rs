use super::client_impl::NdisError;
use crate::ioctl::NDIS_USER_DEVICE_PATH;
use crate::security::KernelSecurityPolicy;
use std::ffi::c_void;
use std::ptr;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::IO::DeviceIoControl;

pub struct NdisDevice {
    handle: HANDLE,
    security: KernelSecurityPolicy,
}

impl NdisDevice {
    pub fn open() -> Result<Self, NdisError> {
        let path: Vec<u16> = NDIS_USER_DEVICE_PATH.encode_utf16().chain([0]).collect();
        unsafe {
            let handle = CreateFileW(
                PCWSTR(path.as_ptr()),
                (FILE_GENERIC_READ | FILE_GENERIC_WRITE).0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )?;
            if handle == INVALID_HANDLE_VALUE {
                return Err(NdisError::DeviceOpenFailed);
            }
            Ok(Self {
                handle,
                security: KernelSecurityPolicy::new(),
            })
        }
    }

    pub fn ioctl(
        &self,
        code: u32,
        input: Option<&[u8]>,
        output: &mut [u8],
    ) -> Result<u32, NdisError> {
        self.security.verify_caller()?;
        self.security
            .validate_ioctl_buffer(code, input, output.len())?;

        let mut bytes_returned = 0u32;
        let in_ptr = input
            .map(|b| b.as_ptr() as *const c_void)
            .unwrap_or(ptr::null());
        let in_len = input.map(|b| b.len() as u32).unwrap_or(0);
        unsafe {
            DeviceIoControl(
                self.handle,
                code,
                Some(in_ptr),
                in_len,
                Some(output.as_mut_ptr() as *mut c_void),
                output.len() as u32,
                Some(&mut bytes_returned),
                None,
            )?;
        }
        Ok(bytes_returned)
    }
}

impl Drop for NdisDevice {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}
