//! 32-Bit Memory Error Information (Type 18)
//!
//! This structure identifies the specifics of an error that might be detected within a Physical Memory Array.

use core::fmt;

use crate::{
    InfoType,
    MalformedStructureError::{self, InvalidFormattedSectionLength},
    RawStructure,
};

/// Main struct for *32-Bit Memory Error Information (Type 18) structure*
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemoryError32 {
    /// Specifies the structure’s handle
    pub handle: u16,
    pub error_type: ErrorType,
    pub error_granularity: ErrorGranularity,
    pub error_operation: ErrorOperation,
    /// Vendor-specific ECC syndrome or CRC data associated with the erroneous access.\
    /// If the value is unknown, this field contains 0000 0000h.
    pub vendor_syndrome: u32,
    /// 32-bit physical address of the error based on the addressing of the bus to which the memory
    /// array is connected.\
    /// If the address is unknown, this field contains 8000 0000h.
    pub memory_array_error_address: u32,
    /// 32-bit physical address of the error relative to the start of the failing memory device, in
    /// bytes.\
    /// If the address is unknown, this field contains 8000 0000h.
    pub device_error_address: u32,
    /// Range, in bytes, within which the error can be determined, when an error address is given.\
    /// If the range is unknown, this field contains 8000 0000h.
    pub error_resolution: u32,
}

/// Type of error that is associated with the current status reported for the memory array or
/// device
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorType {
    Other,
    Unknown,
    Ok,
    BadRead,
    ParityError,
    SingleBitError,
    DoubleBitError,
    MultiBitError,
    NibbleError,
    ChecksumError,
    CrcError,
    CorrectedSingleBitError,
    CorrectedError,
    UncorrectableError,
    Undefined(u8),
}

/// Granularity (for example, device versus Partition) to which the error can be resolved
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorGranularity {
    Other,
    Unknown,
    /// Device level
    DeviceLevel,
    /// Memory partition level
    MemoryPartitionLevel,
    Undefined(u8),
}

/// Memory access operation that caused the error
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorOperation {
    Other,
    Unknown,
    Read,
    Write,
    /// Partial write
    PartialWrite,
    Undefined(u8),
}

impl<'a> MemoryError32 {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let handle = structure.handle;
        if structure.version >= (2, 1).into() && structure.length != 0x17 {
            Err(InvalidFormattedSectionLength(InfoType::MemoryError32, handle, "", 0x17))
        } else {
            Ok(Self {
                handle,
                error_type: structure.get::<u8>(0x04)?.into(),
                error_granularity: structure.get::<u8>(0x05)?.into(),
                error_operation: structure.get::<u8>(0x06)?.into(),
                vendor_syndrome: structure.get::<u32>(0x07)?,
                memory_array_error_address: structure.get::<u32>(0x0B)?,
                device_error_address: structure.get::<u32>(0x0F)?,
                error_resolution: structure.get::<u32>(0x13)?,
            })
        }
    }
}

impl From<u8> for ErrorType {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Ok,
            0x04 => Self::BadRead,
            0x05 => Self::ParityError,
            0x06 => Self::SingleBitError,
            0x07 => Self::DoubleBitError,
            0x08 => Self::MultiBitError,
            0x09 => Self::NibbleError,
            0x0a => Self::ChecksumError,
            0x0b => Self::CrcError,
            0x0c => Self::CorrectedSingleBitError,
            0x0d => Self::CorrectedError,
            0x0e => Self::UncorrectableError,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Ok => write!(f, "OK"),
            Self::BadRead => write!(f, "Bad read"),
            Self::ParityError => write!(f, "Parity error"),
            Self::SingleBitError => write!(f, "Single-bit error"),
            Self::DoubleBitError => write!(f, "Double-bit error"),
            Self::MultiBitError => write!(f, "Multi-bit error"),
            Self::NibbleError => write!(f, "Nibble error"),
            Self::ChecksumError => write!(f, "Checksum error"),
            Self::CrcError => write!(f, "CRC error"),
            Self::CorrectedSingleBitError => write!(f, "Corrected single-bit error"),
            Self::CorrectedError => write!(f, "Corrected error"),
            Self::UncorrectableError => write!(f, "Uncorrectable error"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for ErrorGranularity {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::DeviceLevel,
            0x04 => Self::MemoryPartitionLevel,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for ErrorGranularity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::DeviceLevel => write!(f, "Device level"),
            Self::MemoryPartitionLevel => write!(f, "Memory partition level"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for ErrorOperation {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Read,
            0x04 => Self::Write,
            0x05 => Self::PartialWrite,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for ErrorOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Read => write!(f, "Read"),
            Self::Write => write!(f, "Write"),
            Self::PartialWrite => write!(f, "Partial write"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn error_operation() {
        use super::ErrorOperation;

        let sample = &["", "Other", "Unknown", "Read", "Write", "Partial write", "Undefined: 6"];
        for n in 1u8..7 {
            assert_eq!(sample[n as usize], format!("{:#}", ErrorOperation::from(n)));
        }
    }

    #[test]
    fn error_granularity() {
        use super::ErrorGranularity;

        let sample = &[
            "Undefined: 0",
            "Other",
            "Unknown",
            "Device level",
            "Memory partition level",
            "Undefined: 5",
        ];
        for n in 0u8..6 {
            assert_eq!(sample[n as usize], format!("{:#}", ErrorGranularity::from(n)));
        }
    }

    #[test]
    fn error_type() {
        use super::ErrorType;

        let sample = &[
            "Undefined: 0",
            "Other",
            "Unknown",
            "OK",
            "Bad read",
            "Parity error",
            "Single-bit error",
            "Double-bit error",
            "Multi-bit error",
            "Nibble error",
            "Checksum error",
            "CRC error",
            "Corrected single-bit error",
            "Corrected error",
            "Uncorrectable error",
        ];
        for n in 0u8..0x0E {
            assert_eq!(sample[n as usize], format!("{:#}", ErrorType::from(n)));
        }
    }

    #[test]
    fn memory_error_32() {
        use super::*;
        use crate::{InfoType, RawStructure};

        let length = 23;
        let (data, strings) =
            include_bytes!("../../tests/data/caf65269/entries/18-0/bin")[4..].split_at(length as usize - 4);
        let structure = RawStructure {
            version: (2, 4).into(),
            info: InfoType::MemoryError32,
            length,
            handle: 0x01E3,
            data,
            strings,
        };
        let sample = MemoryError32 {
            handle: 0x01E3,
            error_type: ErrorType::Ok,
            error_granularity: ErrorGranularity::Unknown,
            error_operation: ErrorOperation::Unknown,
            vendor_syndrome: 0x00,
            memory_array_error_address: 0x8000_0000,
            device_error_address: 0x8000_0000,
            error_resolution: 0x8000_0000,
        };
        let result = MemoryError32::try_from(structure).unwrap();
        assert_eq!(sample, result);
    }
}
