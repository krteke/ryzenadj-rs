use crate::{
    error::{Error, Result, check_code},
    family::RyzenFamily,
    table::PowerTable,
};
use ryzenadj_sys as sys;
use std::ptr::NonNull;

pub mod error;
pub mod family;
pub mod table;

pub struct RyzenAdj {
    raw: NonNull<sys::_ryzen_access>,
}

impl RyzenAdj {
    pub fn new() -> Result<Self> {
        let raw_ptr = unsafe { sys::init_ryzenadj() };

        let raw = NonNull::new(raw_ptr).ok_or(Error::InitializationFailed)?;

        Ok(Self { raw })
    }

    pub fn cpu_family(&self) -> RyzenFamily {
        let raw = unsafe { sys::get_cpu_family(self.as_raw()) };

        RyzenFamily::from_raw(raw)
    }

    pub fn power_table(&mut self) -> Result<PowerTable<'_>> {
        check_code(unsafe { sys::refresh_table(self.as_raw()) })?;

        Ok(PowerTable::from(self))
    }

    fn as_raw(&self) -> sys::ryzen_access {
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
