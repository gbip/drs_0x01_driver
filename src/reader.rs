#![allow(dead_code)]
#![allow(unused_imports)]

use arrayvec::ArrayVec;

use addr::EEPReadData;
use addr::RamReadData;
use addr::WritableEEPAddr;
use addr::WritableRamAddr;

pub const TRAME_READER_INTERNAL_BUFFER_SIZE: usize = 64;

#[derive(Debug, PartialEq, Eq)]
pub struct ACKPacket {
    psize: u8,
    pid: u8,
    cmd: Command,
    chk1: u8,
    chk2: u8,
    error: StatusError,
    detail: StatusDetail,
}

impl ACKPacket {
    pub fn is_valid(&self) -> bool {
        use addr::ReadableEEPAddr;
        use addr::ReadableRamAddr;
        use reader::Command::*;

        // Construction de chk1
        let mut chk1 = self.psize;
        chk1 ^= self.pid;
        chk1 ^= u8::from(self.cmd);

        match self.cmd {
            Command::EEPRead { data } => {
                let a: u8 = data.addr.into();
                chk1 ^= a;
                chk1 ^= data.data_len;
                for i in &data.data[0..data.data_len as usize] {
                    chk1 ^= i;
                }
            }
            Command::RamRead { data } => {
                let a: u8 = data.addr.into();
                chk1 ^= a;
                chk1 ^= data.data_len;
                for i in &data.data[0..data.data_len as usize] {
                    chk1 ^= i;
                }
            }
            _ => (),
        };
        chk1 &= 0xFE;
        self.chk1 == chk1 && self.chk2 == !chk1 & 0xFE
    }
}

impl Into<Command> for ACKPacket {
    fn into(self) -> Command {
        self.cmd
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    EEPWrite,
    EEPRead { data: EEPReadData },
    RamWrite,
    RamRead { data: RamReadData },
    IJog,
    SJog,
    Stat,
    Rollback,
    Reboot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum InternalCommand {
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

impl InternalCommand {
    fn inject_payload(self, payload: AssociatedData) -> Command {
        use self::Command::*;
        match (self, payload) {
            (InternalCommand::EEPWrite, AssociatedData::Nothing) => EEPWrite,
            (InternalCommand::RamWrite, AssociatedData::Nothing) => RamWrite,
            (InternalCommand::IJog, AssociatedData::Nothing) => IJog,
            (InternalCommand::SJog, AssociatedData::Nothing) => SJog,
            (InternalCommand::Stat, AssociatedData::Nothing) => Stat,
            (InternalCommand::Rollback, AssociatedData::Nothing) => Rollback,
            (InternalCommand::Reboot, AssociatedData::Nothing) => Reboot,
            (InternalCommand::EEPRead, AssociatedData::EEP(data)) => EEPRead { data },
            (InternalCommand::RamRead, AssociatedData::Ram(data)) => RamRead { data },
            _ => unreachable!(),
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        use reader::Command::*;
        match cmd {
            EEPWrite => 0x41,
            EEPRead { data: _ } => 0x42,
            RamWrite => 0x43,
            RamRead { data: _ } => 0x44,
            IJog => 0x45,
            SJog => 0x46,
            Stat => 0x47,
            Rollback => 0x48,
            Reboot => 0x49,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StatusError {
    ExceedInputVoltageLimit,
    ExceedAllowedPOTLimit,
    ExceedTemperatureLimit,
    InvalidPacket,
    OverloadDetected,
    DriverFaultDetected,
    EEPREGDistorded,
    NoError,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StatusDetail {
    MovingFlag,
    ImpositionFlag,
    ChecksumError,
    UnknownCommand,
    ExceedREGRange,
    GarbageDetected,
    MotorOnFlag,
    NoDetail,
}

#[derive(Copy, Clone, Debug)]
enum AssociatedData {
    EEP(EEPReadData),
    Ram(RamReadData),
    Nothing,
}

pub struct ACKReader {
    state: ReaderState,
    buffer: ArrayVec<[ACKPacket; TRAME_READER_INTERNAL_BUFFER_SIZE]>,
}

// Structure permettant de gérer la machine à états
#[derive(Debug, Clone, Copy)]
enum ReaderState {
    H1,
    H2,
    Psize,
    Pid {
        size: u8,
    },
    Cmd {
        size: u8,
        pid: u8,
    },
    Checksum1 {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
    },
    Checksum2 {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
    },
    DataAddr {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
    },
    DataLenEEP {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        data: EEPReadData,
    },
    Data1EEP {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        data: EEPReadData,
    },
    Data2EEP {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        data: EEPReadData,
    },
    DataLenRAM {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        data: RamReadData,
    },
    Data1RAM {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        data: RamReadData,
    },
    Data2RAM {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        data: RamReadData,
    },
    Error {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        payload: AssociatedData,
    },
    Detail {
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        payload: AssociatedData,
        status_error: StatusError,
    },
}

impl ReaderState {
    fn step(&mut self, byte: u8) -> Option<ACKPacket> {
        use addr::EEPReadData;
        use addr::RamReadData;
        use addr::ReadableEEPAddr;
        use addr::ReadableRamAddr;
        use addr::WritableEEPAddr::*;
        use addr::WritableRamAddr::*;
        use reader::AssociatedData::*;
        use reader::InternalCommand::*;
        use reader::ReaderState::*;
        use reader::StatusDetail::*;
        use reader::StatusError::*;
        use try_from::TryFrom;

        let mut result: Option<ACKPacket> = None;
        match *self {
            H1 => *self = H2,
            H2 => *self = Psize,
            Psize => *self = Pid { size: byte },
            Pid { size } => {
                *self = Cmd {
                    size: size,
                    pid: byte,
                }
            }
            Cmd { size, pid } => {
                let mut command: Option<InternalCommand> = None;
                match byte {
                    0x41 => command = Some(EEPWrite),
                    0x42 => command = Some(EEPRead),
                    0x43 => command = Some(RamWrite),
                    0x44 => command = Some(RamRead),
                    0x45 => command = Some(IJog),
                    0x46 => command = Some(SJog),
                    0x47 => command = Some(Stat),
                    0x48 => command = Some(Rollback),
                    0x49 => command = Some(Reboot),
                    _ => *self = H1,
                }
                if let Some(command) = command {
                    *self = Checksum1 {
                        size: size,
                        pid: pid,
                        cmd: command,
                    }
                }
            }
            Checksum1 { size, pid, cmd } => {
                *self = Checksum2 {
                    size: size,
                    pid: pid,
                    cmd: cmd,
                    chk1: byte,
                }
            }
            Checksum2 {
                size,
                pid,
                cmd,
                chk1,
            } if (cmd == EEPRead || cmd == RamRead) => {
                *self = DataAddr {
                    size: size,
                    pid: pid,
                    cmd: cmd,
                    chk1: chk1,
                    chk2: byte,
                }
            }
            Checksum2 {
                size,
                pid,
                cmd,
                chk1,
            } => {
                *self = Error {
                    size: size,
                    pid: pid,
                    cmd: cmd,
                    chk1: chk1,
                    chk2: byte,
                    payload: Nothing,
                }
            }
            DataAddr {
                size,
                pid,
                cmd,
                chk1,
                chk2,
            } => match cmd {
                EEPRead => {
                    *self = match TryFrom::try_from(byte) {
                        Ok(data_addr) => DataLenEEP {
                            size: size,
                            pid: pid,
                            cmd: cmd,
                            chk1: chk1,
                            chk2: chk2,
                            data: EEPReadData {
                                addr: data_addr,
                                data_len: 0,
                                data: [0, 0],
                            },
                        },
                        Err(_) => H1,
                    }
                }
                RamRead => {
                    *self = match TryFrom::try_from(byte) {
                        Ok(data_addr) => DataLenRAM {
                            size: size,
                            pid: pid,
                            cmd: cmd,
                            chk1: chk1,
                            chk2: chk2,
                            data: RamReadData {
                                addr: data_addr,
                                data_len: 0,
                                data: [0, 0],
                            },
                        },
                        Err(_) => H1,
                    }
                }
                _ => unreachable!(),
            },
            DataLenEEP {
                size,
                pid,
                cmd,
                chk1,
                chk2,
                data,
            } => {
                let new_data = EEPReadData {
                    addr: data.addr,
                    data_len: byte,
                    data: [0, 0],
                };
                *self = Data1EEP {
                    size: size,
                    pid: pid,
                    cmd: cmd,
                    chk1: chk1,
                    chk2: chk2,
                    data: new_data,
                };
            }
            DataLenRAM {
                size,
                pid,
                cmd,
                chk1,
                chk2,
                data,
            } => {
                let new_data = RamReadData {
                    addr: data.addr,
                    data_len: byte,
                    data: [0, 0],
                };
                *self = Data1RAM {
                    size: size,
                    pid: pid,
                    cmd: cmd,
                    chk1: chk1,
                    chk2: chk2,
                    data: new_data,
                }
            }
            Data1EEP {
                size,
                pid,
                cmd: _,
                chk1,
                chk2,
                data,
            } => {
                let new_data = EEPReadData {
                    addr: data.addr,
                    data_len: data.data_len,
                    data: [byte, 0],
                };
                if data.data_len == 2 {
                    *self = Data2EEP {
                        size: size,
                        pid: pid,
                        cmd: InternalCommand::EEPRead,
                        chk1: chk1,
                        chk2: chk2,
                        data: new_data,
                    }
                } else {
                    *self = Error {
                        size: size,
                        pid: pid,
                        cmd: InternalCommand::EEPRead,
                        chk1: chk1,
                        chk2: chk2,
                        payload: AssociatedData::EEP(new_data),
                    }
                }
            }
            Data2EEP {
                size,
                pid,
                cmd,
                chk1,
                chk2,
                data,
            } => {
                let new_data = EEPReadData {
                    addr: data.addr,
                    data_len: data.data_len,
                    data: [data.data[0], byte],
                };
                *self = Error {
                    size: size,
                    pid: pid,
                    cmd: cmd,
                    chk1: chk1,
                    chk2: chk2,
                    payload: AssociatedData::EEP(new_data),
                }
            }
            Data1RAM {
                size,
                pid,
                cmd: _,
                chk1,
                chk2,
                data,
            } => {
                let new_data = RamReadData {
                    addr: data.addr,
                    data_len: data.data_len,
                    data: [byte, 0],
                };
                if data.data_len == 2 {
                    *self = Data2RAM {
                        size: size,
                        pid: pid,
                        cmd: InternalCommand::RamRead,
                        chk1: chk1,
                        chk2: chk2,
                        data: new_data,
                    }
                } else {
                    *self = Error {
                        size: size,
                        pid: pid,
                        cmd: InternalCommand::RamRead,
                        chk1: chk1,
                        chk2: chk2,
                        payload: AssociatedData::Ram(new_data),
                    }
                }
            }
            Data2RAM {
                size,
                pid,
                cmd,
                chk1,
                chk2,
                data,
            } => {
                let new_data = RamReadData {
                    addr: data.addr,
                    data_len: data.data_len,
                    data: [data.data[0], byte],
                };
                *self = Error {
                    size: size,
                    pid: pid,
                    cmd: cmd,
                    chk1: chk1,
                    chk2: chk2,
                    payload: AssociatedData::Ram(new_data),
                }
            }
            Error {
                size,
                pid,
                cmd,
                chk1,
                chk2,
                payload,
            } => match byte {
                0x00 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: NoError,
                    }
                }
                0x01 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: ExceedInputVoltageLimit,
                    }
                }
                0x02 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: ExceedAllowedPOTLimit,
                    }
                }
                0x04 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: ExceedTemperatureLimit,
                    }
                }
                0x08 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: InvalidPacket,
                    }
                }
                0x10 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: OverloadDetected,
                    }
                }
                0x20 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: DriverFaultDetected,
                    }
                }
                0x40 => {
                    *self = Detail {
                        size: size,
                        pid: pid,
                        cmd: cmd,
                        chk1: chk1,
                        chk2: chk2,
                        payload: payload,
                        status_error: EEPREGDistorded,
                    }
                }
                _ => *self = H1,
            },
            Detail {
                size,
                pid,
                cmd,
                chk1,
                chk2,
                payload,
                status_error,
            } => {
                let mut status_detail = None;
                match byte {
                    0x00 => status_detail = Some(NoDetail),
                    0x01 => status_detail = Some(MovingFlag),
                    0x02 => status_detail = Some(ImpositionFlag),
                    0x04 => status_detail = Some(ChecksumError),
                    0x08 => status_detail = Some(UnknownCommand),
                    0x10 => status_detail = Some(ExceedREGRange),
                    0x20 => status_detail = Some(GarbageDetected),
                    0x40 => status_detail = Some(MotorOnFlag),
                    _ => (),
                };
                if let Some(status_detail) = status_detail {
                    result = self.add_to_buffer(
                        size,
                        pid,
                        cmd,
                        chk1,
                        chk2,
                        payload,
                        status_error,
                        status_detail,
                    );
                }
                *self = H1;
            }
        };
        result
    }

    fn add_to_buffer(
        &mut self,
        size: u8,
        pid: u8,
        cmd: InternalCommand,
        chk1: u8,
        chk2: u8,
        payload: AssociatedData,
        status_error: StatusError,
        status_detail: StatusDetail,
    ) -> Option<ACKPacket> {
        let cmd = cmd.inject_payload(payload);
        let packet = ACKPacket {
            psize: size,
            pid: pid,
            cmd: cmd,
            chk1: chk1,
            chk2: chk2,
            error: status_error,
            detail: status_detail,
        };
        if packet.is_valid() {
            Some(packet)
        } else {
            None
        }
    }
}

impl ACKReader {
    /// Creates a new state machine to read incoming Herkulex messages
    pub fn new() -> ACKReader {
        ACKReader {
            state: ReaderState::H1,
            buffer: ArrayVec::new(),
        }
    }

    /// Return the oldest [ACKPacket] read
    pub fn pop_ack_packet(&mut self) -> Option<ACKPacket> {
        self.buffer.pop()
    }

    /// Get the number of available messages in the internal buffer
    pub fn available_messages(&mut self) -> usize {
        self.buffer.len()
    }

    /// Parse a buffer of bytes, adding sucessfully decoded  messages to the internal buffer
    pub fn parse(&mut self, buf: &[u8]) {
        for byte in buf {
            if let Some(trame) = self.state.step(*byte) {
                self.buffer.push(trame);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use addr::*;
    use reader::{ACKPacket, ACKReader, AssociatedData, Command, StatusDetail, StatusError};

    //#[test]
    fn test_eepread() {
        let mut reader = ACKReader::new();

        // Test de EEPRead
        // [H1][H2][psize][pid][cmd][chk1][chk2][data_addr][data_len][data][data][status_error][status_detail]
        let packet_eepread = [
            0xFF, 0xFF, 0x0F, 0xFD, 0x42, 0x14, 0xEA, 0x1E, 0x02, 0xB8, 0x01, 0x08, 0x20,
        ];

        reader.parse(&packet_eepread);

        let data_eepread = EEPReadData {
            addr: ReadableEEPAddr::PositionKp,
            data_len: 2,
            data: [0xB8, 0x01],
        };

        assert_eq!(
            reader.pop_ack_packet().unwrap(),
            ACKPacket {
                psize: 0x0F,
                pid: 0xFD,
                cmd: Command::EEPRead { data: data_eepread },
                chk1: 0x14,
                chk2: 0xEA,
                error: StatusError::InvalidPacket,
                detail: StatusDetail::GarbageDetected,
            }
        );
    }

    //#[test]
    fn test_ramread() {
        let mut reader = ACKReader::new();

        // Test de RamRead
        // [H1][H2][psize][pid][cmd][chk1][chk2][data_addr][data_len][data][status_error][status_detail]
        let packet_ramread = [
            0xFF, 0xFF, 0x0C, 0xFD, 0x44, 0xA0, 0x5E, 0x14, 0x01, 0x01, 0x10, 0x40,
        ];

        reader.parse(&packet_ramread);

        let data_ramread = RamReadData {
            addr: ReadableRamAddr::MinPosition, // 20 (0x14)
            data_len: 1,
            data: [0x01, 0x00],
        };

        assert_eq!(
            reader.pop_ack_packet().unwrap(),
            ACKPacket {
                psize: 0x0C,
                pid: 0xFD,
                cmd: Command::RamRead { data: data_ramread },
                chk1: 0xA0,
                chk2: 0x5E,
                error: StatusError::OverloadDetected,
                detail: StatusDetail::MotorOnFlag,
            }
        );
    }

    #[test]
    fn test_sjog() {
        let mut reader = ACKReader::new();

        // Test de SJOG
        // [H1][H2][psize][pid][cmd][chk1][chk2][status_error][status_detail]
        let packet_sjog = [
            0xFF, 0xFF, 0x09, 0xFD, 0x46, /*0xF2*/ 0xB2, /*0x0C*/ 0x4C, 0x08, 0x08,
        ];

        reader.parse(&packet_sjog);

        assert_eq!(
            reader.pop_ack_packet().unwrap(),
            ACKPacket {
                psize: 0x09,
                pid: 0xFD,
                cmd: Command::SJog,
                chk1: /*0xF2*/ 0xB2,
                chk2: /*0x0C*/ 0x4C,
                error: StatusError::InvalidPacket,
                detail: StatusDetail::UnknownCommand,
            }
        );
    }
}
