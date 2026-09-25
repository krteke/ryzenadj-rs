use crate::{
    error::{Error, Result, StatusCode},
    family::RyzenFamily,
    table::PowerTable,
    types::{DegreesCelsius, Megahertz, Milliamps, Milliwatts, Seconds},
};
use ryzenadj_sys as sys;
use std::{
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod error;
pub mod family;
pub mod table;
pub mod types;

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

    /// Sends an APU slow PPT limit without refreshing the PM table.
    pub fn set_apu_slow_ppt_limit(&mut self, limit: Milliwatts) -> Result<()> {
        unsafe { sys::set_apu_slow_limit(self.as_raw(), limit.0) }.check()
    }

    /// Sends a STAPM time constant without refreshing the PM table.
    pub fn set_stapm_time(&mut self, time: Seconds) -> Result<()> {
        unsafe { sys::set_stapm_time(self.as_raw(), time.0) }.check()
    }

    /// Sends a slow PPT time constant without refreshing the PM table.
    pub fn set_slow_ppt_time(&mut self, time: Seconds) -> Result<()> {
        unsafe { sys::set_slow_time(self.as_raw(), time.0) }.check()
    }

    /// Sends a Tctl temperature limit without refreshing the PM table.
    pub fn set_tctl_temperature_limit(&mut self, limit: DegreesCelsius) -> Result<()> {
        unsafe { sys::set_tctl_temp(self.as_raw(), limit.0) }.check()
    }

    /// Sends an APU STT limit without refreshing the PM table.
    pub fn set_apu_skin_temperature_limit(&mut self, limit: DegreesCelsius) -> Result<()> {
        unsafe { sys::set_apu_skin_temp_limit(self.as_raw(), limit.0) }.check()
    }

    /// Sends a dGPU STT limit without refreshing the PM table.
    pub fn set_dgpu_skin_temperature_limit(&mut self, limit: DegreesCelsius) -> Result<()> {
        unsafe { sys::set_dgpu_skin_temp_limit(self.as_raw(), limit.0) }.check()
    }

    /// Sends a skin temperature power limit without refreshing the PM table.
    pub fn set_skin_temperature_power_limit(&mut self, limit: Milliwatts) -> Result<()> {
        unsafe { sys::set_skin_temp_power_limit(self.as_raw(), limit.0) }.check()
    }

    /// Sends the VDD TDC current limit without refreshing the PM table.
    pub fn set_tdc_vdd_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_vrm_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the SoC TDC current limit without refreshing the PM table.
    pub fn set_tdc_soc_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_vrmsoc_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the GFX TDC current limit without refreshing the PM table.
    pub fn set_tdc_gfx_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_vrmgfx_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the CVIP TDC current limit without refreshing the PM table.
    pub fn set_tdc_cvip_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_vrmcvip_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the VDD EDC current limit without refreshing the PM table.
    pub fn set_edc_vdd_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_vrmmax_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the SoC EDC current limit without refreshing the PM table.
    pub fn set_edc_soc_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_vrmsocmax_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the GFX EDC current limit without refreshing the PM table.
    pub fn set_edc_gfx_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_vrmgfxmax_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the PSI0 VDD current limit without refreshing the PM table.
    pub fn set_psi0_vdd_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_psi0_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the PSI0 SoC current limit without refreshing the PM table.
    pub fn set_psi0_soc_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_psi0soc_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the PSI3 CPU current limit without refreshing the PM table.
    pub fn set_psi3_cpu_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_psi3cpu_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the PSI3 GFX current limit without refreshing the PM table.
    pub fn set_psi3_gfx_limit(&mut self, limit: Milliamps) -> Result<()> {
        unsafe { sys::set_psi3gfx_current(self.as_raw(), limit.0) }.check()
    }

    /// Sends the minimum GFX clock without refreshing the PM table.
    pub fn set_gfx_clock_min(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_min_gfxclk_freq(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the maximum GFX clock without refreshing the PM table.
    pub fn set_gfx_clock_max(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_max_gfxclk_freq(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the minimum SoC clock without refreshing the PM table.
    pub fn set_soc_clock_min(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_min_socclk_freq(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the maximum SoC clock without refreshing the PM table.
    pub fn set_soc_clock_max(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_max_socclk_freq(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the minimum FCLK without refreshing the PM table.
    pub fn set_fclk_min(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_min_fclk_freq(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the maximum FCLK without refreshing the PM table.
    pub fn set_fclk_max(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_max_fclk_freq(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the minimum VCN clock without refreshing the PM table.
    pub fn set_vcn_clock_min(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_min_vcn(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the maximum VCN clock without refreshing the PM table.
    pub fn set_vcn_clock_max(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_max_vcn(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the minimum LCLK without refreshing the PM table.
    pub fn set_lclk_min(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_min_lclk(self.as_raw(), frequency.0) }.check()
    }

    /// Sends the maximum LCLK without refreshing the PM table.
    pub fn set_lclk_max(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_max_lclk(self.as_raw(), frequency.0) }.check()
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

trait NanExt: Sized {
    fn none_if_nan(self) -> Option<Self>;
}

impl NanExt for f32 {
    #[inline]
    fn none_if_nan(self) -> Option<Self> {
        (!self.is_nan()).then_some(self)
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
