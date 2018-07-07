use addr::ReadableEEPAddr;
use addr::ReadableRamAddr;
use addr::WritableEEPAddr;
use addr::WritableRamAddr;

pub enum RegisterRequest {
    EEPWrite(WritableEEPAddr),
    EEPRead(ReadableEEPAddr),
    RamWrite(WritableRamAddr),
    RamRead(ReadableRamAddr),
}

pub enum PositionRequest {
    IJog,
    SJog,
}
pub enum SpecialRequest {
    Stat,
    Rollback { skip_id: u8, skip_baud: u8 },
    Reboot,
}

pub enum Rollback {
    SkipId,
    SkipBaud,
    SkipBoth,
    SkipNone,
}
