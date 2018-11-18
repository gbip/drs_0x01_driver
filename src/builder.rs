use addr::*;
use message::*;

use arrayvec::ArrayVec;

/// The error returned by [`MessageBuilder`](struct.MessageBuilder.html).
pub enum MessageBuilderError {
    /// The maximum number of data has been reached for this message.
    MaximumDataReached,
}

struct Packet {
    pid: u8,
    cmd: u8,
    data: [u8; 16],
    data_size: usize,
}

impl Default for Packet {
    fn default() -> Packet {
        Packet {
            pid: 0,
            cmd: 0,
            data: [0; 16],
            data_size: 0,
        }
    }
}

impl Packet {
    fn build(self) -> HerkulexMessage {
        let mut result = HerkulexMessage::new();
        let size: u8 = self.data_size as u8 + 7;
        let mut checksum1: u8 = size ^ self.pid ^ self.cmd;
        result.push(0xFF);
        result.push(0xFF);
        result.push(size);
        result.push(self.pid);
        result.push(self.cmd);
        for i in 0..self.data_size {
            result.push(self.data[i]);
            checksum1 ^= self.data[i];
        }
        checksum1 &= 0xFE;
        let checksum2: u8 = (!checksum1) & 0xFE;
        result.insert(5, checksum1);
        result.insert(6, checksum2);
        result
    }

    fn push_data(&mut self, data: u8) {
        self.data[self.data_size] = data;
        self.data_size += 1;
    }
}

/// This is the type of all the message provided by this crate.
pub type HerkulexMessage = ArrayVec<[u8; 128]>;

/// This struct allows you to build message to directly speak to the herkulex servomotors.
#[derive(Default)]
pub struct MessageBuilder {}

/// This is a specialized version of the [`MessageBuilder`](struct.MessageBuilder.html) which
/// contains an ID. It is
/// used to
/// build other types of builders such as :
/// * [MessageBuilderMem](struct.MessageBuilderMem.html)
/// * [MessageBuilderPosition](struct.MessageBuilderPosition.html)
/// * [MessageBuilderSpecial](struct.MessageBuilderSpecial.html)
pub struct MessageBuilderCmd {
    pid: u8,
}

/// This is a specialized version of the [`MessageBuilder`](struct.MessageBuilder.html) which contains an ID and a memory
/// request (read or write, and where).
pub struct MessageBuilderMem {
    pid: u8,
    addr: RegisterRequest,
    size: u8,
}

/// This is a specialized version of the [`MessageBuilder`](struct.MessageBuilder.html) which contains an ID and a position
/// request.
pub struct MessageBuilderPositionSJOG {
    pid: u8,
    pos: SJogRequest,
}

/// This is a specialized version of the [`MessageBuilder`](struct.MessageBuilder.html) which contains an ID and a position
/// request.
pub struct MessageBuilderPositionIJOG {
    pid: u8,
    pos: IJogRequest,
}

/// This is a specialized version of the [`MessageBuilder`](struct.MessageBuilder.html) which contains an ID and a special
/// request (reboot, reset or stat).
pub struct MessageBuilderSpecial {
    pid: u8,
    kind: SpecialRequest,
}

impl MessageBuilder {
    /// Create a new message builder
    pub fn new() -> MessageBuilder {
        MessageBuilder {}
    }

    /// Create a new message builder with a preassigned ID.
    pub fn new_with_id(id: u8) -> MessageBuilderCmd {
        MessageBuilderCmd { pid: id }
    }

    /// Set the message ID to choose the servo
    pub fn id(self, id: u8) -> MessageBuilderCmd {
        MessageBuilderCmd { pid: id }
    }
}

impl MessageBuilderCmd {
    /// Create a message of type **RAM_READ** (read from the temporary memory)
    pub fn read_ram<T: Into<Option<u8>>>(
        self,
        ram_addr: ReadableRamAddr,
        size: T,
    ) -> MessageBuilderMem {
        MessageBuilderMem {
            pid: self.pid,
            addr: RegisterRequest::RamRead(ram_addr),
            size: match size.into() {
                Some(s) => s,
                None => ram_addr.bytes(),
            },
        }
    }

    /// Create a message of type **RAM_WRITE** (write to the temporary memory, last until the servo
    /// is restarted)
    pub fn write_ram(self, ram_addr: WritableRamAddr) -> MessageBuilderMem {
        MessageBuilderMem {
            pid: self.pid,
            addr: RegisterRequest::RamWrite(ram_addr),
            size: ram_addr.bytes(),
        }
    }

    /// Create a message of type **READ_EEP** (read the permanent memory)
    pub fn read_eep<T: Into<Option<u8>>>(
        self,
        eep_addr: ReadableEEPAddr,
        size: T,
    ) -> MessageBuilderMem {
        MessageBuilderMem {
            pid: self.pid,
            addr: RegisterRequest::EEPRead(eep_addr),
            size: match size.into() {
                Some(s) => s,
                None => eep_addr.bytes(),
            },
        }
    }

    /// Create a message of type **WRITE_EEP** (write to the permanent memory, require a reboot to
    /// take effect).
    pub fn write_eep(self, eep_addr: WritableEEPAddr) -> MessageBuilderMem {
        MessageBuilderMem {
            pid: self.pid,
            addr: RegisterRequest::EEPWrite(eep_addr),
            size: eep_addr.bytes(),
        }
    }

    /// Create a message of type **REBOOT** (reboot the designed servos)
    pub fn reboot(self) -> MessageBuilderSpecial {
        MessageBuilderSpecial {
            pid: self.pid,
            kind: SpecialRequest::Reboot,
        }
    }

    /// Create a message of type **ROLLBACK** (reset EEP memory)
    pub fn rollback(self, flags: Rollback) -> MessageBuilderSpecial {
        let kind = match flags {
            Rollback::SkipId => SpecialRequest::Rollback {
                skip_id: 1,
                skip_baud: 0,
            },
            Rollback::SkipBaud => SpecialRequest::Rollback {
                skip_id: 0,
                skip_baud: 1,
            },
            Rollback::SkipBoth => SpecialRequest::Rollback {
                skip_id: 1,
                skip_baud: 1,
            },
            Rollback::SkipNone => SpecialRequest::Rollback {
                skip_id: 0,
                skip_baud: 0,
            },
        };
        MessageBuilderSpecial {
            pid: self.pid,
            kind,
        }
    }

    /// Create a message of type **STAT** (request servo status)
    pub fn stat(self) -> MessageBuilderSpecial {
        MessageBuilderSpecial {
            pid: self.pid,
            kind: SpecialRequest::Stat,
        }
    }

    /// Create a new message of type **SJog**.
    /// There is a maximum of 10 servomotors that can be controlled per sjog message.
    ///
    /// # Invalid Values
    ///
    /// The maximum `playtime` value is `0xFE`.
    /// The maximum `id` value is `0xFE`.
    /// The maximum `position` value is 1024.
    pub fn s_jog(
        self,
        playtime: u8,
        mode: JogMode,
        color: JogColor,
        id: u8,
    ) -> MessageBuilderPositionSJOG {
        let mut result = MessageBuilderPositionSJOG {
            pid: self.pid,
            pos: SJogRequest {
                data: ArrayVec::new(),
                playtime,
            },
        };
        result.pos.data.push(SJogData::new(mode, color, id));
        result
    }

    /// Create a new message of type **IJOG**
    /// There is a maximum of 10 servomotors that can be controlled per ijog message.
    ///
    /// # Invalid Values
    ///
    /// The maximum `playtime` value is `0xFE`.
    /// The maximum `id` value is `0xFE`.
    /// The maximum `position` value is 1024.
    pub fn i_jog(
        self,
        playtime: u8,
        mode: JogMode,
        color: JogColor,
        id: u8,
    ) -> MessageBuilderPositionIJOG {
        let mut result = MessageBuilderPositionIJOG {
            pid: self.pid,
            pos: ArrayVec::new(),
        };
        result.pos.push(IJogData::new(mode, color, playtime, id));
        result
    }
}

impl MessageBuilderMem {
    /// Build the final message to be sent to the servomotor through a serial connection.
    pub fn build(self) -> HerkulexMessage {
        let pid = self.pid;
        let cmd = match self.addr {
            RegisterRequest::EEPWrite(_) => 0x01,
            RegisterRequest::EEPRead(_) => 0x02,
            RegisterRequest::RamWrite(_) => 0x03,
            RegisterRequest::RamRead(_) => 0x04,
        };
        let mut packet = Packet::default();
        packet.pid = pid;
        packet.cmd = cmd;

        // TODO : Check write data sizes
        match self.addr {
            // EEP Write packet
            RegisterRequest::EEPWrite(addr) => {
                packet.push_data(addr.into());
                packet.push_data(self.size);
                let (d1, opt_d2) = addr.associated_data();
                packet.push_data(d1);
                if let Some(d2) = opt_d2 {
                    packet.push_data(d2);
                }
            }

            // RAM Write packet
            RegisterRequest::RamWrite(addr) => {
                packet.push_data(addr.into());
                packet.push_data(self.size);
                let (d1, opt_d2) = addr.associated_data();
                packet.push_data(d1);
                if let Some(d2) = opt_d2 {
                    packet.push_data(d2);
                }
            }

            // EEP Read packet
            RegisterRequest::EEPRead(addr) => {
                packet.push_data(addr.into());
                packet.push_data(self.size);
            }

            // Ram Read packet
            RegisterRequest::RamRead(addr) => {
                packet.push_data(addr.into());
                packet.push_data(self.size);
            }
        }
        packet.build()
    }
}

impl MessageBuilderSpecial {
    /// Build the final message to be sent to the servomotor through a serial connection.
    pub fn build(self) -> HerkulexMessage {
        let cmd = match self.kind {
            SpecialRequest::Stat => 0x07,
            SpecialRequest::Rollback { .. } => 0x08,
            SpecialRequest::Reboot => 0x09,
        };
        let mut packet = Packet::default();
        packet.pid = self.pid;
        packet.cmd = cmd;
        if let SpecialRequest::Rollback {
            skip_id: id_bit,
            skip_baud: baud_bit,
        } = self.kind
        {
            packet.push_data(id_bit);
            packet.push_data(baud_bit);
        }
        packet.build()
    }
}

impl MessageBuilderPositionSJOG {
    /// Append a new **SJOG** command to this message.
    ///
    /// # Errors
    ///
    /// Return [MessageBuilderError::MaximumDataReached](enum.MessageBuilderError.html) if there
    /// is already 10 sjog command in this message.
    ///
    /// # Invalid Values
    ///
    /// The maximum `id` value is `0xFE`.
    /// The maximum `playtime` value is `0xFE`.
    pub fn s_jog(
        &mut self,
        mode: JogMode,
        color: JogColor,
        id: u8,
    ) -> Result<(), MessageBuilderError> {
        if self.pos.data.is_full() {
            Err(MessageBuilderError::MaximumDataReached)
        } else {
            self.pos.data.push(SJogData::new(mode, color, id));
            Ok(())
        }
    }

    /// Build the final message to be sent to the servomotor through a serial connection.
    pub fn build(self) -> HerkulexMessage {
        let mut packet = Packet::default();
        packet.pid = self.pid;
        packet.cmd = 6;
        packet.push_data(self.pos.playtime);
        for data in self.pos.data {
            let d = data.mode.associated_data();
            let lsb = (d) as u8;
            let msb = (d >> 8) as u8;
            packet.push_data(lsb);
            packet.push_data(msb);

            let mut set: u8 = 0;
            match data.mode {
                JogMode::Normal { .. } => set |= 0b0000_0000,
                JogMode::Continuous { .. } => set |= 0b0000_0010,
            }
            match data.color {
                JogColor::Blue => set |= 0b0000_1000,
                JogColor::Green => set |= 0b0000_0100,
                JogColor::Red => set |= 0b0001_0000,
            }
            packet.push_data(set);
            packet.push_data(data.id);
        }
        packet.build()
    }
}

impl MessageBuilderPositionIJOG {
    /// Append a new **SJOG** command to this message.
    ///
    /// # Errors
    ///
    /// Return [MessageBuilderError::MaximumDataReached](enum.MessageBuilderError.html) if there
    /// is already 10 sjog command in this message.
    ///
    /// # Invalid Values
    ///
    /// The maximum `id` value is `0xFE`.
    pub fn s_jog(
        &mut self,
        mode: JogMode,
        color: JogColor,
        playtime: u8,
        id: u8,
    ) -> Result<(), MessageBuilderError> {
        if self.pos.is_full() {
            Err(MessageBuilderError::MaximumDataReached)
        } else {
            self.pos.push(IJogData::new(mode, color, playtime, id));
            Ok(())
        }
    }

    /// Build the final message to be sent to the servomotor through a serial connection.
    pub fn build(self) -> HerkulexMessage {
        let mut packet = Packet::default();
        packet.pid = self.pid;
        packet.cmd = 5;
        for data in self.pos {
            let d = data.mode.associated_data();
            let lsb = (d) as u8;
            let msb = (d >> 8) as u8;
            packet.push_data(lsb);
            packet.push_data(msb);

            let mut set: u8 = 0;
            match data.mode {
                JogMode::Normal { .. } => set |= 0b0000_0000,
                JogMode::Continuous { .. } => set |= 0b0000_0010,
            }
            match data.color {
                JogColor::Blue => set |= 0b0000_1000,
                JogColor::Green => set |= 0b0000_0100,
                JogColor::Red => set |= 0b0001_0000,
            }
            packet.push_data(set);
            packet.push_data(data.id);
            packet.push_data(data.playtime);
        }
        packet.build()
    }
}

#[cfg(test)]
mod test {

    use addr::ReadableEEPAddr;
    use builder::*;

    #[test]
    fn reboot_message() {
        let message = MessageBuilder::new().id(0xFD).reboot().build();
        assert_eq!(message.len(), 0x07);
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x07, 0xFD, 0x09, 0xF2, 0x0C]
        );
    }

    #[test]
    fn ram_read_message() {
        let message = MessageBuilder::new()
            .id(0xFD)
            .read_ram(ReadableRamAddr::LEDControl, None)
            .build();
        assert_eq!(message.len(), 0x09);
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x09, 0xFD, 0x04, 0xC4, 0x3A, 0x35, 0x01]
        )
    }

    #[test]
    fn ram_write_message() {
        let message = MessageBuilder::new()
            .id(0xFD)
            .write_ram(WritableRamAddr::LEDControl(0x01))
            .build();
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x0A, 0xFD, 0x03, 0xC0, 0x3E, 0x35, 0x01, 0x01]
        );

        let message = MessageBuilder::new()
            .id(0xFD)
            .write_ram(WritableRamAddr::TorqueControl(0x60))
            .build();
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x0A, 0xFD, 0x03, 0xA0, 0x5E, 0x34, 0x01, 0x60]
        );
    }

    #[test]
    fn rollback_message() {
        let message = MessageBuilder::new()
            .id(0xFD)
            .rollback(Rollback::SkipBoth)
            .build();
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x09, 0xFD, 0x08, 0xFC, 0x02, 1, 1]
        )
    }

    #[test]
    fn stat_message() {
        let message = MessageBuilder::new().id(0xFD).stat().build();
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x07, 0xFD, 0x07, 0xFC, 0x02]
        );
    }

    #[test]
    fn eep_read() {
        let message = MessageBuilder::new()
            .id(0xFD)
            .read_eep(ReadableEEPAddr::PositionKp, 4)
            .build();
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x09, 0xFD, 0x02, 0xEC, 0x12, 0x1E, 0x04]
        );
    }

    #[test]
    fn sjog_message() {
        let message = MessageBuilder::new()
            .id(0xFD)
            .s_jog(60, JogMode::Normal { position: 512 }, JogColor::Green, 0xFD)
            .build();

        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x0C, 0xFD, 0x06, 0x30, 0xCE, 0x3C, 0x00, 0x02, 0x04, 0xFD,]
        );

        let message = MessageBuilder::new()
            .id(0xFD)
            .s_jog(60, JogMode::Continuous { speed: 320 }, JogColor::Blue, 0xFD)
            .build();

        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x0C, 0xFD, 0x06, 124, 130, 0x3C, 0x40, 0x01, 0x0A, 0xFD,]
        )
    }

    #[test]
    fn ijog_message() {
        let message = MessageBuilder::new()
            .id(0xFD)
            .i_jog(60, JogMode::Normal { position: 512 }, JogColor::Green, 0xFD)
            .build();
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x0C, 0xFD, 0x05, 0x32, 0xCC, 0x00, 0x02, 0x04, 0xFD, 0x3C,]
        );

        let message = MessageBuilder::new()
            .id(0xFD)
            .i_jog(60, JogMode::Continuous { speed: 320 }, JogColor::Blue, 0xFD)
            .build();
        assert_eq!(
            message.as_slice(),
            &[0xFF, 0xFF, 0x0C, 0xFD, 0x05, 0x7E, 0x80, 0x40, 0x01, 0x0A, 0xFD, 0x3C,]
        )
    }

}
