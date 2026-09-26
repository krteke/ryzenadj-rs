use core::slice;

use crate::{
    NanExt, RyzenAdj,
    error::{Result, StatusCode},
    table::reading::{
        CclkReading, CurrentLimitReading, FabricMemoryReading, GfxReading, L3Reading,
        PowerLimitReading, SocReading, TemperatureLimitReading,
    },
};
use ryzenadj_sys as sys;

pub mod reading;

pub struct PowerTable<'a> {
    pub(super) owner: &'a mut RyzenAdj,
}

impl PowerTable<'_> {
    pub fn refresh(&mut self) -> Result<()> {
        unsafe { sys::refresh_table(self.owner.as_raw()) }.check()
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

        unsafe { PowerLimitReading::from_raw(sys::get_stapm_limit(raw), sys::get_stapm_value(raw)) }
    }

    /// Returns the fast PPT limit and its reported power.
    pub fn fast_ppt(&self) -> PowerLimitReading {
        let raw = self.owner.as_raw();

        unsafe { PowerLimitReading::from_raw(sys::get_fast_limit(raw), sys::get_fast_value(raw)) }
    }

    /// Returns the slow PPT limit and its reported power.
    pub fn slow_ppt(&self) -> PowerLimitReading {
        let raw = self.owner.as_raw();

        unsafe { PowerLimitReading::from_raw(sys::get_slow_limit(raw), sys::get_slow_value(raw)) }
    }

    /// Returns the APU slow PPT limit and its reported power in watts.
    pub fn apu_slow_ppt(&self) -> PowerLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            PowerLimitReading::from_raw(sys::get_apu_slow_limit(raw), sys::get_apu_slow_value(raw))
        }
    }

    /// Returns the STAPM time constant in seconds, when available.
    pub fn stapm_time(&self) -> Option<f32> {
        let seconds = unsafe { sys::get_stapm_time(self.owner.as_raw()) };
        seconds.none_if_nan()
    }

    /// Returns the slow PPT time constant in seconds, when available.
    pub fn slow_ppt_time(&self) -> Option<f32> {
        let seconds = unsafe { sys::get_slow_time(self.owner.as_raw()) };
        seconds.none_if_nan()
    }

    /// Returns the Tctl limit and measured temperature.
    pub fn tctl_temperature(&self) -> TemperatureLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            TemperatureLimitReading::from_raw(
                sys::get_tctl_temp(raw),
                sys::get_tctl_temp_value(raw),
            )
        }
    }

    /// Returns the APU STT limit and temperature.
    pub fn apu_skin_temperature(&self) -> TemperatureLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            TemperatureLimitReading::from_raw(
                sys::get_apu_skin_temp_limit(raw),
                sys::get_apu_skin_temp_value(raw),
            )
        }
    }

    /// Returns the dGPU STT limit and temperature.
    pub fn dgpu_skin_temperature(&self) -> TemperatureLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            TemperatureLimitReading::from_raw(
                sys::get_dgpu_skin_temp_limit(raw),
                sys::get_dgpu_skin_temp_value(raw),
            )
        }
    }

    /// Returns the VDD TDC current limit and measured current in amperes.
    pub fn tdc_vdd(&self) -> CurrentLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            CurrentLimitReading::from_raw(
                sys::get_vrm_current(raw),
                sys::get_vrm_current_value(raw),
            )
        }
    }

    /// Returns the SoC TDC current limit and measured current in amperes.
    pub fn tdc_soc(&self) -> CurrentLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            CurrentLimitReading::from_raw(
                sys::get_vrmsoc_current(raw),
                sys::get_vrmsoc_current_value(raw),
            )
        }
    }

    /// Returns the VDD EDC current limit and measured current in amperes.
    pub fn edc_vdd(&self) -> CurrentLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            CurrentLimitReading::from_raw(
                sys::get_vrmmax_current(raw),
                sys::get_vrmmax_current_value(raw),
            )
        }
    }

    /// Returns the SoC EDC current limit and measured current in amperes.
    pub fn edc_soc(&self) -> CurrentLimitReading {
        let raw = self.owner.as_raw();

        unsafe {
            CurrentLimitReading::from_raw(
                sys::get_vrmsocmax_current(raw),
                sys::get_vrmsocmax_current_value(raw),
            )
        }
    }

    /// Returns the raw PSI0 VDD limit from the PM table, when available, .
    ///
    /// The unit is unknown, it could be `A` or `mA`.
    pub fn psi0_vdd_limit(&self) -> Option<f32> {
        let value = unsafe { sys::get_psi0_current(self.owner.as_raw()) };
        value.none_if_nan()
    }

    /// Returns the raw PSI0 SoC limit from the PM table, when available.
    ///
    /// The unit is unknown, it could be `A` or `mA`.
    pub fn psi0_soc_limit(&self) -> Option<f32> {
        let value = unsafe { sys::get_psi0soc_current(self.owner.as_raw()) };
        value.none_if_nan()
    }

    /// Returns the CCLK boost readings from the current PM table.
    pub fn cclk(&self) -> CclkReading {
        let raw = self.owner.as_raw();

        unsafe {
            CclkReading {
                setpoint: sys::get_cclk_setpoint(raw).none_if_nan(),
                busy: sys::get_cclk_busy_value(raw).none_if_nan(),
            }
        }
    }

    /// Returns L3 readings from the current PM table.
    pub fn l3(&self) -> L3Reading {
        let raw = self.owner.as_raw();

        unsafe {
            L3Reading {
                clock: sys::get_l3_clk(raw).none_if_nan(),
                logic: sys::get_l3_logic(raw).none_if_nan(),
                vddm: sys::get_l3_vddm(raw).none_if_nan(),
                temperature: sys::get_l3_temp(raw).none_if_nan(),
            }
        }
    }

    /// Returns the GFX readings from the current PM table.
    pub fn gfx(&self) -> GfxReading {
        let raw = self.owner.as_raw();

        unsafe {
            GfxReading {
                clock: sys::get_gfx_clk(raw).none_if_nan(),
                voltage: sys::get_gfx_volt(raw).none_if_nan(),
                temperature: sys::get_gfx_temp(raw).none_if_nan(),
            }
        }
    }

    /// Returns the fabric and memory readings from the current PM table.
    pub fn fabric_memory(&self) -> FabricMemoryReading {
        let raw = self.owner.as_raw();

        unsafe {
            FabricMemoryReading {
                fabric_clock: sys::get_fclk(raw).none_if_nan(),
                memory_clock: sys::get_mem_clk(raw).none_if_nan(),
            }
        }
    }

    /// Returns the SoC readings from the current PM table.
    pub fn soc(&self) -> SocReading {
        let raw = self.owner.as_raw();

        unsafe {
            SocReading {
                power: sys::get_soc_power(raw).none_if_nan(),
                voltage: sys::get_soc_volt(raw).none_if_nan(),
            }
        }
    }

    /// Returns the socket power from the current PM table.
    pub fn socket_power(&self) -> Option<f32> {
        let value = unsafe { sys::get_socket_power(self.owner.as_raw()) };
        value.none_if_nan()
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrentLimitReading, PowerLimitReading, TemperatureLimitReading};

    #[test]
    fn power_fields_are_independently_available() {
        let only_measured = PowerLimitReading::from_raw(f32::NAN, 35.0);
        assert_eq!(only_measured.limit, None);
        assert_eq!(only_measured.measured, Some(35.0));

        let only_limit = PowerLimitReading::from_raw(45.0, f32::NAN);
        assert_eq!(only_limit.limit, Some(45.0));
        assert_eq!(only_limit.measured, None);

        let zero = PowerLimitReading::from_raw(0.0, 0.0);
        assert_eq!(zero.limit, Some(0.0));
        assert_eq!(zero.measured, Some(0.0));
    }

    #[test]
    fn temperature_fields_are_independently_available() {
        let only_measured = TemperatureLimitReading::from_raw(f32::NAN, 72.0);
        assert_eq!(only_measured.limit, None);
        assert_eq!(only_measured.measured, Some(72.0));

        let only_limit = TemperatureLimitReading::from_raw(95.0, f32::NAN);
        assert_eq!(only_limit.limit, Some(95.0));
        assert_eq!(only_limit.measured, None);

        let zero = TemperatureLimitReading::from_raw(0.0, 0.0);
        assert_eq!(zero.limit, Some(0.0));
        assert_eq!(zero.measured, Some(0.0));
    }

    #[test]
    fn current_fields_are_independently_available() {
        let only_measured = CurrentLimitReading::from_raw(f32::NAN, 7.5);
        assert_eq!(only_measured.limit, None);
        assert_eq!(only_measured.measured, Some(7.5));

        let only_limit = CurrentLimitReading::from_raw(54.0, f32::NAN);
        assert_eq!(only_limit.limit, Some(54.0));
        assert_eq!(only_limit.measured, None);

        let zero = CurrentLimitReading::from_raw(0.0, 0.0);
        assert_eq!(zero.limit, Some(0.0));
        assert_eq!(zero.measured, Some(0.0));
    }
}
