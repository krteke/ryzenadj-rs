use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milliwatts(pub u32);

/// A current limit in milliamperes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milliamps(pub u32);

/// A clock frequency in megahertz.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Megahertz(pub u32);

/// An integer number of seconds for an SMU time constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seconds(pub u32);

/// A temperature in whole degrees Celsius.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegreesCelsius(pub u32);

/// A raw firmware control value for the ramp after PROCHOT is deasserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProchotDeassertionRamp(pub u32);

/// A core overclocking voltage identification (VID) code.
///
/// The upstream CLI documents `VID = (1.55 - volts) / 0.00625`.
/// For example, `OcVid(48)` represents a requested voltage of 1.25 V.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OcVid(pub u32);

/// A signed Curve Optimizer adjustment in firmware-defined steps.
///
/// All-core and GFX requests pass the full 32-bit two's-complement representation to C:
/// `-30` becomes `0xFFFF_FFE2`, and `-1` becomes `0xFFFF_FFFF`.
/// Per-core requests require an offset that fits in `i16` and encode it in the low 16 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurveOptimizerOffset(pub i32);

/// A firmware core address, specified by CCD, CCX, and core indices.
///
/// The indices occupy bits 31–28, 27–24, and 23–20, respectively, following
/// [ZenStates-Core's encoding](https://github.com/irusanov/ZenStates-Core/blob/d08e0ac3e0e9ca26f740d39468405205d161d1ae/Hardware/Smu/Commands/SetPsmMarginSingleCore.cs#L3-L10).
/// This encoding has not been verified on hardware in this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoreAddress {
    bits: u32,
}

impl CoreAddress {
    /// Constructs an address with each index in `0..=15`.
    ///
    /// Returns [`Error::InvalidCoreAddress`] if any index exceeds its four-bit field.
    pub fn new(ccd: u8, ccx: u8, core: u8) -> Result<Self> {
        if ccd > 15 || ccx > 15 || core > 15 {
            return Err(Error::InvalidCoreAddress { ccd, ccx, core });
        }

        Ok(Self {
            bits: (u32::from(ccd) << 28) | (u32::from(ccx) << 24) | (u32::from(core) << 20),
        })
    }

    pub(crate) fn encode_curve_optimizer(self, offset: CurveOptimizerOffset) -> Result<u32> {
        let offset = i16::try_from(offset.0)
            .map_err(|_| Error::PerCoreCurveOptimizerOffsetOutOfRange(offset.0))?;

        // Keep the sign within the low 16 bits; bits 19–16 stay zero.
        Ok(self.bits | u32::from(offset as u16))
    }

    pub(crate) fn encode_oc_clock(self, frequency: Megahertz) -> Result<u32> {
        if frequency.0 > 0xF_FFFF {
            return Err(Error::PerCoreOcClockOutOfRange(frequency.0));
        }

        Ok(self.bits | frequency.0)
    }
}

/// A requested hardware performance preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformancePreference {
    PowerSaving,
    MaxPerformance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_address_fields_do_not_overlap() {
        for ((ccd, ccx, core), expected) in [
            ((0, 0, 0), 0),
            ((15, 0, 0), 0xF000_0000),
            ((0, 15, 0), 0x0F00_0000),
            ((0, 0, 15), 0x00F0_0000),
            ((15, 15, 15), 0xFFF0_0000),
        ] {
            let address = CoreAddress::new(ccd, ccx, core).unwrap();
            assert_eq!(address.encode_oc_clock(Megahertz(0)), Ok(expected));
        }
    }

    #[test]
    fn core_address_rejects_each_overflowing_index() {
        for (ccd, ccx, core) in [(16, 0, 0), (0, 16, 0), (0, 0, 16)] {
            assert_eq!(
                CoreAddress::new(ccd, ccx, core),
                Err(Error::InvalidCoreAddress { ccd, ccx, core }),
            );
        }
    }

    #[test]
    fn per_core_co_preserves_address_and_reserved_bits() {
        let address = CoreAddress::new(1, 2, 3).unwrap();
        for (offset, expected) in [
            (-32768, 0x1230_8000),
            (-30, 0x1230_FFE2),
            (-1, 0x1230_FFFF),
            (0, 0x1230_0000),
            (30, 0x1230_001E),
            (32767, 0x1230_7FFF),
        ] {
            assert_eq!(
                address.encode_curve_optimizer(CurveOptimizerOffset(offset)),
                Ok(expected),
            );
        }
    }

    #[test]
    fn per_core_co_rejects_offsets_outside_signed_16_bits() {
        let address = CoreAddress::new(1, 2, 3).unwrap();
        for offset in [-32769, 32768] {
            assert_eq!(
                address.encode_curve_optimizer(CurveOptimizerOffset(offset)),
                Err(Error::PerCoreCurveOptimizerOffsetOutOfRange(offset)),
            );
        }
    }

    #[test]
    fn per_core_oc_uses_only_the_low_20_bits() {
        let address = CoreAddress::new(1, 2, 3).unwrap();
        for (frequency, expected) in [
            (0, 0x1230_0000),
            (4200, 0x1230_1068),
            (0xF_FFFF, 0x123F_FFFF),
        ] {
            assert_eq!(address.encode_oc_clock(Megahertz(frequency)), Ok(expected));
        }
        assert_eq!(
            address.encode_oc_clock(Megahertz(0x10_0000)),
            Err(Error::PerCoreOcClockOutOfRange(0x10_0000)),
        );
    }
}
