use std::ptr::NonNull;

use ryzenadj_sys as sys;

use crate::error::{Error, Result};

pub mod error;

pub struct RyzenAdj {
    raw: NonNull<sys::_ryzen_access>,
}

impl RyzenAdj {
    pub fn new() -> Result<Self> {
        let raw_ptr = unsafe { sys::init_ryzenadj() };

        let raw = NonNull::new(raw_ptr).ok_or(Error::InitializationFailed)?;

        Ok(Self { raw })
    }

    pub fn as_raw(&self) -> sys::ryzen_access {
        self.raw.as_ptr()
    }
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        unsafe {
            sys::cleanup_ryzenadj(self.as_raw());
        }
    }
}
