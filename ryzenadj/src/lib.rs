//! Rust wrapper for the bundled RyzenAdj C library.
//!
//! # Upstream limitations
//!
//! The C Windows backend may crash during cleanup before PM Table initialization
//! or after some initialization failures. C's STAPM time setter also has switch
//! fallthroughs that can send a second request and return its status.

use crate::{
    error::{Error, Result, StatusCode},
    family::RyzenFamily,
    table::PowerTable,
    types::{
        CoreAddress, CurveOptimizerOffset, DegreesCelsius, Megahertz, Milliamps, Milliwatts, OcVid,
        PerformancePreference, ProchotDeassertionRamp, Seconds,
    },
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

/// Owns a C RyzenAdj handle and releases it through C cleanup on drop.
pub struct RyzenAdj {
    raw: NonNull<sys::_ryzen_access>,
    _instance_guard: InstanceGuard,
}

impl RyzenAdj {
    /// Creates a handle without initializing the PM Table cache.
    ///
    /// Returns [`Error::AlreadyInUse`] if another wrapper handle exists, or
    /// [`Error::InitializationFailed`] if C returns a null handle.
    pub fn new() -> Result<Self> {
        let guard = InstanceGuard::acquire()?;
        let raw_ptr = unsafe { sys::init_ryzenadj() };

        let raw = NonNull::new(raw_ptr).ok_or(Error::InitializationFailed)?;

        Ok(Self {
            raw,
            _instance_guard: guard,
        })
    }

    /// Returns the CPU family stored in the C handle during initialization.
    pub fn cpu_family(&self) -> RyzenFamily {
        let raw = unsafe { sys::get_cpu_family(self.as_raw()) };

        RyzenFamily::from_raw(raw)
    }

    /// Returns the raw SMU BIOS interface version.
    ///
    /// C caches nonzero versions; a zero value causes another query on the next
    /// call. C ignores the SMU request status, so this cannot report whether the
    /// query succeeded.
    pub fn bios_interface_version(&mut self) -> i32 {
        unsafe { sys::get_bios_if_ver(self.as_raw()) }
    }

    /// Refreshes the C PM Table cache and returns an exclusive borrowed view.
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

    /// Sends a raw PROCHOT deassertion ramp value without refreshing the PM table.
    pub fn set_prochot_deassertion_ramp(&mut self, ramp: ProchotDeassertionRamp) -> Result<()> {
        unsafe { sys::set_prochot_deassertion_ramp(self.as_raw(), ramp.0) }.check()
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

    /// Sends a forced GFX clock in MHz without refreshing the PM table.
    pub fn set_gfx_clock(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_gfx_clk(self.as_raw(), frequency.0) }.check()
    }

    /// Sends a forced OC clock for all cores in MHz without refreshing the PM table.
    pub fn set_oc_clock(&mut self, frequency: Megahertz) -> Result<()> {
        unsafe { sys::set_oc_clk(self.as_raw(), frequency.0) }.check()
    }

    /// Sends a forced OC clock for one core without refreshing the PM table.
    ///
    /// The MHz value occupies the low 20 bits, following
    /// [ZenStates-Core's encoding](https://github.com/irusanov/ZenStates-Core/blob/d08e0ac3e0e9ca26f740d39468405205d161d1ae/Hardware/Smu/Commands/SetFrequencySingleCore.cs#L6-L17).
    /// Returns [`Error::PerCoreOcClockOutOfRange`] before sending a request if the
    /// value exceeds `0xF_FFFF`.
    pub fn set_per_core_oc_clock(&mut self, core: CoreAddress, frequency: Megahertz) -> Result<()> {
        let value = core.encode_oc_clock(frequency)?;
        unsafe { sys::set_per_core_oc_clk(self.as_raw(), value) }.check()
    }

    /// Sends a core OC VID code without refreshing the PM table.
    pub fn set_oc_vid(&mut self, vid: OcVid) -> Result<()> {
        unsafe { sys::set_oc_volt(self.as_raw(), vid.0) }.check()
    }

    /// Sends a performance preference without refreshing the PM table.
    pub fn set_performance_preference(&mut self, preference: PerformancePreference) -> Result<()> {
        let status = unsafe {
            match preference {
                PerformancePreference::PowerSaving => sys::set_power_saving(self.as_raw()),
                PerformancePreference::MaxPerformance => sys::set_max_performance(self.as_raw()),
            }
        };
        status.check()
    }

    /// Requests enabling or disabling overclocking without refreshing the PM table.
    pub fn set_overclocking_enabled(&mut self, enabled: bool) -> Result<()> {
        let status = unsafe {
            if enabled {
                sys::set_enable_oc(self.as_raw())
            } else {
                sys::set_disable_oc(self.as_raw())
            }
        };
        status.check()
    }

    /// Sends a Curve Optimizer offset for all CPU cores without refreshing the PM table.
    pub fn set_all_core_curve_optimizer(&mut self, offset: CurveOptimizerOffset) -> Result<()> {
        unsafe { sys::set_coall(self.as_raw(), offset.0 as u32) }.check()
    }

    /// Sends a Curve Optimizer offset for one core without refreshing the PM table.
    ///
    /// Encodes the offset as a signed 16-bit value, leaving bits 19–16 zero,
    /// following [ZenStates-Core's encoding](https://github.com/irusanov/ZenStates-Core/blob/d08e0ac3e0e9ca26f740d39468405205d161d1ae/Hardware/Smu/Commands/SetPsmMarginSingleCore.cs#L3-L25).
    /// Returns [`Error::PerCoreCurveOptimizerOffsetOutOfRange`] before sending a
    /// request if the offset is outside `i16::MIN..=i16::MAX`.
    pub fn set_per_core_curve_optimizer(
        &mut self,
        core: CoreAddress,
        offset: CurveOptimizerOffset,
    ) -> Result<()> {
        let value = core.encode_curve_optimizer(offset)?;
        unsafe { sys::set_coper(self.as_raw(), value) }.check()
    }

    /// Sends a GFX Curve Optimizer offset without refreshing the PM table.
    pub fn set_gfx_curve_optimizer(&mut self, offset: CurveOptimizerOffset) -> Result<()> {
        unsafe { sys::set_cogfx(self.as_raw(), offset.0 as u32) }.check()
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
