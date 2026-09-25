use crate::NanExt;

/// A power limit and its corresponding PM Table reading, in `watts`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerLimitReading {
    pub limit: f32,
    pub measured: f32,
}

/// Tctl temperature readings in `degrees Celsius`.
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
