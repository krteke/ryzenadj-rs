use std::ptr::NonNull;

use ryzenadj_sys as sys;

use crate::error::{Error, Result};

pub mod error;
mod family;

pub use family::RyzenFamily;

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

    pub fn cpu_family(&self) -> Result<RyzenFamily> {
        let raw = unsafe { sys::get_cpu_family(self.as_raw()) };
        RyzenFamily::try_from(raw)
    }
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        unsafe {
            sys::cleanup_ryzenadj(self.as_raw());
        }
    }
}
