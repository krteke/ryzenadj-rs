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
/// The full 32-bit two's-complement representation is passed to C:
/// `-30` becomes `0xFFFF_FFE2`, and `-1` becomes `0xFFFF_FFFF`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurveOptimizerOffset(pub i32);

/// A requested hardware performance preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformancePreference {
    PowerSaving,
    MaxPerformance,
}
