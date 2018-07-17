use builder::{HerkulexMessage, MessageBuilder};

use message::{JogColor, JogMode};

use addr::*;

use core::cmp::min;

/// This struct allows you to quickly build messages for a servomotor.
#[derive(Debug)]
pub struct Servo {
    id: u8,
}

impl Default for Servo {
    fn default() -> Self {
        Servo { id: 0xFD }
    }
}

impl Servo {
    /// Create a new Servo with the given ID.
    ///
    /// # Notes
    ///
    /// * Valid ID are in the range 0..253.
    /// * 254 is the broadcast ID.
    pub fn new(id: u8) -> Servo {
        Servo { id }
    }

    /// Create a reboot message requesting the servo to reboot.
    /// During the reboot all changes applied to the EEP memory will take effect.
    pub fn reboot(&self) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id).reboot().build()
    }

    /// Request the servo to go to a position.
    pub fn set_position(&self, position: u16) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id)
            .s_jog(
                60,
                JogMode::Normal {
                    position: min(position, 1023),
                },
                JogColor::Blue,
                self.id,
            )
            .build()
    }

    /// Request the servo to have a certain speed.
    pub fn set_speed(&self, speed: u16) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id)
            .s_jog(
                60,
                JogMode::Continuous {
                    speed: min(speed, 1023),
                },
                JogColor::Blue,
                self.id,
            )
            .build()
    }

    /// Request the servo to send it's status.
    pub fn stat(&self) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id).stat().build()
    }

    /// Write to the volatile RAM of the servo.
    /// Ram is cleared on every reboot, and populated with data from the EEP memory.
    pub fn ram_write(&self, addr: WritableRamAddr) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id).write_ram(addr).build()
    }

    /// Write to the permanent EEP memory.
    /// For the change to take effect you need to reboot the servo so that the values are loaded
    /// in RAM.
    pub fn eep_write(&self, addr: WritableEEPAddr) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id).write_eep(addr).build()
    }

    /// Request the servo to send back some data from RAM.
    pub fn ram_request(&self, addr: ReadableRamAddr) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id)
            .read_ram(addr, addr.bytes())
            .build()
    }

    /// Request the servo to send back some data from EEP.
    pub fn eep_request(&self, addr: ReadableEEPAddr) -> HerkulexMessage {
        MessageBuilder::new_with_id(self.id)
            .read_eep(addr, addr.bytes())
            .build()
    }
}
