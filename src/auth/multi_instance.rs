// Multi-instance via mutex trick

use std::ptr;
use std::ffi::CString;

#[cfg(windows)]
use winapi::um::synchapi::{CreateMutexA, ReleaseMutex, WaitForSingleObject};
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;
#[cfg(windows)]
use winapi::um::winbase::WAIT_OBJECT_0;
#[cfg(windows)]
use winapi::shared::minwindef::TRUE;

pub struct MultiInstanceManager {
    #[cfg(windows)]
    mutex_handle: Option<*mut std::ffi::c_void>,
    enabled: bool,
}

// Required for the raw pointer - we manage it safely
unsafe impl Send for MultiInstanceManager {}
unsafe impl Sync for MultiInstanceManager {}

impl MultiInstanceManager {
    pub fn new() -> Self {
        Self {
            #[cfg(windows)]
            mutex_handle: None,
            enabled: false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[cfg(windows)]
    pub fn enable(&mut self) -> Result<(), String> {
        if self.enabled {
            return Ok(());
        }

        unsafe {
            // Create/open the mutex that Roblox uses to prevent multiple instances
            let mutex_name = CString::new("ROBLOX_singletonMutex").unwrap();
            let handle = CreateMutexA(ptr::null_mut(), TRUE, mutex_name.as_ptr());

            if handle.is_null() {
                return Err("Failed to create mutex".to_string());
            }

            // Try to acquire the mutex
            let wait_result = WaitForSingleObject(handle, 0);
            
            if wait_result == WAIT_OBJECT_0 {
                self.mutex_handle = Some(handle as *mut std::ffi::c_void);
                self.enabled = true;
                Ok(())
            } else {
                CloseHandle(handle);
                Err("Roblox is already running - close it first to enable multi-instance".to_string())
            }
        }
    }

    #[cfg(windows)]
    pub fn disable(&mut self) {
        if let Some(handle) = self.mutex_handle.take() {
            unsafe {
                ReleaseMutex(handle as *mut _);
                CloseHandle(handle as *mut _);
            }
        }
        self.enabled = false;
    }

    pub fn toggle(&mut self) -> Result<bool, String> {
        if self.enabled {
            self.disable();
            Ok(false)
        } else {
            self.enable()?;
            Ok(true)
        }
    }

    // Non-Windows stubs
    #[cfg(not(windows))]
    pub fn enable(&mut self) -> Result<(), String> {
        Err("Multi-instance only supported on Windows".to_string())
    }

    #[cfg(not(windows))]
    pub fn disable(&mut self) {}
}

impl Drop for MultiInstanceManager {
    fn drop(&mut self) {
        self.disable();
    }
}

impl Default for MultiInstanceManager {
    fn default() -> Self {
        Self::new()
    }
}
