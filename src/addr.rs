//! All the servomotor addresses mapped to some enums.

extern crate try_from;
use self::try_from::TryFrom;

/// This enum represent all the RAM (volatile) memory adresses which can be read. I comes from the
/// page 24 of the datasheet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadableRamAddr {
    /// Servo ID
    ID,
    /// TODO : Refer to pg 33
    AckPolicy,
    /// Activates LED according to Policy
    AlarmLEDPolicy,
    /// Releases Torque accroding to Policy
    TorquePolicy,
    /// Maximum allowed temperature (0xDF = 85°C)
    MaxTemperature,
    /// Minimum allowed voltage (0x5B = 6.714 VDC)
    MinVoltage,
    /// Maximum allowed voltage (0x89 = 10 VDC)
    MaxVoltage,
    /// Ratio of time to reach goal position to acceleration or deceleration
    AccelerationRatio,
    /// Max acceleration time, 11.2ms interval. Acceleration(0x2D : 504 ms)
    MaxAcceleration,
    /// Outside controle range
    DeadZone,
    /// TODO : Refer to datasheet page 36
    SaturatorOffset,
    /// TODO : Refer to datasheet page 36
    SaturatorSlope,
    /// PWM Offset value, refer to datasheet page 37
    PWMOffset,
    /// Set minimum PWM value, refer to the datasheet page 37
    MinPWM,
    /// Set maximum PWM value, refer to the datasheet page 37
    MaxPWM,
    /// Set PWM Overload thershold range, refer to the datasheet page 34
    OverloadPWMThreshold,
    /// Minimum position value (between 0 and 1023)
    MinPosition,
    /// Maximum position value (between 0 and 1023)
    MaxPosition,
    /// Proportional gain
    PositionKp,
    /// Derivative gain
    PositionKd,
    /// Integral gain
    PositionKi,
    /// Refer to the datasheet page 35
    PositionFFFirstGain,
    /// Refer to the datasheet page 35
    PositionFFSecondGain,
    /// Alarm LED blink period according to Policy 11.2ms/Tick (0x2D : 504 ms)
    LedBlinkPeriod,
    /// Temp/Voltage error check interval. 11.2ms/tick (0x2D : 504 ms)
    ADCFaultDetectionPeriod,
    /// Packet error check interval. 11.2ms/tick (0x12 : 201 ms)
    PacketGarbageDetectionPeriod,
    /// Stop detection check interval. 11.2ms/tick (0x1B : 302 ms)
    StopDetectionPeriod,
    /// Overload check interbal. 11.2ms/tick (0x96 : 1.68 s)
    OverloadDetectionPeriod,
    /// Stop Threshold
    StopThreshold,
    /// Offset Threshold
    InpositionMargin,
    /// Servo compensation
    CalibrationDifference,
    /// Refer to datasheet page 39
    StatusError,
    /// Refer to datasheet page 39
    StatusDetail,
    /// Torque enable states (refer to datasheet page 28)
    TorqueControl,
    /// 0x01 : Green, 0x02 : Blue, 0x04 : Red
    LEDControl,
    /// Input voltage raw data 8bit (refer to datasheet page 31)
    Voltage,
    /// Current temperature data 8bit (refer to datasheet page 31)
    Temperature,
    /// 0 : Position control
    /// 1 : Turn/Velocity control
    CurrentControlMode,
    /// 11.2ms/tick
    Tick,
    /// Calibrated current position raw data, 10 bit.
    CalibratedPosition,
    /// Uncalibrated absolute position raw data.
    AbsolutePosition,
    /// Position change/11.2ms
    DifferentialPosition,
    /// Torque raw data
    PWM,
    /// Uncalibrated goal position raw data
    AbsoluteGoalPosition,
    /// Current intermediate goal position in trajectory
    AbsoluteDesiredTrajectoryPosition,
    /// Desired speed based on speed profile raw data
    DesiredVelocity,
}

impl ReadableRamAddr {
    /// Return the size in bytes of the value stocked at this address
    pub fn bytes(&self) -> u8 {
        match *self {
            ReadableRamAddr::ID => 1,
            ReadableRamAddr::AckPolicy => 1,
            ReadableRamAddr::AlarmLEDPolicy => 1,
            ReadableRamAddr::TorquePolicy => 1,
            ReadableRamAddr::MaxTemperature => 1,
            ReadableRamAddr::MinVoltage => 1,
            ReadableRamAddr::MaxVoltage => 1,
            ReadableRamAddr::AccelerationRatio => 1,
            ReadableRamAddr::MaxAcceleration => 1,
            ReadableRamAddr::DeadZone => 1,
            ReadableRamAddr::SaturatorOffset => 1,
            ReadableRamAddr::SaturatorSlope => 2,
            ReadableRamAddr::PWMOffset => 1,
            ReadableRamAddr::MinPWM => 1,
            ReadableRamAddr::MaxPWM => 2,
            ReadableRamAddr::OverloadPWMThreshold => 2,
            ReadableRamAddr::MinPosition => 2,
            ReadableRamAddr::MaxPosition => 2,
            ReadableRamAddr::PositionKp => 2,
            ReadableRamAddr::PositionKd => 2,
            ReadableRamAddr::PositionKi => 2,
            ReadableRamAddr::PositionFFFirstGain => 2,
            ReadableRamAddr::PositionFFSecondGain => 2,
            ReadableRamAddr::LedBlinkPeriod => 1,
            ReadableRamAddr::ADCFaultDetectionPeriod => 1,
            ReadableRamAddr::PacketGarbageDetectionPeriod => 1,
            ReadableRamAddr::StopDetectionPeriod => 1,
            ReadableRamAddr::OverloadDetectionPeriod => 1,
            ReadableRamAddr::StopThreshold => 1,
            ReadableRamAddr::InpositionMargin => 1,
            ReadableRamAddr::CalibrationDifference => 1,
            ReadableRamAddr::StatusError => 1,
            ReadableRamAddr::StatusDetail => 1,
            ReadableRamAddr::TorqueControl => 1,
            ReadableRamAddr::LEDControl => 1,
            ReadableRamAddr::Voltage => 2,
            ReadableRamAddr::Temperature => 2,
            ReadableRamAddr::CurrentControlMode => 2,
            ReadableRamAddr::Tick => 2,
            ReadableRamAddr::CalibratedPosition => 2,
            ReadableRamAddr::AbsolutePosition => 2,
            ReadableRamAddr::DifferentialPosition => 2,
            ReadableRamAddr::PWM => 2,
            ReadableRamAddr::AbsoluteGoalPosition => 2,
            ReadableRamAddr::AbsoluteDesiredTrajectoryPosition => 2,
            ReadableRamAddr::DesiredVelocity => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RamReadData {
    pub addr : ReadableRamAddr,
    pub data_len : u8,
    pub data : [u8;2],
}

/// This enum represent all the RAM (volatile) memory addresses which can be written to. I comes
/// from the page 24 of the
/// datasheet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WritableRamAddr {
    /// Servo ID
    ID(u8),
    /// TODO : Refer to pg 33
    AckPolicy(u8),
    /// Activates LED according to Policy
    AlarmLEDPolicy(u8),
    /// Releases Torque accroding to Policy
    TorquePolicy(u8),
    /// Maximum allowed temperature (0xDF = 85°C)
    MaxTemperature(u8),
    /// Minimum allowed voltage (0x5B = 6.714 VDC)
    MinVoltage(u8),
    /// Maximum allowed voltage (0x89 = 10 VDC)
    MaxVoltage(u8),
    /// Ratio of time to reach goal position to acceleration or deceleration
    AccelerationRatio(u8),
    /// Max acceleration time, 11.2ms interval. Acceleration(0x2D : 504 ms)
    MaxAcceleration(u8),
    /// Outside controle range
    DeadZone(u8),
    /// TODO : Refer to datasheet page 36
    SaturatorOffset(u8),
    /// TODO : Refer to datasheet page 36
    SaturatorSlope(u8, u8),
    /// PWM Offset value, refer to datasheet page 37
    PWMOffset(u8),
    /// Set minimum PWM value, refer to the datasheet page 37
    MinPWM(u8),
    /// Set maximum PWM value, refer to the datasheet page 37
    MaxPWM(u8, u8),
    /// Set PWM Overload thershold range, refer to the datasheet page 34
    OverloadPWMThreshold(u8, u8),
    /// Minimum position value (between 0 and 1023)
    MinPosition(u8, u8),
    /// Maximum position value (between 0 and 1023)
    MaxPosition(u8, u8),
    /// Proportional gain
    PositionKp(u8, u8),
    /// Derivative gain
    PositionKd(u8, u8),
    /// Integral gain
    PositionKi(u8, u8),
    /// Refer to the datasheet page 35
    PositionFFFirstGain(u8, u8),
    /// Refer to the datasheet page 35
    PositionFFSecondGain(u8, u8),
    /// Alarm LED blink period according to Policy 11.2ms/Tick (0x2D : 504 ms)
    LedBlinkPeriod(u8),
    /// Temp/Voltage error check interval. 11.2ms/tick (0x2D : 504 ms)
    ADCFaultDetectionPeriod(u8),
    /// Packet error check interval. 11.2ms/tick (0x12 : 201 ms)
    PacketGarbageDetectionPeriod(u8),
    /// Stop detection check interval. 11.2ms/tick (0x1B : 302 ms)
    StopDetectionPeriod(u8),
    /// Overload check interbal. 11.2ms/tick (0x96 : 1.68 s)
    OverloadDetectionPeriod(u8),
    /// Stop Threshold
    StopThreshold(u8),
    /// Offset Threshold
    InpositionMargin(u8),
    /// Servo compensation
    CalibrationDifference(u8),
    /// Refer to datasheet page 39
    StatusError(u8),
    /// Refer to datasheet page 39
    StatusDetail(u8),
    /// Torque enable states (refer to datasheet page 28)
    TorqueControl(u8),
    /// 0x01 : Green, 0x02 : Blue, 0x04 : Red
    LEDControl(u8),
}

impl WritableRamAddr {
    /// Return the size in bytes of the value stocked at this address
    pub fn bytes(&self) -> u8 {
        match *self {
            WritableRamAddr::ID(_) => 1,
            WritableRamAddr::AckPolicy(_) => 1,
            WritableRamAddr::AlarmLEDPolicy(_) => 1,
            WritableRamAddr::TorquePolicy(_) => 1,
            WritableRamAddr::MaxTemperature(_) => 1,
            WritableRamAddr::MinVoltage(_) => 1,
            WritableRamAddr::MaxVoltage(_) => 1,
            WritableRamAddr::AccelerationRatio(_) => 1,
            WritableRamAddr::MaxAcceleration(_) => 1,
            WritableRamAddr::DeadZone(_) => 1,
            WritableRamAddr::SaturatorOffset(_) => 1,
            WritableRamAddr::SaturatorSlope(_, _) => 2,
            WritableRamAddr::PWMOffset(_) => 1,
            WritableRamAddr::MinPWM(_) => 1,
            WritableRamAddr::MaxPWM(_, _) => 2,
            WritableRamAddr::OverloadPWMThreshold(_, _) => 2,
            WritableRamAddr::MinPosition(_, _) => 2,
            WritableRamAddr::MaxPosition(_, _) => 2,
            WritableRamAddr::PositionKp(_, _) => 2,
            WritableRamAddr::PositionKd(_, _) => 2,
            WritableRamAddr::PositionKi(_, _) => 2,
            WritableRamAddr::PositionFFFirstGain(_, _) => 2,
            WritableRamAddr::PositionFFSecondGain(_, _) => 2,
            WritableRamAddr::LedBlinkPeriod(_) => 1,
            WritableRamAddr::ADCFaultDetectionPeriod(_) => 1,
            WritableRamAddr::PacketGarbageDetectionPeriod(_) => 1,
            WritableRamAddr::StopDetectionPeriod(_) => 1,
            WritableRamAddr::OverloadDetectionPeriod(_) => 1,
            WritableRamAddr::StopThreshold(_) => 1,
            WritableRamAddr::InpositionMargin(_) => 1,
            WritableRamAddr::CalibrationDifference(_) => 1,
            WritableRamAddr::StatusError(_) => 1,
            WritableRamAddr::StatusDetail(_) => 1,
            WritableRamAddr::TorqueControl(_) => 1,
            WritableRamAddr::LEDControl(_) => 1,
        }
    }

    pub(crate) fn associated_data(self) -> (u8, Option<u8>) {
        match self {
            WritableRamAddr::ID(d) => (d, None),
            WritableRamAddr::AckPolicy(d) => (d, None),
            WritableRamAddr::AlarmLEDPolicy(d) => (d, None),
            WritableRamAddr::TorquePolicy(d) => (d, None),
            WritableRamAddr::MaxTemperature(d) => (d, None),
            WritableRamAddr::MinVoltage(d) => (d, None),
            WritableRamAddr::MaxVoltage(d) => (d, None),
            WritableRamAddr::AccelerationRatio(d) => (d, None),
            WritableRamAddr::MaxAcceleration(d) => (d, None),
            WritableRamAddr::DeadZone(d) => (d, None),
            WritableRamAddr::SaturatorOffset(d) => (d, None),
            WritableRamAddr::SaturatorSlope(d, d2) => (d, Some(d2)),
            WritableRamAddr::PWMOffset(d) => (d, None),
            WritableRamAddr::MinPWM(d) => (d, None),
            WritableRamAddr::MaxPWM(d, d2) => (d, Some(d2)),
            WritableRamAddr::OverloadPWMThreshold(d, d2) => (d, Some(d2)),
            WritableRamAddr::MinPosition(d, d2) => (d, Some(d2)),
            WritableRamAddr::MaxPosition(d, d2) => (d, Some(d2)),
            WritableRamAddr::PositionKp(d, d2) => (d, Some(d2)),
            WritableRamAddr::PositionKd(d, d2) => (d, Some(d2)),
            WritableRamAddr::PositionKi(d, d2) => (d, Some(d2)),
            WritableRamAddr::PositionFFFirstGain(d, d2) => (d, Some(d2)),
            WritableRamAddr::PositionFFSecondGain(d, d2) => (d, Some(d2)),
            WritableRamAddr::LedBlinkPeriod(d) => (d, None),
            WritableRamAddr::ADCFaultDetectionPeriod(d) => (d, None),
            WritableRamAddr::PacketGarbageDetectionPeriod(d) => (d, None),
            WritableRamAddr::StopDetectionPeriod(d) => (d, None),
            WritableRamAddr::OverloadDetectionPeriod(d) => (d, None),
            WritableRamAddr::StopThreshold(d) => (d, None),
            WritableRamAddr::InpositionMargin(d) => (d, None),
            WritableRamAddr::CalibrationDifference(d) => (d, None),
            WritableRamAddr::StatusError(d) => (d, None),
            WritableRamAddr::StatusDetail(d) => (d, None),
            WritableRamAddr::TorqueControl(d) => (d, None),
            WritableRamAddr::LEDControl(d) => (d, None),
        }
    }
}

impl From<ReadableRamAddr> for u8 {
    fn from(addr: ReadableRamAddr) -> Self {
        use addr::ReadableRamAddr::*;
        match addr {
            ID => 0,
            AckPolicy => 1,
            AlarmLEDPolicy => 2,
            TorquePolicy => 3,
            MaxTemperature => 5,
            MinVoltage => 6,
            MaxVoltage => 7,
            AccelerationRatio => 8,
            MaxAcceleration => 9,
            DeadZone => 10,
            SaturatorOffset => 11,
            SaturatorSlope => 12,
            PWMOffset => 14,
            MinPWM => 15,
            MaxPWM => 16,
            OverloadPWMThreshold => 18,
            MinPosition => 20,
            MaxPosition => 22,
            PositionKp => 24,
            PositionKd => 26,
            PositionKi => 28,
            PositionFFFirstGain => 30,
            PositionFFSecondGain => 32,
            LedBlinkPeriod => 38,
            ADCFaultDetectionPeriod => 39,
            PacketGarbageDetectionPeriod => 40,
            StopDetectionPeriod => 41,
            OverloadDetectionPeriod => 42,
            StopThreshold => 43,
            InpositionMargin => 44,
            CalibrationDifference => 47,
            StatusError => 48,
            StatusDetail => 49,
            TorqueControl => 52,
            LEDControl => 53,
            Voltage => 54,
            Temperature => 55,
            CurrentControlMode => 56,
            Tick => 57,
            CalibratedPosition => 58,
            AbsolutePosition => 60,
            DifferentialPosition => 62,
            PWM => 64,
            AbsoluteGoalPosition => 68,
            AbsoluteDesiredTrajectoryPosition => 70,
            DesiredVelocity => 72,
        }
    }
}

impl TryFrom<u8> for ReadableRamAddr {
    type Err = Error;
    fn try_from(addr: u8) -> Result<ReadableRamAddr, Error> {
        match addr {
            0 => Ok(ReadableRamAddr::ID),
            1 => Ok(ReadableRamAddr::AckPolicy),
            2 => Ok(ReadableRamAddr::AlarmLEDPolicy),
            3 => Ok(ReadableRamAddr::TorquePolicy),
            5 => Ok(ReadableRamAddr::MaxTemperature),
            6 => Ok(ReadableRamAddr::MinVoltage),
            7 => Ok(ReadableRamAddr::MaxVoltage),
            8 => Ok(ReadableRamAddr::AccelerationRatio),
            9 => Ok(ReadableRamAddr::MaxAcceleration),
            10 => Ok(ReadableRamAddr::DeadZone),
            11 => Ok(ReadableRamAddr::SaturatorOffset),
            12 => Ok(ReadableRamAddr::SaturatorSlope),
            14 => Ok(ReadableRamAddr::PWMOffset),
            15 => Ok(ReadableRamAddr::MinPWM),
            16 => Ok(ReadableRamAddr::MaxPWM),
            18 => Ok(ReadableRamAddr::OverloadPWMThreshold),
            20 => Ok(ReadableRamAddr::MinPosition),
            22 => Ok(ReadableRamAddr::MaxPosition),
            24 => Ok(ReadableRamAddr::PositionKp),
            26 => Ok(ReadableRamAddr::PositionKd),
            28 => Ok(ReadableRamAddr::PositionKi),
            30 => Ok(ReadableRamAddr::PositionFFFirstGain),
            32 => Ok(ReadableRamAddr::PositionFFSecondGain),
            38 => Ok(ReadableRamAddr::LedBlinkPeriod),
            39 => Ok(ReadableRamAddr::ADCFaultDetectionPeriod),
            40 => Ok(ReadableRamAddr::PacketGarbageDetectionPeriod),
            41 => Ok(ReadableRamAddr::StopDetectionPeriod),
            42 => Ok(ReadableRamAddr::OverloadDetectionPeriod),
            43 => Ok(ReadableRamAddr::StopThreshold),
            44 => Ok(ReadableRamAddr::InpositionMargin),
            47 => Ok(ReadableRamAddr::CalibrationDifference),
            48 => Ok(ReadableRamAddr::StatusError),
            49 => Ok(ReadableRamAddr::StatusDetail),
            52 => Ok(ReadableRamAddr::TorqueControl),
            53 => Ok(ReadableRamAddr::LEDControl),
            54 => Ok(ReadableRamAddr::Voltage),
            55 => Ok(ReadableRamAddr::Temperature),
            56 => Ok(ReadableRamAddr::CurrentControlMode),
            57 => Ok(ReadableRamAddr::Tick),
            58 => Ok(ReadableRamAddr::CalibratedPosition),
            60 => Ok(ReadableRamAddr::AbsolutePosition),
            62 => Ok(ReadableRamAddr::DifferentialPosition),
            64 => Ok(ReadableRamAddr::PWM),
            68 => Ok(ReadableRamAddr::AbsoluteGoalPosition),
            70 => Ok(ReadableRamAddr::AbsoluteDesiredTrajectoryPosition),
            72 => Ok(ReadableRamAddr::DesiredVelocity),
        }
    }
}

impl From<WritableRamAddr> for u8 {
    fn from(addr: WritableRamAddr) -> Self {
        use addr::WritableRamAddr::*;
        match addr {
            ID(_) => 0,
            AckPolicy(_) => 1,
            AlarmLEDPolicy(_) => 2,
            TorquePolicy(_) => 3,
            MaxTemperature(_) => 5,
            MinVoltage(_) => 6,
            MaxVoltage(_) => 7,
            AccelerationRatio(_) => 8,
            MaxAcceleration(_) => 9,
            DeadZone(_) => 10,
            SaturatorOffset(_) => 11,
            SaturatorSlope(_, _) => 12,
            PWMOffset(_) => 14,
            MinPWM(_) => 15,
            MaxPWM(_, _) => 16,
            OverloadPWMThreshold(_, _) => 18,
            MinPosition(_, _) => 20,
            MaxPosition(_, _) => 22,
            PositionKp(_, _) => 24,
            PositionKd(_, _) => 26,
            PositionKi(_, _) => 28,
            PositionFFFirstGain(_, _) => 30,
            PositionFFSecondGain(_, _) => 32,
            LedBlinkPeriod(_) => 38,
            ADCFaultDetectionPeriod(_) => 39,
            PacketGarbageDetectionPeriod(_) => 40,
            StopDetectionPeriod(_) => 41,
            OverloadDetectionPeriod(_) => 42,
            StopThreshold(_) => 43,
            InpositionMargin(_) => 44,
            CalibrationDifference(_) => 47,
            StatusError(_) => 48,
            StatusDetail(_) => 49,
            TorqueControl(_) => 52,
            LEDControl(_) => 53,
        }
    }
}

impl TryFrom<u8> for WritableRamAddr {
    type Err = Error;
    fn try_from(addr: u8) -> Result<WritableRamAddr, Error> {
        match addr {
            0 => Ok(WritableRamAddr::ID(0)),
            1 => Ok(WritableRamAddr::AckPolicy(0)),
            2 => Ok(WritableRamAddr::AlarmLEDPolicy(0)),
            3 => Ok(WritableRamAddr::TorquePolicy(0)),
            5 => Ok(WritableRamAddr::MaxTemperature(0)),
            6 => Ok(WritableRamAddr::MinVoltage(0)),
            7 => Ok(WritableRamAddr::MaxVoltage(0)),
            8 => Ok(WritableRamAddr::AccelerationRatio(0)),
            9 => Ok(WritableRamAddr::MaxAcceleration(0)),
            10 => Ok(WritableRamAddr::DeadZone(0)),
            11 => Ok(WritableRamAddr::SaturatorOffset(0)),
            12 => Ok(WritableRamAddr::SaturatorSlope(0, 0)),
            14 => Ok(WritableRamAddr::PWMOffset(0)),
            15 => Ok(WritableRamAddr::MinPWM(0)),
            16 => Ok(WritableRamAddr::MaxPWM(0, 0)),
            18 => Ok(WritableRamAddr::OverloadPWMThreshold(0, 0)),
            20 => Ok(WritableRamAddr::MinPosition(0, 0)),
            22 => Ok(WritableRamAddr::MaxPosition(0, 0)),
            24 => Ok(WritableRamAddr::PositionKp(0, 0)),
            26 => Ok(WritableRamAddr::PositionKd(0, 0)),
            28 => Ok(WritableRamAddr::PositionKi(0, 0)),
            30 => Ok(WritableRamAddr::PositionFFFirstGain(0, 0)),
            32 => Ok(WritableRamAddr::PositionFFSecondGain(0, 0)),
            38 => Ok(WritableRamAddr::LedBlinkPeriod(0)),
            39 => Ok(WritableRamAddr::ADCFaultDetectionPeriod(0)),
            40 => Ok(WritableRamAddr::PacketGarbageDetectionPeriod(0)),
            41 => Ok(WritableRamAddr::StopDetectionPeriod(0)),
            42 => Ok(WritableRamAddr::OverloadDetectionPeriod(0)),
            43 => Ok(WritableRamAddr::StopThreshold(0)),
            44 => Ok(WritableRamAddr::InpositionMargin(0)),
            47 => Ok(WritableRamAddr::CalibrationDifference(0)),
            48 => Ok(WritableRamAddr::StatusError(0)),
            49 => Ok(WritableRamAddr::StatusDetail(0)),
            52 => Ok(WritableRamAddr::TorqueControl(0)),
            53 => Ok(WritableRamAddr::LEDControl(0)),
        }
    }
}

/// This enum represent all the EPP (permanent) memory addresses which can be read. I comes from
/// the page 21 of the
/// datasheet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadableEEPAddr {
    /// DRS model number first byte
    ModelNo1,
    /// DRS model number second byte
    ModelNo2,
    /// Firmware version first byte
    Version1,
    /// Firmware version second byte
    Version2,
    /// Communication speed
    BaudRate,
    /// Servo ID
    ID,
    /// TODO : Refer to pg 33
    AckPolicy,
    /// Activates LED according to Policy
    AlarmLEDPolicy,
    /// Releases Torque accroding to Policy
    TorquePolicy,
    /// Maximum allowed temperature (0xDF = 85°C)
    MaxTemperature,
    /// Minimum allowed voltage (0x5B = 6.714 VDC)
    MinVoltage,
    /// Maximum allowed voltage (0x89 = 10 VDC)
    MaxVoltage,
    /// Ratio of time to reach goal position to acceleration or deceleration
    AccelerationRatio,
    /// Max acceleration time, 11.2ms interval. Acceleration(0x2D : 504 ms)
    MaxAccelerationTime,
    /// Outside controle range
    DeadZone,
    /// TODO : Refer to datasheet page 36
    SaturatorOffset,
    /// TODO : Refer to datasheet page 36
    SaturatorSlope,
    /// PWM Offset value, refer to datasheet page 37
    PWMOffset,
    /// Set minimum PWM value, refer to the datasheet page 37
    MinPWM,
    /// Set maximum PWM value, refer to the datasheet page 37
    MaxPWM,
    /// Set PWM Overload thershold range, refer to the datasheet page 34
    OverloadPWMThreshold,
    /// Minimum position value (between 0 and 1023)
    MinPosition,
    /// Maximum position value (between 0 and 1023)
    MaxPosition,
    /// Proportional gain
    PositionKp,
    /// Derivative gain
    PositionKd,
    /// Integral gain
    PositionKi,
    /// Refer to the datasheet page 35
    PositionFFFirstGain,
    /// Refer to the datasheet page 35
    PositionFFSecondGain,
    /// Alarm LED blink period according to Policy 11.2ms/Tick (0x2D : 504 ms)
    LedBlinkPeriod,
    /// Temp/Voltage error check interval. 11.2ms/tick (0x2D : 504 ms)
    ADCFaultCheckPeriod,
    /// Packet error check interval. 11.2ms/tick (0x12 : 201 ms)
    PacketGarbageDetectionPeriod,
    /// Stop detection check interval. 11.2ms/tick (0x1B : 302 ms)
    StopDetectionPeriod,
    /// Overload check interbal. 11.2ms/tick (0x96 : 1.68 s)
    OverloadDetectionPeriod,
    /// Stop Threshold
    StopThreshold,
    /// Offset Threshold
    InpositionMargin,
    /// Servo compensation
    CalibrationDifference,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EEPReadData {
    pub addr : ReadableEEPAddr,
    pub data_len : u8,
    pub data : [u8;2],
}

impl ReadableEEPAddr {
    /// Return the number of bytes associated with an address
    pub fn bytes(&self) -> u8 {
        match *self {
            ReadableEEPAddr::ModelNo1 => 1,
            ReadableEEPAddr::ModelNo2 => 1,
            ReadableEEPAddr::Version1 => 1,
            ReadableEEPAddr::Version2 => 1,
            ReadableEEPAddr::BaudRate => 1,
            ReadableEEPAddr::ID => 1,
            ReadableEEPAddr::AckPolicy => 1,
            ReadableEEPAddr::AlarmLEDPolicy => 1,
            ReadableEEPAddr::TorquePolicy => 1,
            ReadableEEPAddr::MaxTemperature => 1,
            ReadableEEPAddr::MinVoltage => 1,
            ReadableEEPAddr::MaxVoltage => 1,
            ReadableEEPAddr::AccelerationRatio => 1,
            ReadableEEPAddr::MaxAccelerationTime => 1,
            ReadableEEPAddr::DeadZone => 1,
            ReadableEEPAddr::SaturatorOffset => 1,
            ReadableEEPAddr::SaturatorSlope => 2,
            ReadableEEPAddr::PWMOffset => 1,
            ReadableEEPAddr::MinPWM => 1,
            ReadableEEPAddr::MaxPWM => 2,
            ReadableEEPAddr::OverloadPWMThreshold => 2,
            ReadableEEPAddr::MinPosition => 2,
            ReadableEEPAddr::MaxPosition => 2,
            ReadableEEPAddr::PositionKp => 2,
            ReadableEEPAddr::PositionKd => 2,
            ReadableEEPAddr::PositionKi => 2,
            ReadableEEPAddr::PositionFFFirstGain => 2,
            ReadableEEPAddr::PositionFFSecondGain => 2,
            ReadableEEPAddr::LedBlinkPeriod => 1,
            ReadableEEPAddr::ADCFaultCheckPeriod => 1,
            ReadableEEPAddr::PacketGarbageDetectionPeriod => 1,
            ReadableEEPAddr::StopDetectionPeriod => 1,
            ReadableEEPAddr::OverloadDetectionPeriod => 1,
            ReadableEEPAddr::StopThreshold => 1,
            ReadableEEPAddr::InpositionMargin => 1,
            ReadableEEPAddr::CalibrationDifference => 1,
        }
    }
}

impl From<ReadableEEPAddr> for u8 {
    fn from(me: ReadableEEPAddr) -> Self {
        match me {
            ReadableEEPAddr::ModelNo1 => 0,
            ReadableEEPAddr::ModelNo2 => 1,
            ReadableEEPAddr::Version1 => 2,
            ReadableEEPAddr::Version2 => 3,
            ReadableEEPAddr::BaudRate => 4,
            ReadableEEPAddr::ID => 6,
            ReadableEEPAddr::AckPolicy => 7,
            ReadableEEPAddr::AlarmLEDPolicy => 8,
            ReadableEEPAddr::TorquePolicy => 9,
            ReadableEEPAddr::MaxTemperature => 11,
            ReadableEEPAddr::MinVoltage => 12,
            ReadableEEPAddr::MaxVoltage => 13,
            ReadableEEPAddr::AccelerationRatio => 14,
            ReadableEEPAddr::MaxAccelerationTime => 15,
            ReadableEEPAddr::DeadZone => 16,
            ReadableEEPAddr::SaturatorOffset => 17,
            ReadableEEPAddr::SaturatorSlope => 18,
            ReadableEEPAddr::PWMOffset => 20,
            ReadableEEPAddr::MinPWM => 21,
            ReadableEEPAddr::MaxPWM => 22,
            ReadableEEPAddr::OverloadPWMThreshold => 24,
            ReadableEEPAddr::MinPosition => 26,
            ReadableEEPAddr::MaxPosition => 28,
            ReadableEEPAddr::PositionKp => 30,
            ReadableEEPAddr::PositionKd => 32,
            ReadableEEPAddr::PositionKi => 34,
            ReadableEEPAddr::PositionFFFirstGain => 36,
            ReadableEEPAddr::PositionFFSecondGain => 38,
            ReadableEEPAddr::LedBlinkPeriod => 44,
            ReadableEEPAddr::ADCFaultCheckPeriod => 45,
            ReadableEEPAddr::PacketGarbageDetectionPeriod => 46,
            ReadableEEPAddr::StopDetectionPeriod => 47,
            ReadableEEPAddr::OverloadDetectionPeriod => 48,
            ReadableEEPAddr::StopThreshold => 49,
            ReadableEEPAddr::InpositionMargin => 50,
            ReadableEEPAddr::CalibrationDifference => 53,
        }
    }
}

impl TryFrom<u8> for ReadableEEPAddr {
    type Err = Error;
    fn try_from(addr: u8) -> Result<ReadableEEPAddr, Error> {
        match addr {
            0 => Ok(ReadableEEPAddr::ModelNo1),
            1 => Ok(ReadableEEPAddr::ModelNo2),
            2 => Ok(ReadableEEPAddr::Version1),
            3 => Ok(ReadableEEPAddr::Version2),
            4 => Ok(ReadableEEPAddr::BaudRate),
            6 => Ok(ReadableEEPAddr::ID),
            7 => Ok(ReadableEEPAddr::AckPolicy),
            8 => Ok(ReadableEEPAddr::AlarmLEDPolicy),
            9 => Ok(ReadableEEPAddr::TorquePolicy),
            11 => Ok(ReadableEEPAddr::MaxTemperature),
            12 => Ok(ReadableEEPAddr::MinVoltage),
            13 => Ok(ReadableEEPAddr::MaxVoltage),
            14 => Ok(ReadableEEPAddr::AccelerationRatio),
            15 => Ok(ReadableEEPAddr::MaxAccelerationTime),
            16 => Ok(ReadableEEPAddr::DeadZone),
            17 => Ok(ReadableEEPAddr::SaturatorOffset),
            18 => Ok(ReadableEEPAddr::SaturatorSlope),
            20 => Ok(ReadableEEPAddr::PWMOffset),
            21 => Ok(ReadableEEPAddr::MinPWM),
            22 => Ok(ReadableEEPAddr::MaxPWM),
            24 => Ok(ReadableEEPAddr::OverloadPWMThreshold),
            26 => Ok(ReadableEEPAddr::MinPosition),
            28 => Ok(ReadableEEPAddr::MaxPosition),
            30 => Ok(ReadableEEPAddr::PositionKp),
            32 => Ok(ReadableEEPAddr::PositionKd),
            34 => Ok(ReadableEEPAddr::PositionKi),
            36 => Ok(ReadableEEPAddr::PositionFFFirstGain),
            38 => Ok(ReadableEEPAddr::PositionFFSecondGain),
            44 => Ok(ReadableEEPAddr::LedBlinkPeriod),
            45 => Ok(ReadableEEPAddr::ADCFaultCheckPeriod),
            46 => Ok(ReadableEEPAddr::PacketGarbageDetectionPeriod),
            47 => Ok(ReadableEEPAddr::StopDetectionPeriod),
            48 => Ok(ReadableEEPAddr::OverloadDetectionPeriod),
            49 => Ok(ReadableEEPAddr::StopThreshold),
            50 => Ok(ReadableEEPAddr::InpositionMargin),
            53 => Ok(ReadableEEPAddr::CalibrationDifference),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// This enum represent all the EPP (permanent) memory addresses which can be written to. I comes
/// from the page 21 of the datasheet.
pub enum WritableEEPAddr {
    /// Communication speed
    BaudRate(u8),
    /// Servo ID
    ID(u8),
    /// TODO : Refer to pg 33
    AckPolicy(u8),
    /// Activates LED according to Policy
    AlarmLEDPolicy(u8),
    /// Releases Torque accroding to Policy
    TorquePolicy(u8),
    /// Maximum allowed temperature (0xDF = 85°C)
    MaxTemperature(u8),
    /// Minimum allowed voltage (0x5B = 6.714 VDC)
    MinVoltage(u8),
    /// Maximum allowed voltage (0x89 = 10 VDC)
    MaxVoltage(u8),
    /// Ratio of time to reach goal position to acceleration or deceleration
    AccelerationRatio(u8),
    /// Max acceleration time, 11.2ms interval. Acceleration(0x2D : 504 ms)
    MaxAccelerationTime(u8),
    /// Outside controle range
    DeadZone(u8),
    /// TODO : Refer to datasheet page 36
    SaturatorOffset(u8),
    /// TODO : Refer to datasheet page 36
    SaturatorSlope(u8, u8),
    /// PWM Offset value, refer to datasheet page 37
    PWMOffset(u8),
    /// Set minimum PWM value, refer to the datasheet page 37
    MinPWM(u8),
    /// Set maximum PWM value, refer to the datasheet page 37
    MaxPWM(u8, u8),
    /// Set PWM Overload thershold range, refer to the datasheet page 34
    OverloadPWMThreshold(u8, u8),
    /// Minimum position value (between 0 and 1023)
    MinPosition(u8, u8),
    /// Maximum position value (between 0 and 1023)
    MaxPosition(u8, u8),
    /// Proportional gain
    PositionKp(u8, u8),
    /// Derivative gain
    PositionKd(u8, u8),
    /// Integral gain
    PositionKi(u8, u8),
    /// Refer to the datasheet page 35
    PositionFFFirstGain(u8, u8),
    /// Refer to the datasheet page 35
    PositionFFSecondGain(u8, u8),
    /// Alarm LED blink period according to Policy 11.2ms/Tick (0x2D : 504 ms)
    LedBlinkPeriod(u8),
    /// Temp/Voltage error check interval. 11.2ms/tick (0x2D : 504 ms)
    ADCFaultCheckPeriod(u8),
    /// Packet error check interval. 11.2ms/tick (0x12 : 201 ms)
    PacketGarbageDetectionPeriod(u8),
    /// Stop detection check interval. 11.2ms/tick (0x1B : 302 ms)
    StopDetectionPeriod(u8),
    /// Overload check interbal. 11.2ms/tick (0x96 : 1.68 s)
    OverloadDetectionPeriod(u8),
    /// Stop Threshold
    StopThreshold(u8),
    /// Offset Threshold
    InpositionMargin(u8),
    /// Servo compensation
    CalibrationDifference(u8),
}

impl From<WritableEEPAddr> for u8 {
    fn from(me: WritableEEPAddr) -> Self {
        match me {
            WritableEEPAddr::BaudRate(_) => 4,
            WritableEEPAddr::ID(_) => 6,
            WritableEEPAddr::AckPolicy(_) => 7,
            WritableEEPAddr::AlarmLEDPolicy(_) => 8,
            WritableEEPAddr::TorquePolicy(_) => 9,
            WritableEEPAddr::MaxTemperature(_) => 11,
            WritableEEPAddr::MinVoltage(_) => 12,
            WritableEEPAddr::MaxVoltage(_) => 13,
            WritableEEPAddr::AccelerationRatio(_) => 14,
            WritableEEPAddr::MaxAccelerationTime(_) => 15,
            WritableEEPAddr::DeadZone(_) => 16,
            WritableEEPAddr::SaturatorOffset(_) => 17,
            WritableEEPAddr::SaturatorSlope(_, _) => 18,
            WritableEEPAddr::PWMOffset(_) => 20,
            WritableEEPAddr::MinPWM(_) => 21,
            WritableEEPAddr::MaxPWM(_, _) => 22,
            WritableEEPAddr::OverloadPWMThreshold(_, _) => 24,
            WritableEEPAddr::MinPosition(_, _) => 26,
            WritableEEPAddr::MaxPosition(_, _) => 28,
            WritableEEPAddr::PositionKp(_, _) => 30,
            WritableEEPAddr::PositionKd(_, _) => 32,
            WritableEEPAddr::PositionKi(_, _) => 34,
            WritableEEPAddr::PositionFFFirstGain(_, _) => 36,
            WritableEEPAddr::PositionFFSecondGain(_, _) => 38,
            WritableEEPAddr::LedBlinkPeriod(_) => 44,
            WritableEEPAddr::ADCFaultCheckPeriod(_) => 45,
            WritableEEPAddr::PacketGarbageDetectionPeriod(_) => 46,
            WritableEEPAddr::StopDetectionPeriod(_) => 47,
            WritableEEPAddr::OverloadDetectionPeriod(_) => 48,
            WritableEEPAddr::StopThreshold(_) => 49,
            WritableEEPAddr::InpositionMargin(_) => 50,
            WritableEEPAddr::CalibrationDifference(_) => 53,
        }
    }
}

enum Error {
    InvalidAddress
}

impl TryFrom<u8> for WritableEEPAddr {
    type Err = Error;
    fn try_from(me: u8) -> Result<WritableEEPAddr, Error> {
        match me {
            4 => Ok( WritableEEPAddr::BaudRate(0)),
            6 => Ok( WritableEEPAddr::ID(0)),
            7 => Ok( WritableEEPAddr::AckPolicy(0)),
            8 => Ok( WritableEEPAddr::AlarmLEDPolicy(0)),
            9 => Ok( WritableEEPAddr::TorquePolicy(0)),
            11 => Ok( WritableEEPAddr::MaxTemperature(0)),
            12 => Ok( WritableEEPAddr::MinVoltage(0)),
            13 => Ok( WritableEEPAddr::MaxVoltage(0)),
            14 => Ok( WritableEEPAddr::AccelerationRatio(0)),
            15 => Ok( WritableEEPAddr::MaxAccelerationTime(0)),
            16 => Ok( WritableEEPAddr::DeadZone(0)),
            17 => Ok( WritableEEPAddr::SaturatorOffset(0)),
            18 => Ok( WritableEEPAddr::SaturatorSlope(0, 0)),
            20 => Ok( WritableEEPAddr::PWMOffset(0)),
            21 => Ok( WritableEEPAddr::MinPWM(0)),
            22 => Ok( WritableEEPAddr::MaxPWM(0, 0)),
            24 => Ok( WritableEEPAddr::OverloadPWMThreshold(0, 0)),
            26 => Ok( WritableEEPAddr::MinPosition(0, 0)),
            28 => Ok( WritableEEPAddr::MaxPosition(0, 0)),
            30 => Ok( WritableEEPAddr::PositionKp(0, 0)),
            32 => Ok( WritableEEPAddr::PositionKd(0, 0)),
            34 => Ok( WritableEEPAddr::PositionKi(0, 0)),
            36 => Ok( WritableEEPAddr::PositionFFFirstGain(0, 0)),
            38 => Ok( WritableEEPAddr::PositionFFSecondGain(0, 0)),
            44 => Ok( WritableEEPAddr::LedBlinkPeriod(0)),
            45 => Ok( WritableEEPAddr::ADCFaultCheckPeriod(0)),
            46 => Ok( WritableEEPAddr::PacketGarbageDetectionPeriod(0)),
            47 => Ok( WritableEEPAddr::StopDetectionPeriod(0)),
            48 => Ok( WritableEEPAddr::OverloadDetectionPeriod(0)),
            49 => Ok( WritableEEPAddr::StopThreshold(0)),
            50 => Ok( WritableEEPAddr::InpositionMargin(0)),
            53 => Ok( WritableEEPAddr::CalibrationDifference(0)),
            _ => Err(Error::InvalidAddress)// blblblblblb
        }
    }
}

impl WritableEEPAddr {
    /// Return the number of bytes associated with an address
    pub fn bytes(&self) -> u8 {
        match *self {
            WritableEEPAddr::BaudRate(_) => 1,
            WritableEEPAddr::ID(_) => 1,
            WritableEEPAddr::AckPolicy(_) => 1,
            WritableEEPAddr::AlarmLEDPolicy(_) => 1,
            WritableEEPAddr::TorquePolicy(_) => 1,
            WritableEEPAddr::MaxTemperature(_) => 1,
            WritableEEPAddr::MinVoltage(_) => 1,
            WritableEEPAddr::MaxVoltage(_) => 1,
            WritableEEPAddr::AccelerationRatio(_) => 1,
            WritableEEPAddr::MaxAccelerationTime(_) => 1,
            WritableEEPAddr::DeadZone(_) => 1,
            WritableEEPAddr::SaturatorOffset(_) => 1,
            WritableEEPAddr::SaturatorSlope(_, _) => 2,
            WritableEEPAddr::PWMOffset(_) => 1,
            WritableEEPAddr::MinPWM(_) => 1,
            WritableEEPAddr::MaxPWM(_, _) => 2,
            WritableEEPAddr::OverloadPWMThreshold(_, _) => 2,
            WritableEEPAddr::MinPosition(_, _) => 2,
            WritableEEPAddr::MaxPosition(_, _) => 2,
            WritableEEPAddr::PositionKp(_, _) => 2,
            WritableEEPAddr::PositionKd(_, _) => 2,
            WritableEEPAddr::PositionKi(_, _) => 2,
            WritableEEPAddr::PositionFFFirstGain(_, _) => 2,
            WritableEEPAddr::PositionFFSecondGain(_, _) => 2,
            WritableEEPAddr::LedBlinkPeriod(_) => 1,
            WritableEEPAddr::ADCFaultCheckPeriod(_) => 1,
            WritableEEPAddr::PacketGarbageDetectionPeriod(_) => 1,
            WritableEEPAddr::StopDetectionPeriod(_) => 1,
            WritableEEPAddr::OverloadDetectionPeriod(_) => 1,
            WritableEEPAddr::StopThreshold(_) => 1,
            WritableEEPAddr::InpositionMargin(_) => 1,
            WritableEEPAddr::CalibrationDifference(_) => 1,
        }
    }

    pub(crate) fn associated_data(self) -> (u8, Option<u8>) {
        match self {
            WritableEEPAddr::BaudRate(d) => (d, None),
            WritableEEPAddr::ID(d) => (d, None),
            WritableEEPAddr::AckPolicy(d) => (d, None),
            WritableEEPAddr::AlarmLEDPolicy(d) => (d, None),
            WritableEEPAddr::TorquePolicy(d) => (d, None),
            WritableEEPAddr::MaxTemperature(d) => (d, None),
            WritableEEPAddr::MinVoltage(d) => (d, None),
            WritableEEPAddr::MaxVoltage(d) => (d, None),
            WritableEEPAddr::AccelerationRatio(d) => (d, None),
            WritableEEPAddr::MaxAccelerationTime(d) => (d, None),
            WritableEEPAddr::DeadZone(d) => (d, None),
            WritableEEPAddr::SaturatorOffset(d) => (d, None),
            WritableEEPAddr::SaturatorSlope(d, d2) => (d, Some(d2)),
            WritableEEPAddr::PWMOffset(d) => (d, None),
            WritableEEPAddr::MinPWM(d) => (d, None),
            WritableEEPAddr::MaxPWM(d, d2) => (d, Some(d2)),
            WritableEEPAddr::OverloadPWMThreshold(d, d2) => (d, Some(d2)),
            WritableEEPAddr::MinPosition(d, d2) => (d, Some(d2)),
            WritableEEPAddr::MaxPosition(d, d2) => (d, Some(d2)),
            WritableEEPAddr::PositionKp(d, d2) => (d, Some(d2)),
            WritableEEPAddr::PositionKd(d, d2) => (d, Some(d2)),
            WritableEEPAddr::PositionKi(d, d2) => (d, Some(d2)),
            WritableEEPAddr::PositionFFFirstGain(d, d2) => (d, Some(d2)),
            WritableEEPAddr::PositionFFSecondGain(d, d2) => (d, Some(d2)),
            WritableEEPAddr::LedBlinkPeriod(d) => (d, None),
            WritableEEPAddr::ADCFaultCheckPeriod(d) => (d, None),
            WritableEEPAddr::PacketGarbageDetectionPeriod(d) => (d, None),
            WritableEEPAddr::StopDetectionPeriod(d) => (d, None),
            WritableEEPAddr::OverloadDetectionPeriod(d) => (d, None),
            WritableEEPAddr::StopThreshold(d) => (d, None),
            WritableEEPAddr::InpositionMargin(d) => (d, None),
            WritableEEPAddr::CalibrationDifference(d) => (d, None),
        }
    }
}
