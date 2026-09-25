use core::slice;

use crate::{
    RyzenAdj,
    error::{Result, check_code},
};
use ryzenadj_sys as sys;

pub struct PowerTable<'a> {
    pub(super) owner: &'a mut RyzenAdj,
}

/// A power limit and its corresponding PM Table reading, in `watts`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerLimitReading {
    pub limit: f32,
    pub measured: f32,
}

impl PowerTable<'_> {
    pub fn refresh(&mut self) -> Result<()> {
        check_code(unsafe { sys::refresh_table(self.owner.as_raw()) })
    }

    pub fn version(&self) -> u32 {
        unsafe { sys::get_table_ver(self.owner.as_raw()) }
    }

    pub fn values(&self) -> &[f32] {
        let ptr = unsafe { sys::get_table_values(self.owner.as_raw()) };

        let bytes = unsafe { sys::get_table_size(self.owner.as_raw()) };

        unsafe { slice::from_raw_parts(ptr, bytes / size_of::<f32>()) }
    }

    /// Returns the STAPM limit and its reported power.
    pub fn stapm(&self) -> PowerLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            PowerLimitReading {
                limit: sys::get_stapm_limit(raw),
                measured: sys::get_stapm_value(raw),
            }
        }
    }

    /// Returns the fast PPT limit and its reported power.
    pub fn fast_ppt(&self) -> PowerLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            PowerLimitReading {
                limit: sys::get_fast_limit(raw),
                measured: sys::get_fast_value(raw),
            }
        }
    }

    /// Returns the slow PPT limit and its reported power.
    pub fn slow_ppt(&self) -> PowerLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            PowerLimitReading {
                limit: sys::get_slow_limit(raw),
                measured: sys::get_slow_value(raw),
            }
        }
    }
}
