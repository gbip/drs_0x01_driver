#![allow(dead_code)]
#![allow(unused_imports)]

use arrayvec::ArrayVec;

pub const TRAME_READER_INTERNAL_BUFFER_SIZE: usize = 64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ACKcmd {
    EEPWrite,
    EEPRead,
    RamWrite,
    RamRead,
    IJog,
    SJog,
    Stat,
    Rollback,
    Reboot,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StatusError {
    ExceedInputVoltageLimit,
    ExceedAllowedPOTLimit,
    ExceedTemperatureLimit,
    InvalidPacket,
    OverloadDetected,
    DriverFaultDetected,
    EEPREGDistorded,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StatusDetail {
    MovingFlag,
    ImpositionFlag,
    ChecksumError,
    UnknownCommand,
    ExceedREGRange,
    GarbageDetected,
    MotorOnFlag,
}

#[derive(Debug, PartialEq)]
struct ACKPacket {
    command: ACKcmd,
    error: Option<StatusError>,
    detail: Option<StatusDetail>,
}
struct ACKReader {
    pub(crate) state: ACKReaderState,
    buffer: ArrayVec<[ACKPacket; TRAME_READER_INTERNAL_BUFFER_SIZE]>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ACKReaderState {
    H1,
    H2,
    Psize,
    Pid,
    Cmd,
    Checksum1 {
        cmd: ACKcmd,
    },
    Checksum2 {
        cmd: ACKcmd,
    },
    DataAddr {
        cmd: ACKcmd,
    },
    DataLen {
        cmd: ACKcmd,
    },
    Data {
        cmd: ACKcmd,
        data_len: u8,
        current_index: u8,
    },
    Error {
        cmd: ACKcmd,
    },
    Detail {
        cmd: ACKcmd,
        error: Option<StatusError>,
    },
}

impl ACKReader {
    // Creation d'un ACKReader a l'état H1 et avec un buffer vide
    pub fn new() -> ACKReader {
        ACKReader {
            state: ACKReaderState::H1,
            buffer: ArrayVec::new(),
        }
    }

    // Renvoi le premier ACKcmd du buffer
    pub fn pop_ack(&mut self) -> Option<ACKPacket> {
        self.buffer.pop()
    }

    // Renvoi la taille du buffer
    pub fn get_buffer_size(&mut self) -> usize {
        self.buffer.len()
    }

    // Lit les octetc de l'ACK un par un
    fn parse(&mut self, buf: &[u8]) {
        for byte in buf {
            self.step(*byte);
        }
    }

    // Lit un octet et fait avancer ou non l'état
    pub fn step(&mut self, byte: u8) {
        use reader::ACKReaderState::*;
        use reader::ACKcmd::*;
        use reader::StatusDetail::*;
        use reader::StatusError::*;

        match self.state.clone() {
            H1 if byte == 0xFF => self.state = H2,
            H2 if byte == 0xFF => self.state = Psize,
            Psize => self.state = Pid,
            Pid => self.state = Cmd,
            Cmd => match byte {
                0x41 => self.state = Checksum1 { cmd: EEPWrite },
                0x42 => self.state = Checksum1 { cmd: EEPRead },
                0x43 => self.state = Checksum1 { cmd: RamWrite },
                0x44 => self.state = Checksum1 { cmd: RamRead },
                0x45 => self.state = Checksum1 { cmd: IJog },
                0x46 => self.state = Checksum1 { cmd: SJog },
                0x47 => self.state = Checksum1 { cmd: Stat },
                0x48 => self.state = Checksum1 { cmd: Rollback },
                0x49 => self.state = Checksum1 { cmd: Reboot },
                _ => {
                    self.state = H1;
                }
            },
            Checksum1 { ref cmd } => {
                self.state = Checksum2 { cmd: *cmd };
            }
            Checksum2 { ref cmd } => {
                self.state = DataAddr { cmd: *cmd };
            }
            DataAddr { ref cmd } => {
                self.state = DataLen { cmd: *cmd };
            }
            DataLen { ref cmd } if byte > 0 => {
                // POURQUOI REF ?? :(
                self.state = Data {
                    cmd: *cmd,
                    data_len: byte,
                    current_index: 0,
                };
            }
            DataLen { ref cmd } if byte == 0 => {
                self.state = Error { cmd: *cmd };
            }
            Data {
                ref cmd,
                data_len,
                current_index,
            }
                if current_index < data_len - 1 =>
            {
                self.state = Data {
                    cmd: *cmd,
                    data_len: data_len,
                    current_index: current_index + 1,
                }
            }
            Data {
                ref cmd,
                data_len,
                current_index,
            }
                if current_index == data_len - 1 =>
            {
                self.state = Error { cmd: *cmd };
            }
            Error { cmd } => match byte {
                0x00 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: None,
                    }
                }
                0x01 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: Some(ExceedInputVoltageLimit),
                    }
                }
                0x02 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: Some(ExceedAllowedPOTLimit),
                    }
                }
                0x04 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: Some(ExceedTemperatureLimit),
                    }
                }
                0x08 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: Some(InvalidPacket),
                    }
                }
                0x10 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: Some(OverloadDetected),
                    }
                }
                0x20 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: Some(DriverFaultDetected),
                    }
                }
                0x40 => {
                    self.state = Detail {
                        cmd: cmd,
                        error: Some(EEPREGDistorded),
                    }
                }
                _ => (),
            },
            Detail { ref cmd, ref error } => {
                let mut detail: Option<StatusDetail>;
                match byte {
                    0x00 => detail = None,
                    0x01 => detail = Some(MovingFlag),
                    0x02 => detail = Some(ImpositionFlag),
                    0x04 => detail = Some(ChecksumError),
                    0x08 => detail = Some(UnknownCommand),
                    0x10 => detail = Some(ExceedREGRange),
                    0x20 => detail = Some(GarbageDetected),
                    0x40 => detail = Some(MotorOnFlag),
                    _ => detail = None,
                }
                let packet = ACKPacket {
                    command: *cmd,
                    error: *error,
                    detail,
                };
                self.buffer.push(packet);
                self.state = H1;
            }
            _ => self.state = H1,
        }
    }
}

#[cfg(test)]
mod test {
    use reader::{ACKPacket, ACKReader, ACKcmd, StatusDetail, StatusError};
    #[test]
    fn test() {
        let mut reader = ACKReader::new();
        let packet = [
            0xFF, 0xFF, 0x0F, 0xFD, 0x42, 0x4C, 0xB2, 0x1E, 0x04, 0xB8, 0x01, 0x40, 0x1F, 0x00,
            0x00,
        ];

        reader.parse(&packet);
        assert_eq!(
            reader.pop_ack(),
            Some(ACKPacket {
                command: ACKcmd::EEPRead,
                error: None,
                detail: None
            })
        );
    }
}
