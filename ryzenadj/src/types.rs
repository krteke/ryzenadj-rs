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
