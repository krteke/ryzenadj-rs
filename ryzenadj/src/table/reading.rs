use crate::NanExt;

/// A power limit and its corresponding PM Table reading, in `watts`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerLimitReading {
    pub limit: Option<f32>,
    pub measured: Option<f32>,
}

impl PowerLimitReading {
    pub(crate) fn from_raw(limit: f32, measured: f32) -> Self {
        Self {
            limit: limit.none_if_nan(),
            measured: measured.none_if_nan(),
        }
    }
}

/// A temperature limit and its corresponding reading, in degrees Celsius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemperatureLimitReading {
    pub limit: Option<f32>,
    pub measured: Option<f32>,
}

impl TemperatureLimitReading {
    pub(crate) fn from_raw(limit: f32, measured: f32) -> Self {
        Self {
            limit: limit.none_if_nan(),
            measured: measured.none_if_nan(),
        }
    }
}

/// A VRM current limit and its measured value, in amperes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurrentLimitReading {
    pub limit: Option<f32>,
    pub measured: Option<f32>,
}

impl CurrentLimitReading {
    pub(crate) fn from_raw(limit: f32, measured: f32) -> Self {
        Self {
            limit: limit.none_if_nan(),
            measured: measured.none_if_nan(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoreReading {
    pub clock: Option<f32>,
    pub voltage: Option<f32>,
    pub power: Option<f32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GfxReading {
    pub clock: Option<f32>,
    pub voltage: Option<f32>,
    /// The Strix Point C getter uses an offset described as an unknown clock.
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SocReading {
    pub power: Option<f32>,
    pub voltage: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct L3Reading {
    pub clock: Option<f32>,
    pub logic: Option<f32>,
    pub vddm: Option<f32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FabricMemoryReading {
    /// On Van Gogh, the C FCLK and memory offsets resolve to the same float slot.
    pub fabric_clock: Option<f32>,
    /// MT/s ?
    pub memory_clock: Option<f32>,
}

/// CCLK boost
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CclkReading {
    /// On Strix Point, C uses the same table offset as `get_socket_power`.
    pub setpoint: Option<f32>,
    pub busy: Option<f32>,
}
