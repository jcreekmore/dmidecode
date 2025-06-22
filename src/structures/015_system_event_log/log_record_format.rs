//! Log Record format
//!
//! Each log record consists of a required fixed-length record header, followed by (optional)
//! additional data that is defined by the event type. The fixed-length log record header is
//! present as the first eight bytes of each log record, regardless of event type.
//!
//! Most of data in this module does not present in System Event Log (Type 15) structure, but
//! describes data in Event Log

use core::fmt;

use crate::bitfield::{BitField, FlagType, Layout};

/// Log Record format
///
/// Each log record consists of a required fixed-length record header, followed by (optional)
/// additional data that is defined by the event type. The fixed-length log record header is
/// present as the first eight bytes of each log record, regardless of event type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LogRecordFormat {
    event_type: EventLogType,
    /// Specifies the byte length of the event record, including the record’s Type and Length
    /// fields The most-significant bit of the field specifies whether (0) or not (1) the record
    /// has been read. The implication of the record having been read is that the information in
    /// the log record has been processed by a higher software layer.
    length: u8,
    datetime: Datetime,
    log_variable_data: Option<LogVariableData>,
}

/// BCD representation of the date and time of the occurrence of the event
///
/// The information is present in year, month, day, hour, minute, and second order.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Datetime {
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

/// Event-specific additional status information
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LogVariableData;

/// Specifies the “Type” of event noted in an event-log entry
///
/// Defined in [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf) 7.16.6.1
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EventLogType {
    Reserved(u8),
    /// Single-bit ECC memory error
    SingleBitEccMemoryError,
    /// Multi-bit ECC memory error
    MultiBitEccMemoryError,
    /// Parity memory error
    ParityMemoryError,
    /// Bus time-out
    BusTimeOut,
    /// I/O Channel Check
    IoChannelCheck,
    /// Software NMI
    SoftwareNmi,
    /// POST Memory Resize
    PostMemoryResize,
    /// POST Error
    PostError,
    /// PCI Parity Error
    PciParityError,
    /// PCI System Error
    PciSystemError,
    /// CPU Failure
    CpuFailure,
    /// EISA FailSafe Timer time-out
    EisaFailSafeTimerTimeOut,
    /// Correctable memory log disabled
    CorrectableMemoryLogDisabled,
    /// Logging disabled for a specific Event Type — too many errors of the same type received in a
    /// short amount of time
    LoggingDisabledForSpecificEventType,
    /// System Limit Exceeded (for example, voltage or temperature threshold exceeded)
    SystemLimitExceeded,
    /// Asynchronous hardware timer expired and issued a system reset
    AsynchronousHardwareTimerExpired,
    /// System configuration information
    SystemConfigurationInformation,
    /// Hard-disk information
    HardDiskInformation,
    /// System reconfigured
    SystemReconfigured,
    /// Uncorrectable CPU-complex error
    UncorrectableCpuComplexError,
    /// Log Area Reset/Cleared
    LogAreaReset,
    /// System boot. If implemented, this log entry is guaranteed to be the first one written on
    /// any system boot.
    SystemBoot,
    /// Unused, available for assignment by this specification
    Unused(u8),
    /// Available for system- and OEM-specific assignments
    Available(u8),
    /// End of log\
    /// When an application searches through the event-log records, the end of the log is
    /// identified when a log record with this type is found.
    EndOfLog,
}

/// Variable Data Format Type
///
/// Identifies the standard format that application software can apply to the first n bytes
/// of the associated Log Type’s variable data. Additional OEM-specific data might follow in the
/// log’s variable data field.\
/// Defined in [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf) 7.16.6.2
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VariableDataFormatType {
    /// No standard format data is available.
    None,
    /// Contains the handle of the SMBIOS structure associated with the hardware element that failed.
    Handle { handle: u16 },
    /// Multiple-Event\
    /// Contains a multiple-event counter
    MultipleEvent { counter: u32 },
    /// Multiple-Event Handle\
    /// Contains the handle of the SMBIOS structure associated with the hardware element that
    /// failed;  it is followed by a DWORD containing a multiple-event counter
    MultipleEventHandle { handle: u16, counter: u32 },
    /// POST Results Bitmap
    PostResults(PostResults),
    /// System Management Type
    SystemManagementType(SystemManagementType),
    /// Multiple-Event System Management Type
    MultipleEventSystemManagementType {
        system_management_type: SystemManagementType,
        counter: u32,
    },
    /// Unused, available for assignment
    Unused(u8),
    /// Available for system- and OEM-specific assignments.
    OemAssigned(u8),
}

/// Multiple-Event Counter
///
/// Some system events can be persistent; after they occur, it is possible to quickly fill the log
/// with redundant multiple logs. The Multiple Event Count Increment (MECI) and Multiple Event Time
/// Window (METW) values can be used to reduce the occurrence of these multiple logs while
/// providing multiple event counts.\
/// Defined in [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf) 7.16.6.3
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MultipleEvent {
    /// Number of minutes that must pass between duplicate log entries that utilize a
    /// multiple-event counter, specified in BCD The value ranges from 00h to 99h to represent 0 to
    /// 99 minutes.
    pub time_window: u8,
    /// Number of occurrences of a duplicate event that must pass before the multiple-event counter
    /// associated with the log entry is updated, specified as a numeric value in the range 1 to
    /// 255 (The value 0 is reserved.)
    pub count_increment: u8,
}

/// POST Results Bitmap
///
/// This variable data type expected to be associated with the POST Error (08h) event log type and
/// identifies that one or more error types have occurred. The bitmap consists of two DWORD
/// values.\
/// Defined in [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf) 7.16.6.4
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PostResults(u64);

/// System management types
///
/// System management types present in an event log record’s variable data.
/// In general, each type is associated with a management event that occurred within the system.\
/// Defined in [SMBIOS Specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf) 7.16.6.5
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SystemManagementType {
    /// +2.5V Out of range, #1
    OutOfRangeVoltagePlus2_5Num1,
    /// +2.5V Out of range, #2
    OutOfRangeVoltagePlus2_5Num2,
    /// +3.3V Out of range
    OutOfRangeVoltagePlus3_3,
    /// +5V Out of range
    OutOfRangeVoltagePlus5,
    /// -5V Out of range
    OutOfRangeVoltageMinus5,
    /// +12V Out of range
    OutOfRangeVoltagePlus12,
    /// -12V Out of range
    OutOfRangeVoltageMinus12,
    /// Reserved for future out-of-range voltage levels
    OutOfRangeVoltageReserved(u8),
    /// System board temperature out of range
    OutOfRangeTemperatureSystemBoard,
    /// Processor #1 temperature out of range
    OutOfRangeTemperatureProcessor1,
    /// Processor #2 temperature out of range
    OutOfRangeTemperatureProcessor2,
    /// Processor #3 temperature out of range
    OutOfRangeTemperatureProcessor3,
    /// Processor #4 temperature out of range
    OutOfRangeTemperatureProcessor4,
    /// Reserved for future out-of-range temperatures
    OutOfRangeTemperatureReserved(u8),
    /// Fan n (n = 0 to 7) Out of range
    OutOfRangeFan(u8),
    /// Reserved for future assignment by this specification
    Reserved(u32),
    /// Chassis secure switch activated
    ChassisSecureSwitchActivated,
    /// A system-management probe or cooling device is out of range. Contains the handle of the
    /// SMBIOS structure associated with the errant device.
    OutOfRangeSystemManagementProbe(u16),
    /// OEM assigned
    OemAssigned(u32),
}

impl From<u8> for EventLogType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 | 0x0F => Self::Reserved(byte),
            0x01 => Self::SingleBitEccMemoryError,
            0x02 => Self::MultiBitEccMemoryError,
            0x03 => Self::ParityMemoryError,
            0x04 => Self::BusTimeOut,
            0x05 => Self::IoChannelCheck,
            0x06 => Self::SoftwareNmi,
            0x07 => Self::PostMemoryResize,
            0x08 => Self::PostError,
            0x09 => Self::PciParityError,
            0x0A => Self::PciSystemError,
            0x0B => Self::CpuFailure,
            0x0C => Self::EisaFailSafeTimerTimeOut,
            0x0D => Self::CorrectableMemoryLogDisabled,
            0x0E => Self::LoggingDisabledForSpecificEventType,
            0x10 => Self::SystemLimitExceeded,
            0x11 => Self::AsynchronousHardwareTimerExpired,
            0x12 => Self::SystemConfigurationInformation,
            0x13 => Self::HardDiskInformation,
            0x14 => Self::SystemReconfigured,
            0x15 => Self::UncorrectableCpuComplexError,
            0x16 => Self::LogAreaReset,
            0x17 => Self::SystemBoot,
            v @ 0x18..=0x7F => Self::Unused(v),
            v @ 0x80..=0xFE => Self::Available(v),
            0xFF => Self::EndOfLog,
        }
    }
}
impl From<EventLogType> for u8 {
    fn from(type_: EventLogType) -> Self {
        match type_ {
            EventLogType::Reserved(byte) => byte,
            EventLogType::SingleBitEccMemoryError => 0x01,
            EventLogType::MultiBitEccMemoryError => 0x02,
            EventLogType::ParityMemoryError => 0x03,
            EventLogType::BusTimeOut => 0x04,
            EventLogType::IoChannelCheck => 0x05,
            EventLogType::SoftwareNmi => 0x06,
            EventLogType::PostMemoryResize => 0x07,
            EventLogType::PostError => 0x08,
            EventLogType::PciParityError => 0x09,
            EventLogType::PciSystemError => 0x0a,
            EventLogType::CpuFailure => 0x0b,
            EventLogType::EisaFailSafeTimerTimeOut => 0x0c,
            EventLogType::CorrectableMemoryLogDisabled => 0x0d,
            EventLogType::LoggingDisabledForSpecificEventType => 0x0e,
            EventLogType::SystemLimitExceeded => 0x10,
            EventLogType::AsynchronousHardwareTimerExpired => 0x11,
            EventLogType::SystemConfigurationInformation => 0x12,
            EventLogType::HardDiskInformation => 0x13,
            EventLogType::SystemReconfigured => 0x14,
            EventLogType::UncorrectableCpuComplexError => 0x15,
            EventLogType::LogAreaReset => 0x16,
            EventLogType::SystemBoot => 0x17,
            EventLogType::Unused(v) => v,
            EventLogType::Available(v) => v,
            EventLogType::EndOfLog => 0xFF,
        }
    }
}
impl fmt::Display for EventLogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (f.alternate(), self) {
            (true, Self::Reserved(v)) => write!(f, "Reserved: {}", v),
            (false, Self::Reserved(_)) => write!(f, "Reserved"),
            (_, Self::SingleBitEccMemoryError) => write!(f, "Single-bit ECC memory error"),
            (_, Self::MultiBitEccMemoryError) => write!(f, "Multi-bit ECC memory error"),
            (_, Self::ParityMemoryError) => write!(f, "Parity memory error"),
            (true, Self::BusTimeOut) => write!(f, "Bus time-out"),
            (false, Self::BusTimeOut) => write!(f, "Bus timeout"),
            (true, Self::IoChannelCheck) => write!(f, "I/O Channel Check"),
            (false, Self::IoChannelCheck) => write!(f, "I/O channel block"),
            (_, Self::SoftwareNmi) => write!(f, "Software NMI"),
            (true, Self::PostMemoryResize) => write!(f, "POST Memory Resize"),
            (false, Self::PostMemoryResize) => write!(f, "POST memory resize"),
            (true, Self::PostError) => write!(f, "POST Error"),
            (false, Self::PostError) => write!(f, "POST error"),
            (true, Self::PciParityError) => write!(f, "PCI Parity Error"),
            (false, Self::PciParityError) => write!(f, "PCI parity error"),
            (true, Self::PciSystemError) => write!(f, "PCI System Error"),
            (false, Self::PciSystemError) => write!(f, "PCI system error"),
            (true, Self::CpuFailure) => write!(f, "CPU Failure"),
            (false, Self::CpuFailure) => write!(f, "CPU failure"),
            (true, Self::EisaFailSafeTimerTimeOut) => write!(f, "EISA FailSafe Timer time-out"),
            (false, Self::EisaFailSafeTimerTimeOut) => write!(f, "EISA failsafe timer timeout"),
            (_, Self::CorrectableMemoryLogDisabled) => write!(f, "Correctable memory log disabled"),
            (true, Self::LoggingDisabledForSpecificEventType) => {
                write!(f, "Logging disabled for a specific Event Type")
            }
            (false, Self::LoggingDisabledForSpecificEventType) => write!(f, "Logging disabled"),
            (true, Self::SystemLimitExceeded) => write!(f, "System Limit Exceeded"),
            (false, Self::SystemLimitExceeded) => write!(f, "System limit exceeded"),
            (true, Self::AsynchronousHardwareTimerExpired) => {
                write!(f, "Asynchronous hardware timer expired and issued a system reset")
            }
            (false, Self::AsynchronousHardwareTimerExpired) => {
                write!(f, "Asynchronous hardware timer expired")
            }
            (_, Self::SystemConfigurationInformation) => {
                write!(f, "System configuration information")
            }
            (true, Self::HardDiskInformation) => write!(f, "Hard-disk information"),
            (false, Self::HardDiskInformation) => write!(f, "Hard disk information"),
            (_, Self::SystemReconfigured) => write!(f, "System reconfigured"),
            (_, Self::UncorrectableCpuComplexError) => write!(f, "Uncorrectable CPU-complex error"),
            (true, Self::LogAreaReset) => write!(f, "Log Area Reset/Cleared"),
            (false, Self::LogAreaReset) => write!(f, "Log area reset/cleared"),
            (_, Self::SystemBoot) => write!(f, "System boot"),
            (_, Self::Unused(v)) => write!(f, "Unused: {}", v),
            (true, Self::Available(v)) => write!(f, "Available for system- and OEM-specific assignments: {}", v),
            (false, Self::Available(_)) => write!(f, "OEM-specific"),
            (_, Self::EndOfLog) => write!(f, "End of log"),
        }
    }
}

impl From<u8> for VariableDataFormatType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            0x01 => Self::Handle { handle: 0 },
            0x02 => Self::MultipleEvent { counter: 0 },
            0x03 => Self::MultipleEventHandle { handle: 0, counter: 0 },
            0x04 => Self::PostResults((0).into()),
            0x05 => Self::SystemManagementType((0xFFFF).into()),
            0x06 => Self::MultipleEventSystemManagementType {
                system_management_type: (0xFFFF).into(),
                counter: 0,
            },
            v @ 0x07..=0x7F => Self::Unused(v),
            v @ 0x80..=0xFF => Self::OemAssigned(v),
        }
    }
}
impl From<VariableDataFormatType> for u8 {
    fn from(type_: VariableDataFormatType) -> Self {
        match type_ {
            VariableDataFormatType::None => 0x00,
            VariableDataFormatType::Handle { .. } => 0x01,
            VariableDataFormatType::MultipleEvent { .. } => 0x02,
            VariableDataFormatType::MultipleEventHandle { .. } => 0x03,
            VariableDataFormatType::PostResults(_) => 0x04,
            VariableDataFormatType::SystemManagementType(_) => 0x05,
            VariableDataFormatType::MultipleEventSystemManagementType { .. } => 0x06,
            VariableDataFormatType::Unused(v) => v,
            VariableDataFormatType::OemAssigned(v) => v,
        }
    }
}
impl fmt::Display for VariableDataFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (f.alternate(), self) {
            (true, Self::None) => write!(f, "No standard format data is available"),
            (false, Self::None) => write!(f, "None"),
            (true, Self::Handle { handle }) => {
                write!(f, "SMBIOS structure associated handle: {}", handle)
            }
            (false, Self::Handle { .. }) => write!(f, "Handle"),
            (true, Self::MultipleEvent { counter }) => {
                write!(f, "Multiple-event counter value: {}", counter)
            }
            (false, Self::MultipleEvent { .. }) => write!(f, "Multiple-event"),
            (true, Self::MultipleEventHandle { handle, counter }) => {
                write!(f, "Multiple-event: Handle 0x{:04X}, Count {}", handle, counter)
            }
            (false, Self::MultipleEventHandle { .. }) => write!(f, "Multiple-event handle"),
            (true, Self::PostResults(pr)) => write!(f, "POST result: {:#X}", pr.0),
            (false, Self::PostResults(_)) => write!(f, "POST results bitmap"),
            (true, Self::SystemManagementType(sm)) => write!(f, "System management: {}", sm),
            (false, Self::SystemManagementType(_)) => write!(f, "System management"),
            (
                true,
                Self::MultipleEventSystemManagementType {
                    system_management_type,
                    counter,
                },
            ) => write!(
                f,
                "Multiple-event system management: Type {}, Count {}",
                system_management_type, counter
            ),
            (false, Self::MultipleEventSystemManagementType { .. }) => {
                write!(f, "Multiple-event system management")
            }
            (_, Self::Unused(v)) => write!(f, "Unused: {}", v),
            (true, Self::OemAssigned(v)) => write!(f, "OEM assigned: {}", v),
            (false, Self::OemAssigned(_)) => write!(f, "OEM-specific"),
        }
    }
}

impl BitField<'_> for PostResults {
    type Size = u64;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 64;
        "Channel 2 Timer error",
        "Primary PIC (8259 #1) error",
        "Secondary PIC (8259 #2) error",
        "CMOS RAM Battery Failure",
        "CMOS RAM System Options Not Set",
        "CMOS RAM Checksum Error",
        "CMOS RAM Configuration Error",
        "Mouse and Keyboard Swapped",
        "Keyboard Locked",
        "Keyboard Not Functional",
        "Keyboard Controller Not Functional",
        "CMOS Memory Size Different",
        "Memory Decreased in Size",
        "Cache Memory Error",
        "Floppy Drive 0 Error",
        "Floppy Drive 1 Error",
        "Floppy Controller Failure",
        "Number of ATA Drives Reduced Error",
        "RTC Time Not Set",
        "DDC Monitor Configuration Change",
        "Reserved": 4,
        "Second DWORD has valid data",
        "Reserved": 3,
        "Available for OEM assignment": 11,
        "PCI Memory Conflict",
        "PCI I/O Conflict",
        "PCI IRQ Conflict",
        "PNP Memory Conflict",
        "PNP 32 bit Memory Conflict",
        "PNP I/O Conflict",
        "PNP IRQ Conflict",
        "PNP DMA Conflict",
        "Bad PNP Serial ID Checksum",
        "Bad PNP Resource Data Checksum",
        "Static Resource Conflict",
        "NVRAM Checksum Error, NVRAM Cleared",
        "System Board Device Resource Conflict",
        "Primary Output Device Not Found",
        "Primary Input Device Not Found",
        "Primary Boot Device Not Found",
        "NVRAM Cleared By Jumper",
        "NVRAM Data Invalid, NVRAM Cleared",
        "FDC Resource Conflict",
        "Primary ATA Controller Resource Conflict",
        "Secondary ATA Controller Resource Conflict",
        "Parallel Port Resource Conflict",
        "Serial Port 1 Resource Conflict",
        "Serial Port 2 Resource Conflict",
        "Audio Resource Conflict",
    );
}
impl From<u64> for PostResults {
    fn from(qword: u64) -> Self {
        Self(qword)
    }
}

impl From<u32> for SystemManagementType {
    fn from(byte: u32) -> Self {
        match byte {
            0x00000000 => Self::OutOfRangeVoltagePlus2_5Num1,
            0x00000001 => Self::OutOfRangeVoltagePlus2_5Num2,
            0x00000002 => Self::OutOfRangeVoltagePlus3_3,
            0x00000003 => Self::OutOfRangeVoltagePlus5,
            0x00000004 => Self::OutOfRangeVoltageMinus5,
            0x00000005 => Self::OutOfRangeVoltagePlus12,
            0x00000006 => Self::OutOfRangeVoltageMinus12,
            v @ 0x00000007..=0x0000000F => Self::OutOfRangeVoltageReserved(v as u8),
            0x00000010 => Self::OutOfRangeTemperatureSystemBoard,
            0x00000011 => Self::OutOfRangeTemperatureProcessor1,
            0x00000012 => Self::OutOfRangeTemperatureProcessor2,
            0x00000013 => Self::OutOfRangeTemperatureProcessor3,
            0x00000014 => Self::OutOfRangeTemperatureProcessor4,
            v @ 0x00000015..=0x1F => Self::OutOfRangeTemperatureReserved(v as u8),
            v @ 0x00000020..=0x27 => Self::OutOfRangeFan((v & 0b111) as u8),
            0x00000030 => Self::ChassisSecureSwitchActivated,
            v @ 0x00010000..=0x0001FFFF => Self::OutOfRangeSystemManagementProbe(v as u16),
            v @ 0x80000000..=0xFFFFFFFF => Self::OemAssigned(v),
            v => Self::Reserved(v),
        }
    }
}
impl fmt::Display for SystemManagementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRangeVoltagePlus2_5Num1 => write!(f, "+2.5V Out of range, #1"),
            Self::OutOfRangeVoltagePlus2_5Num2 => write!(f, "+2.5V Out of range, #2"),
            Self::OutOfRangeVoltagePlus3_3 => write!(f, "+3.3V Out of range"),
            Self::OutOfRangeVoltagePlus5 => write!(f, "+5V Out of range"),
            Self::OutOfRangeVoltageMinus5 => write!(f, "-5V Out of range"),
            Self::OutOfRangeVoltagePlus12 => write!(f, "+12V Out of range"),
            Self::OutOfRangeVoltageMinus12 => write!(f, "-12V Out of range"),
            Self::OutOfRangeVoltageReserved(v) => write!(f, "Out-of-range voltage reserved: {}", v),
            Self::OutOfRangeTemperatureSystemBoard => {
                write!(f, "System board temperature out of range")
            }
            Self::OutOfRangeTemperatureProcessor1 => {
                write!(f, "Processor #1 temperature out of range")
            }
            Self::OutOfRangeTemperatureProcessor2 => {
                write!(f, "Processor #2 temperature out of range")
            }
            Self::OutOfRangeTemperatureProcessor3 => {
                write!(f, "Processor #3 temperature out of range")
            }
            Self::OutOfRangeTemperatureProcessor4 => {
                write!(f, "Processor #4 temperature out of range")
            }
            Self::OutOfRangeTemperatureReserved(v) => {
                write!(f, "Out-of-range temperatures reserved: {}", v)
            }
            Self::OutOfRangeFan(v) => write!(f, "Fan {} Out of range", v),
            Self::ChassisSecureSwitchActivated => write!(f, "Chassis secure switch activated"),
            Self::OutOfRangeSystemManagementProbe(v) => write!(
                f,
                "A system-management probe or cooling device with handle 0x{:04X} is out of range",
                v
            ),
            Self::OemAssigned(v) => write!(f, "OEM assigned: {}", v),
            Self::Reserved(v) => write!(f, "Reserved: {}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn system_management_type() {
        use super::SystemManagementType::{self, *};
        let data = [
            (0x03, OutOfRangeVoltagePlus5, "+5V Out of range"),
            (0x0A, OutOfRangeVoltageReserved(10), "Out-of-range voltage reserved: 10"),
            (
                0x1F,
                OutOfRangeTemperatureReserved(31),
                "Out-of-range temperatures reserved: 31",
            ),
            (0x23, OutOfRangeFan(3), "Fan 3 Out of range"),
            (
                0x1BEEF,
                OutOfRangeSystemManagementProbe(0xBEEF),
                "A system-management probe or cooling device with handle 0xBEEF is out of range",
            ),
            (0x20000, Reserved(131072), "Reserved: 131072"),
            (u32::MAX, OemAssigned(u32::MAX), "OEM assigned: 4294967295"),
        ];
        let result = data.iter().map(|v| v.0.into()).collect::<Vec<SystemManagementType>>();
        let enum_sample = data.iter().map(|v| v.1).collect::<Vec<_>>();
        let display_sample = data.iter().map(|v| v.2).collect::<Vec<_>>();
        assert_eq!(enum_sample, result, "Enum variants");
        assert_eq!(
            display_sample,
            result.iter().map(|v| format!("{}", v)).collect::<Vec<_>>(),
            "Enum variants"
        );
    }

    #[test]
    fn post_results() {
        use super::PostResults;
        use crate::bitfield::{BitField, FlagType::Reserved, Position};

        let qword: u64 = 0b10101010000 << 32 | 0b101010;
        let pr: PostResults = qword.into();
        let significant_sample = vec![
            "Primary PIC (8259 #1) error",
            "CMOS RAM Battery Failure",
            "CMOS RAM Checksum Error",
            "PCI I/O Conflict",
            "PNP Memory Conflict",
        ];
        let reserved_sample = vec![
            (Position(36), "Available for OEM assignment".to_string()),
            (Position(38), "Available for OEM assignment".to_string()),
        ];
        assert_eq!(
            significant_sample,
            pr.significants().map(|v| format!("{}", v)).collect::<Vec<_>>(),
            "Significants"
        );
        assert_eq!(
            reserved_sample,
            pr.iter()
                .filter_map(|f| {
                    if f.is_set && matches!(f.type_, Reserved(_)) {
                        Some((f.position, format!("{}", f)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
            "Reserved"
        );
    }
}
