use addr::ReadableEEPAddr;
use addr::ReadableRamAddr;
use addr::WritableEEPAddr;
use addr::WritableRamAddr;

use arrayvec::ArrayVec;

pub enum RegisterRequest {
    EEPWrite(WritableEEPAddr),
    EEPRead(ReadableEEPAddr),
    RamWrite(WritableRamAddr),
    RamRead(ReadableRamAddr),
}

pub(crate) struct SJogRequest {
    pub(crate) data: ArrayVec<[SJogData; 10]>,
    pub(crate) playtime: u8,
}

pub(crate) type IJogRequest = ArrayVec<[IJogData; 10]>;

#[derive(Debug)]
pub enum SpecialRequest {
    Stat,
    Rollback { skip_id: u8, skip_baud: u8 },
    Reboot,
}

#[derive(Clone, Copy)]
pub enum Rollback {
    SkipId,
    SkipBaud,
    SkipBoth,
    SkipNone,
}

/// This represent the rotation sense of the servomotor while controlled in `Speed`.
#[derive(Debug)]
pub enum Rotation {
    /// CounterClockwise rotation, which is the default rotation sense.
    CounterClockwise,
    /// Clockwise rotation, representing the inverted rotation.
    Clockwise,
}

/// This represent the servomotor control mode.
/// The servomotor is either controlled in `Position` or `Speed`.
#[derive(Debug)]
pub enum JogMode {
    /// Control the servomotor by position.
    /// Make sure that the position is in range for your servomotor.
    Normal {
        /// The calibrated position.
        /// The value must be in the 0..1023 range
        position: u16,
    },
    /// Control the servomotor by speed.
    /// Make sur that you respect the maximum speed.
    /// The 14th bit represent the sign, if it set the servomotor will rotate the other way.
    Continuous {
        /// The desired PWM value.
        /// The value must be in the 0..1023 range
        speed: u16,
        /// Inverts the rotation sense of the servo by modifying the 14th bit.
        rotation: Rotation,
    },
}

impl JogMode {
    pub(crate) fn associated_data(&self) -> u16 {
        match *self {
            JogMode::Normal { position } => position,
            JogMode::Continuous { speed, rotation: Rotation::Clockwise } => 0x4000 | speed,
            JogMode::Continuous { speed, rotation: Rotation::CounterClockwise } => speed,
        }
    }
}

impl Default for JogMode {
    fn default() -> Self {
        JogMode::Normal { position: 0 }
    }
}

/// The color of the LED of the servomotor.
#[derive(Debug)]
pub enum JogColor {
    /// Red
    Red,
    /// Green
    Green,
    /// Blue
    Blue,
}

impl Default for JogColor {
    fn default() -> Self {
        JogColor::Green
    }
}

#[derive(Default, Debug)]
pub(crate) struct SJogData {
    pub mode: JogMode,
    pub color: JogColor,
    pub id: u8,
}

impl SJogData {
    pub(crate) fn new(mode: JogMode, color: JogColor, id: u8) -> Self {
        SJogData { mode, color, id }
    }
}

#[derive(Default, Debug)]
pub(crate) struct IJogData {
    pub mode: JogMode,
    pub color: JogColor,
    pub playtime: u8,
    pub id: u8,
}

impl IJogData {
    pub(crate) fn new(mode: JogMode, color: JogColor, playtime: u8, id: u8) -> Self {
        IJogData {
            mode,
            color,
            id,
            playtime,
        }
    }
}
