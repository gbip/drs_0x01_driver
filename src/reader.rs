#![allow(dead_code)]
#![allow(unused_imports)]

use arrayvec::ArrayVec;

pub const TRAME_READER_INTERNAL_BUFFER_SIZE: usize = 64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ACKcmd {
    EEPWrite,
    EEPRead, // contient data (doc p42)
    RamWrite,
    RamRead, // contient data (doc p45)
    IJog,
    SJog,
    Stat,     // no data
    Rollback, // no data
    Reboot,   // no data
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
    data_addr: Option<u8>,
    data_len: Option<u8>,
    data: Option<[u8; 16]>, // doc p20
    error: Option<StatusError>,
    detail: Option<StatusDetail>,
}

struct ACKReader {
    pub(crate) state: ACKReaderState,
    buffer: ArrayVec<[ACKPacket; TRAME_READER_INTERNAL_BUFFER_SIZE]>,
}

struct AssociatedData {
    error: Option<u8>,
    status: Option<u8>,
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
        data_addr: Option<u8>,
    },
    Data {
        cmd: ACKcmd,
        data_addr: Option<u8>,
        data_len: Option<u8>,
        data: Option<[u8; 16]>,
        current_index: u8,
    },
    Error {
        cmd: ACKcmd,
        data_addr: Option<u8>,
        data_len: Option<u8>,
        data: Option<[u8; 16]>,
    },
    Detail {
        cmd: ACKcmd,
        data_addr: Option<u8>,
        data_len: Option<u8>,
        data: Option<[u8; 16]>,
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

    // Renvoi le premier ACKPacket du buffer
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
            // Si la commande etait EEPRead ou RamRead, on recupere des donnees
            DataAddr { ref cmd } if (*cmd == EEPRead || *cmd == RamRead) => {
                self.state = DataLen {
                    cmd: *cmd,
                    data_addr: Some(byte),
                };
            }
            // Sinon on passe à l'état suivant
            DataAddr { ref cmd } => {
                self.state = Error {
                    cmd: *cmd,
                    data_addr: None,
                    data_len: None,
                    data: None,
                };
            }
            // Si on doit recuperer des donnees, on renvoie aussi la taille de ces donnees
            DataLen {
                ref cmd,
                ref data_addr,
            }
                if byte > 0 =>
            {
                self.state = Data {
                    cmd: *cmd,
                    data_addr: *data_addr,
                    data_len: Some(byte),
                    data: Some([0x00; 16]),
                    current_index: 0,
                };
            }
            // Si DataLen = 0 passer a l'etat suivant
            DataLen {
                ref cmd,
                ref data_addr,
            }
                if byte == 0 =>
            {
                self.state = Error {
                    cmd: *cmd,
                    data_addr: *data_addr,
                    data_len: None,
                    data: None,
                };
            }
            Data {
                ref cmd,
                ref data_addr,
                ref data_len,
                data,
                current_index,
            }
                if current_index < data_len.unwrap() - 1 =>
            {
                let mut in_data = data.unwrap(); // c'est pas joli mais ca marche :)
                in_data[current_index as usize] = byte; // c'est pas joli mais ca marche :)
                self.state = Data {
                    cmd: *cmd,
                    data_addr: *data_addr,
                    data_len: *data_len,
                    data: Some(in_data),
                    current_index: current_index + 1,
                }
            }
            Data {
                ref cmd,
                ref data_addr,
                ref data_len,
                ref data,
                current_index,
            }
                if current_index == data_len.unwrap() - 1 =>
            {
                let mut in_data = data.unwrap(); // c'est pas joli mais ca marche :)
                in_data[current_index as usize] = byte; // c'est pas joli mais ca marche :)
                self.state = Error {
                    cmd: *cmd,
                    data_addr: *data_addr,
                    data_len: *data_len,
                    data: Some(in_data),
                };
            }
            Error {
                ref cmd,
                ref data_addr,
                ref data_len,
                ref mut data,
            } => match byte {
                0x00 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: None,
                    }
                }
                0x01 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: Some(ExceedInputVoltageLimit),
                    }
                }
                0x02 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: Some(ExceedAllowedPOTLimit),
                    }
                }
                0x04 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: Some(ExceedTemperatureLimit),
                    }
                }
                0x08 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: Some(InvalidPacket),
                    }
                }
                0x10 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: Some(OverloadDetected),
                    }
                }
                0x20 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: Some(DriverFaultDetected),
                    }
                }
                0x40 => {
                    self.state = Detail {
                        cmd: *cmd,
                        data_addr: *data_addr,
                        data_len: *data_len,
                        data: *data,
                        error: Some(EEPREGDistorded),
                    }
                }
                _ => self.state = H1,
            },
            Detail {
                ref cmd,
                ref data_addr,
                ref data_len,
                ref data,
                ref error,
            } => {
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
                    data_addr: *data_addr,
                    data_len: *data_len,
                    data: *data,
                    error: *error,
                    detail: detail,
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

        // Test de EEPRead
        let packet_eepread = [
            0xFF, 0xFF, 0x0F, 0xFD, 0x42, 0x4C, 0xB2, 0x1E, 0x04, 0xB8, 0x01, 0x40, 0x1F, 0x08,
            0x20,
        ];

        reader.parse(&packet_eepread);

        let data_eepread: [u8; 16] = [
            0xB8, 0x01, 0x40, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        assert_eq!(
            reader.pop_ack(),
            Some(ACKPacket {
                command: ACKcmd::EEPRead,
                data_addr: Some(0x1E),
                data_len: Some(0x04),
                data: Some(data_eepread),
                error: Some(StatusError::InvalidPacket),
                detail: Some(StatusDetail::GarbageDetected),
            })
        );

        // Test de RAMRead
        let packet_ramread = [
            0xFF, 0xFF, 0x0C, 0xFD, 0x44, 0xC2, 0x3C, 0x35, 0x01, 0x01, 0x00, 0x40,
        ];

        reader.parse(&packet_ramread);

        let data_ramread: [u8; 16] = [
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        assert_eq!(
            reader.pop_ack(),
            Some(ACKPacket {
                command: ACKcmd::RamRead,
                data_addr: Some(0x35),
                data_len: Some(0x01),
                data: Some(data_ramread),
                error: None,
                detail: Some(StatusDetail::MotorOnFlag),
            })
        );
    }
}
