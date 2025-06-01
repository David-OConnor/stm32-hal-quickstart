//! This module keeps tracks of the health of various peripherals. Which items are tracked,
//! and the semantic meaning of the statuses depend on the project.

use defmt::Format;
use hal::instant::Instant;

// #[derive(Format)]
pub struct SystemStatus {
    // Add fields as required.

    // pub imu: SensorStatus,
    // pub baro: SensorStatus,
    pub update_timestamps: UpdateTimestamps,
}

#[derive(Clone, Copy, Default, PartialEq, Format)]
#[repr(u8)] // For serialization
pub enum SensorStatus {
    Pass = 0,
    Fault = 1,
    #[default]
    /// Either an external sensor not plugged in, or a complete failture, werein it's not recognized.
    NotConnected = 2,
}

/// Times, in seconds since start, of the last valid reading received.
/// A `None` value means have never received an update.
#[derive(Default)]
pub struct UpdateTimestamps {
    pub imu: Option<Instant>,
    pub baro: Option<Instant>,
}
