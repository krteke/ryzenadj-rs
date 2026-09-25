use crate::{
    error::{Error, Result, StatusCode},
    family::RyzenFamily,
    table::PowerTable,
};
use ryzenadj_sys as sys;
use std::{
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod error;
pub mod family;
pub mod table;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milliwatts(pub u32);

static INSTANCE_IN_USE: AtomicBool = AtomicBool::new(false);

struct InstanceGuard;

impl InstanceGuard {
    fn acquire() -> Result<Self> {
        if INSTANCE_IN_USE
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return Err(Error::AlreadyInUse);
        }

        Ok(Self)
    }
}

impl Drop for InstanceGuard {
    fn drop(&mut self) {
        INSTANCE_IN_USE.store(false, Ordering::Release);
    }
}

pub struct RyzenAdj {
    raw: NonNull<sys::_ryzen_access>,
    _instance_guard: InstanceGuard,
}

impl RyzenAdj {
    pub fn new() -> Result<Self> {
        let guard = InstanceGuard::acquire()?;
        let raw_ptr = unsafe { sys::init_ryzenadj() };

        let raw = NonNull::new(raw_ptr).ok_or(Error::InitializationFailed)?;

        Ok(Self {
            raw,
            _instance_guard: guard,
        })
    }

    pub fn cpu_family(&self) -> RyzenFamily {
        let raw = unsafe { sys::get_cpu_family(self.as_raw()) };

        RyzenFamily::from_raw(raw)
    }

    pub fn power_table(&mut self) -> Result<PowerTable<'_>> {
        unsafe { sys::refresh_table(self.as_raw()) }.check()?;

        Ok(PowerTable { owner: self })
    }

    /// Sends a STAPM limit without refreshing the PM table.
    pub fn set_stapm_limit(&mut self, limit: Milliwatts) -> Result<()> {
        unsafe { sys::set_stapm_limit(self.as_raw(), limit.0) }.check()
    }

    /// Sends a fast PPT limit without refreshing the PM table.
    pub fn set_fast_ppt_limit(&mut self, limit: Milliwatts) -> Result<()> {
        unsafe { sys::set_fast_limit(self.as_raw(), limit.0) }.check()
    }

    /// Sends a slow PPT limit without refreshing the PM table.
    pub fn set_slow_ppt_limit(&mut self, limit: Milliwatts) -> Result<()> {
        unsafe { sys::set_slow_limit(self.as_raw(), limit.0) }.check()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_guard_releases_ownership() {
        let first = InstanceGuard::acquire().unwrap();
        assert!(matches!(InstanceGuard::acquire(), Err(Error::AlreadyInUse)));

        drop(first);
        let _second = InstanceGuard::acquire().unwrap();
    }
}
