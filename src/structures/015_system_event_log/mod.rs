//! System Event Log (Type 15)
//!
//! The presence of this structure within the SMBIOS data returned for a system indicates that the
//! system supports an event log. An event log is a fixed-length area within a non-volatile storage
//! element, starting with a fixed-length (and vendor-specific) header record, followed by one or
//! more variable-length log records.\
//! An application can implement event-log change notification by periodically reading the System
//! Event Log structure (by its assigned handle) and looking for a change in the Log Change Token.
//! This token uniquely identifies the last time the event log was updated. When it sees the token
//! changed, the application can retrieve the entire event log and determine the changes since the
//! last time it read the event log.

use core::convert::TryInto;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::slice::Chunks;

use crate::{
    bitfield::{BitField, FlagType, Layout},
    InfoType,
    MalformedStructureError::{self, InvalidFormattedSectionLength},
    RawStructure,
};

pub mod log_record_format;
pub use self::log_record_format::{EventLogType, VariableDataFormatType};

/// Main struct for *System Event Log (Type 15) structure*
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SystemEventLog<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Length, in bytes, of the overall event log area, from the first byte of header to the last
    /// byte of data
    pub log_area_length: u16,
    /// Defines the starting offset (or index) within the nonvolatile storage of the event-log’s
    /// header, from the Access Method Address For single-byte indexed I/O accesses, the
    /// most-significant byte of the start offset is set to 00h.
    pub log_header_start_offset: u16,
    /// Defines the starting offset (or index) within the nonvolatile storage of the event-log’s
    /// first data byte, from the Access Method Address For single-byte indexed I/O accesses, the
    /// most-significant byte of the start offset is set to 00h.
    pub log_data_start_offset: u16,
    pub access_method: AccessMethod,
    pub log_status: LogStatus,
    /// Unique token that is reassigned every time the event log changes Can be used to determine
    /// if additional events have occurred since the last time the log was read.
    pub log_change_token: u32,
    /// Format of the log header area
    pub log_header_format: Option<LogHeaderFormat>,
    /// List of Supported Event Log Type Descriptors
    pub supported_event_log_type_descriptors: Option<SupportedEventLogTypeDescriptors<'a>>,
}

/// Defines the Location and Method used by higher-level software to access the log area.
///
/// Each variant contains address associated with the access method.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AccessMethod {
    /// Indexed I/O: 1 8-bit index port, 1 8-bit data port.
    IndexedIoOne8bitIndexOne8bitData { index: u8, data: u8 },
    /// Indexed I/O: 2 8-bit index ports, 1 8-bit data port.
    IndexedIoTwo8bitIndexOne8bitData { index: [u8; 2], data: u8 },
    /// Indexed I/O: 1 16-bit index port, 1 8-bit data port.
    IndexedIoOne16bitIndexOne8bitData { index: u16, data: u8 },
    /// Memory-mapped physical 32-bit address.
    MemoryMappedPhysicaAddress { physical_address: u32 },
    /// Available through General-Purpose NonVolatile Data functions.
    GeneralPurposeNonVolatileData { gpnv_handle: u16 },
    /// Available for future assignment
    Available { method: u8, address: u32 },
    /// BIOS Vendor/OEM-specific
    OemSpecific { method: u8, address: u32 },
}

/// Current status of the system event-log
///
/// The Log Status fields might not be up-to-date (dynamic) when the structure is accessed using
/// the table interface.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LogStatus(u8);

/// Identify the standard formats of the event log headers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LogHeaderFormat {
    /// No header (for example, the header is 0 bytes in length)
    NoHeader,
    /// Type 1 log header
    LogHeaderType1,
    /// Available for future assignment
    Available(u8),
    /// BIOS Vendor/OEM-specific
    OemSpecific(u8),
}

///// The type 1 event log header
//#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
//pub struct LogHeaderType1 {
//    /// Reserved area for OEM customization, not assignable by SMBIOS specification
//    pub oem_reserved: [u8; 5],
//    pub multiple_event: MultipleEvent,
//    pub pre_boot_event_log_reset: PreBootEventLogReset,
//    pub cmos_checksum: CmosChecksum,
//    /// Available for future assignment
//    pub reserved: [u8; 3],
//    /// Version of Type 1 header implemented
//    pub header_revision: u8,
//}
//#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
//pub struct MultipleEvent {
//    /// Number of minutes that must pass between duplicate log entries that utilize a
//    /// multiple-event counter, specified in BCD The value ranges from 00h to 99h to represent 0 to
//    /// 99 minutes.
//    pub time_window: u8,
//    /// Number of occurrences of a duplicate event that must pass before the multiple-event counter
//    /// associated with the log entry is updated, specified as a numeric value in the range 1 to
//    /// 255 (The value 0 is reserved.)
//    pub count_increment: u8,
//}
//#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
//pub struct PreBootEventLogReset {
//    /// CMOS RAM address (in the range 10h - FFh) associated with the Pre-boot Event Log Reset.
//    pub cmos_address: u8,
//    /// Bit within the above CMOS RAM location that is set to indicate that the log should be
//    /// cleared.
//    pub cmos_bit_index: u8,
//}
//#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
//pub struct CmosChecksum {
//    /// CMOS RAM address associated with the start of the area that is to be checksummed
//    pub starting_offset: u8,
//    /// Number of consecutive CMOS RAM addresses
//    pub byte_count: u8,
//    /// CMOS RAM address associated with the start of two consecutive bytes into which the
//    /// calculated checksum value is stored.
//    pub checksum_offset: u8,
//}

/// An iterator through Event Log Type Descriptors
#[derive(Clone, Debug)]
pub struct SupportedEventLogTypeDescriptors<'a>(Chunks<'a, u8>);

/// Supported Event Log Type descriptor
///
/// The presence of an entry identifies that the Log Type is supported by the system and the format
/// of any variable data that accompanies the first bytes of the log’s variable data — a specific
/// log record might have more variable data than specified by its Variable Data Format Type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EventLogTypeDescriptor {
    pub log_type: EventLogType,
    pub variable_data_format_type: VariableDataFormatType,
}

impl<'a> SystemEventLog<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let handle = structure.handle;
        let number_of_supported_log_type_descriptors = structure.get::<u8>(0x15).ok();
        let length_of_each_log_type_descriptor = structure.get::<u8>(0x16).ok();
        let len_gt_2_1 = number_of_supported_log_type_descriptors
            .and_then(|x| length_of_each_log_type_descriptor.map(|y| 0x17 + x as usize * y as usize));
        match (
            (structure.version.major, structure.version.minor),
            structure.data.len() + 4,
        ) {
            (v, l) if v == (2, 0) && l != 0x14 => Err(InvalidFormattedSectionLength(
                InfoType::SystemEventLog,
                handle,
                "",
                0x14,
            )),
            (v, l) if v >= (2, 1) && Some(l) != len_gt_2_1 => {
                if let Some(len) = len_gt_2_1 {
                    Err(InvalidFormattedSectionLength(
                        InfoType::SystemEventLog,
                        handle,
                        "17h+(x*y) = ",
                        len as u8,
                    ))
                } else {
                    Err(InvalidFormattedSectionLength(
                        InfoType::SystemEventLog,
                        handle,
                        "minimum of ",
                        0,
                    ))
                }
            }
            _ => {
                let access_method = {
                    let method = structure.get::<u8>(0x0A)?;
                    let address = structure.get::<u32>(0x10)?;
                    AccessMethod::new(method, address)
                };
                let supported_event_log_type_descriptors = (|| {
                    let number = number_of_supported_log_type_descriptors? as usize;
                    let length = length_of_each_log_type_descriptor? as usize;
                    let data = structure.get_slice(0x17, number * length)?;
                    Some(SupportedEventLogTypeDescriptors::new(data, length))
                })();
                Ok(Self {
                    handle,
                    log_area_length: structure.get::<u16>(0x04)?,
                    log_header_start_offset: structure.get::<u16>(0x06)?,
                    log_data_start_offset: structure.get::<u16>(0x08)?,
                    access_method,
                    log_status: structure.get::<u8>(0x0B)?.into(),
                    log_change_token: structure.get::<u32>(0x0C)?,
                    log_header_format: structure.get::<u8>(0x14).ok().map(Into::into),
                    supported_event_log_type_descriptors,
                })
            }
        }
    }
}

impl AccessMethod {
    /// According to [Table 62](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf)
    /// ## Access Method Address: DWORD layout
    ///
    /// | Access Type           | BYTE 3   | BYTE 2   | BYTE 1     | BYTE 0     |
    /// |-----------------------|----------|----------|------------|------------|
    /// | 00:02 – Indexed I/O   | Data MSB | Data LSB | Index MSB  | Index LSB  |
    /// | 03 – Absolute Address | Byte 3   | Byte 2   | Byte 1     | Byte 0     |
    /// | Use GPNV              | 0        | 0        | Handle MSB | Handle LSB |
    ///
    pub fn new(method: u8, address: u32) -> Self {
        let [index_lsb, index_msb, data_lsb, _data_msb] = u32::to_le_bytes(address);
        match method {
            0x00 => Self::IndexedIoOne8bitIndexOne8bitData {
                index: index_lsb,
                data: data_lsb,
            },
            0x01 => Self::IndexedIoTwo8bitIndexOne8bitData {
                index: [index_lsb, index_msb],
                data: data_lsb,
            },
            0x02 => Self::IndexedIoOne16bitIndexOne8bitData {
                index: u16::from_le_bytes([index_lsb, index_msb]),
                data: data_lsb,
            },
            0x03 => Self::MemoryMappedPhysicaAddress {
                physical_address: address,
            },
            0x04 => Self::GeneralPurposeNonVolatileData {
                gpnv_handle: u16::from_le_bytes([index_lsb, index_msb]),
            },
            method @ 0x80..=0xFF => Self::OemSpecific { method, address },
            method => Self::Available { method, address },
        }
    }
    pub fn address(&self) -> u32 {
        match self {
            Self::IndexedIoOne8bitIndexOne8bitData { index, data } => u32::from_le_bytes([*index, 0, *data, 0]),
            Self::IndexedIoTwo8bitIndexOne8bitData { index, data } => {
                u32::from_le_bytes([index[0], index[1], *data, 0])
            }
            Self::IndexedIoOne16bitIndexOne8bitData { index, data } => {
                let index = u16::to_le_bytes(*index);
                u32::from_le_bytes([index[0], index[1], *data, 0])
            }
            Self::MemoryMappedPhysicaAddress { physical_address } => *physical_address,
            Self::GeneralPurposeNonVolatileData { gpnv_handle } => *gpnv_handle as u32,
            Self::OemSpecific { address, .. } => *address,
            Self::Available { address, .. } => *address,
        }
    }
}
impl fmt::Display for AccessMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (f.alternate(), self) {
            (false, Self::IndexedIoOne8bitIndexOne8bitData { .. }) => {
                write!(f, "Indexed I/O, one 8-bit index port, one 8-bit data port")
            }
            (false, Self::IndexedIoTwo8bitIndexOne8bitData { .. }) => {
                write!(f, "Indexed I/O, two 8-bit index ports, one 8-bit data port")
            }
            (false, Self::IndexedIoOne16bitIndexOne8bitData { .. }) => {
                write!(f, "Indexed I/O, one 16-bit index port, one 8-bit data port")
            }
            (false, Self::MemoryMappedPhysicaAddress { .. }) => write!(f, "Memory-mapped physical 32-bit address"),
            (false, Self::GeneralPurposeNonVolatileData { .. }) => {
                write!(f, "General-purpose non-volatile data functions")
            }
            (false, Self::OemSpecific { method, .. }) => write!(f, "OEM-specific: {}", method),
            (false, Self::Available { method, .. }) => write!(f, "Available: {}", method),
            // With address
            (true, Self::IndexedIoOne8bitIndexOne8bitData { index, data }) => write!(
                f,
                "Indexed I/O, one 8-bit index port, one 8-bit data port: Index 0x{:02X}, Data 0x{:02X}",
                index, data
            ),
            (true, Self::IndexedIoTwo8bitIndexOne8bitData { index, data }) => write!(
                f,
                "Indexed I/O, two 8-bit index ports, one 8-bit data port: Index {:X?}, Data 0x{:02X}",
                index, data
            ),
            (true, Self::IndexedIoOne16bitIndexOne8bitData { index, data }) => write!(
                f,
                "Indexed I/O, one 16-bit index port, one 8-bit data port: Index 0x{:04X}, Data 0x{:02X}",
                index, data
            ),
            (true, Self::MemoryMappedPhysicaAddress { physical_address }) => {
                write!(f, "Memory-mapped physical 32-bit address: 0x{:08X}", physical_address)
            }
            (true, Self::GeneralPurposeNonVolatileData { gpnv_handle }) => write!(
                f,
                "General-Purpose NonVolatile Data functions, handle 0x{:04X}",
                gpnv_handle
            ),
            (true, Self::OemSpecific { method, address }) => write!(
                f,
                "BIOS Vendor/OEM-specific: Method {}, Address 0x{:08X}",
                method, address
            ),
            (true, Self::Available { method, address }) => {
                write!(f, "Available: Method {}, Address 0x{:08X}", method, address)
            }
        }
    }
}

impl BitField<'_> for LogStatus {
    type Size = u8;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 8;
        "Valid" "Log area valid",
        "Full" "Log area full",
        "Reserved": 6,
    );
}
impl From<u8> for LogStatus {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl From<u8> for LogHeaderFormat {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::NoHeader,
            0x01 => Self::LogHeaderType1,
            v @ 0x80..=0xFF => Self::OemSpecific(v),
            v => Self::Available(v),
        }
    }
}
impl fmt::Display for LogHeaderFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (f.alternate(), self) {
            (_, Self::NoHeader) => write!(f, "No Header"),
            (true, Self::LogHeaderType1) => write!(f, "Type 1 log header"),
            (false, Self::LogHeaderType1) => write!(f, "Type 1"),
            (true, Self::OemSpecific(v)) => write!(f, "BIOS vendor or OEM-specific format: {}", v),
            (false, Self::OemSpecific(_)) => write!(f, "OEM-specific"),
            (_, Self::Available(v)) => write!(f, "Available: {}", v),
        }
    }
}

impl<'a> SupportedEventLogTypeDescriptors<'a> {
    fn new(data: &'a [u8], size: usize) -> Self {
        Self(data.chunks(size))
    }
}
impl PartialEq for SupportedEventLogTypeDescriptors<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.clone().eq(other.0.clone())
    }
}
impl Eq for SupportedEventLogTypeDescriptors<'_> {}
impl Hash for SupportedEventLogTypeDescriptors<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.clone().for_each(|c| c.hash(state));
    }
}
impl Iterator for SupportedEventLogTypeDescriptors<'_> {
    type Item = EventLogTypeDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next()?;
        next.try_into().ok().map(|a: [u8; 2]| EventLogTypeDescriptor {
            log_type: a[0].into(),
            variable_data_format_type: a[1].into(),
        })
    }
}

impl From<[u8; 2]> for EventLogTypeDescriptor {
    fn from(a: [u8; 2]) -> Self {
        Self {
            log_type: a[0].into(),
            variable_data_format_type: a[1].into(),
        }
    }
}
impl From<EventLogTypeDescriptor> for [u8; 2] {
    fn from(d: EventLogTypeDescriptor) -> Self {
        [d.log_type.into(), d.variable_data_format_type.into()]
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn access_method() {
        use super::AccessMethod;

        let address = u32::from_le_bytes([0x78, 0x56, 0x34, 0x12]);
        let data = &[
            (
                0,
                "Indexed I/O, one 8-bit index port, one 8-bit data port: Index 0x78, Data 0x34",
            ),
            (
                1,
                "Indexed I/O, two 8-bit index ports, one 8-bit data port: Index [78, 56], Data 0x34",
            ),
            (
                2,
                "Indexed I/O, one 16-bit index port, one 8-bit data port: Index 0x5678, Data 0x34",
            ),
            (3, "Memory-mapped physical 32-bit address: 0x12345678"),
            (4, "General-Purpose NonVolatile Data functions, handle 0x5678"),
            (5, "Available: Method 5, Address 0x12345678"),
            (0x80, "BIOS Vendor/OEM-specific: Method 128, Address 0x12345678"),
        ];
        for (m, s) in data {
            assert_eq!(*s, format!("{:#}", AccessMethod::new(*m, address)));
        }
    }

    #[test]
    fn log_status() {
        use super::LogStatus;
        use bitfield::BitField;

        let byte: u8 = 0b111;
        let ls: LogStatus = byte.into();
        let sample = vec!["Log area valid", "Log area full"];
        assert_eq!(
            sample,
            ls.significants().map(|v| format!("{:#}", v)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn log_header_format() {
        use super::LogHeaderFormat;

        let data = &[
            (0u8, "No Header"),
            (1, "Type 1 log header"),
            (2, "Available: 2"),
            (0xFF, "BIOS vendor or OEM-specific format: 255"),
        ];
        for (v, s) in data {
            assert_eq!(*s, format!("{:#}", LogHeaderFormat::from(*v)));
        }
    }

    #[test]
    fn supported_event_log_type_descriptors() {
        use super::{
            EventLogType as T, EventLogTypeDescriptor as Desc, SupportedEventLogTypeDescriptors,
            VariableDataFormatType as D,
        };

        let data = &[
            0x02, 0x00, 0x04, 0x01, 0x08, 0x02, 0x16, 0x03, 0x66, 0x00, 0xEE, 0x00, 0xFF, 0x00,
        ];
        let sample = vec![
            Desc {
                log_type: T::MultiBitEccMemoryError,
                variable_data_format_type: D::None,
            },
            Desc {
                log_type: T::BusTimeOut,
                variable_data_format_type: D::Handle { handle: 0 },
            },
            Desc {
                log_type: T::PostError,
                variable_data_format_type: D::MultipleEvent { counter: 0 },
            },
            Desc {
                log_type: T::LogAreaReset,
                variable_data_format_type: D::MultipleEventHandle { handle: 0, counter: 0 },
            },
            Desc {
                log_type: T::Unused(0x66),
                variable_data_format_type: D::None,
            },
            Desc {
                log_type: T::Available(0xEE),
                variable_data_format_type: D::None,
            },
            Desc {
                log_type: T::EndOfLog,
                variable_data_format_type: D::None,
            },
        ];
        let result = SupportedEventLogTypeDescriptors::new(data, 2);
        assert_eq!(sample, result.collect::<Vec<_>>());
    }

    #[test]
    fn system_event_log() {
        use super::EventLogType as T;
        use super::VariableDataFormatType as D;
        use super::*;
        use crate::{bitfield::Position, InfoType, RawStructure};

        let length = 77 - 4;
        let (data, strings) =
            include_bytes!("../../../tests/data/02daadcd/entries/15-0/bin")[4..].split_at(length as usize);
        let structure = RawStructure {
            version: (2, 7).into(),
            info: InfoType::SystemEventLog,
            length,
            handle: 0x0036,
            data,
            strings,
        };
        let result = SystemEventLog::try_from(structure).unwrap();

        let access_method = AccessMethod::MemoryMappedPhysicaAddress {
            physical_address: 0xFFC40000,
        };
        assert_eq!(access_method, result.access_method, "AccessMethod");

        let log_status = [Position(0)].iter().collect::<u8>().into();
        assert_eq!(log_status, result.log_status, "LogStatus");

        let seltd_length = result.supported_event_log_type_descriptors.clone().unwrap().count();
        assert_eq!(27, seltd_length, "Supported Log Type Descriptors count");

        let seltd_sample = [
            (T::SingleBitEccMemoryError, D::None),
            (T::MultiBitEccMemoryError, D::None),
            (T::ParityMemoryError, D::None),
            (T::BusTimeOut, D::None),
            (T::IoChannelCheck, D::None),
            (T::SoftwareNmi, D::None),
            (T::PostMemoryResize, D::None),
            (T::PostError, D::PostResults(0.into())),
            (T::PciParityError, D::None),
            (T::PciSystemError, D::None),
            (T::CpuFailure, D::None),
            (T::EisaFailSafeTimerTimeOut, D::None),
            (T::CorrectableMemoryLogDisabled, D::None),
            (T::LoggingDisabledForSpecificEventType, D::None),
            (T::SystemLimitExceeded, D::None),
            (T::AsynchronousHardwareTimerExpired, D::None),
            (T::SystemConfigurationInformation, D::None),
            (T::HardDiskInformation, D::None),
            (T::SystemReconfigured, D::None),
            (T::UncorrectableCpuComplexError, D::None),
            (T::LogAreaReset, D::None),
            (T::SystemBoot, D::None),
            (T::EndOfLog, D::None),
            (T::Available(0xB0), D::OemAssigned(0xB0)),
            (T::Available(0xB1), D::OemAssigned(0xB1)),
            (T::Available(0xE0), D::OemAssigned(0xE0)),
            (T::Available(0xE1), D::OemAssigned(0xE1)),
        ]
        .iter()
        .map(|(t, d)| EventLogTypeDescriptor {
            log_type: *t,
            variable_data_format_type: *d,
        })
        .collect::<Vec<_>>();
        let seltd_result = result
            .supported_event_log_type_descriptors
            .clone()
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(seltd_sample, seltd_result, "SupportedEventLogTypeDescriptors");

        let sample_bytes = seltd_sample.iter().fold(Vec::new(), |mut vec: Vec<u8>, eltd| {
            vec.push(eltd.log_type.into());
            vec.push(eltd.variable_data_format_type.into());
            vec
        });
        let sample = SystemEventLog {
            handle: 0x0036,
            log_area_length: 16383,
            log_header_start_offset: 0x0000,
            log_data_start_offset: 0x0010,
            access_method,
            log_status,
            log_change_token: 0x00000001,
            log_header_format: Some(LogHeaderFormat::LogHeaderType1),
            supported_event_log_type_descriptors: Some(SupportedEventLogTypeDescriptors::new(&sample_bytes, 2)),
        };
        assert_eq!(sample, result, "SystemEventLog");
    }
}
