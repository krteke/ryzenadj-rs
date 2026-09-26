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

/// A core overclocking voltage identification (VID) code.
///
/// The upstream CLI documents `VID = (1.55 - volts) / 0.00625`.
/// For example, `OcVid(48)` represents a requested voltage of 1.25 V.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OcVid(pub u32);

/// A requested hardware performance preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformancePreference {
    PowerSaving,
    MaxPerformance,
}
