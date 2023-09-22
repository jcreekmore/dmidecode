//! Physical Memory Array (Type 16)
//!
//! This structure describes a collection of memory devices that operate together to form a memory
//! address space.

use core::convert::TryInto;
use core::fmt;

use crate::{MalformedStructureError, RawStructure};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MemoryArrayLocation {
    Other,
    Unknown,
    SystemBoardOrMotherboard,
    IsaAddOnCard,
    EisaAddOnCard,
    PciAddOnCard,
    McaAddOnCard,
    PcmciaAddOnCard,
    ProprietaryAddOnCard,
    NuBus,
    Pc98c20AddOnCard,
    Pc98c24AddOnCard,
    Pc98eAddOnCard,
    Pc98LocalBusAddOnCard,
    CxlAddOnCard,
    Undefined(u8),
}

impl Default for MemoryArrayLocation {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for MemoryArrayLocation {
    fn from(_type: u8) -> Self {
        match _type {
            1 => Self::Other,
            2 => Self::Unknown,
            3 => Self::SystemBoardOrMotherboard,
            4 => Self::IsaAddOnCard,
            5 => Self::EisaAddOnCard,
            6 => Self::PciAddOnCard,
            7 => Self::McaAddOnCard,
            8 => Self::PcmciaAddOnCard,
            9 => Self::ProprietaryAddOnCard,
            10 => Self::NuBus,
            11 => Self::Pc98c20AddOnCard,
            12 => Self::Pc98c24AddOnCard,
            13 => Self::Pc98eAddOnCard,
            14 => Self::Pc98LocalBusAddOnCard,
            15 => Self::CxlAddOnCard,
            t => Self::Undefined(t),
        }
    }
}

impl fmt::Display for MemoryArrayLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::SystemBoardOrMotherboard => write!(f, "System board or motherboard"),
            Self::IsaAddOnCard => write!(f, "ISA add-on card"),
            Self::EisaAddOnCard => write!(f, "EISA add-on card"),
            Self::PciAddOnCard => write!(f, "PCI add-on card"),
            Self::McaAddOnCard => write!(f, "MCA add-on card"),
            Self::PcmciaAddOnCard => write!(f, "PCMCIA add-on card"),
            Self::ProprietaryAddOnCard => write!(f, "Proprietary add-on card"),
            Self::NuBus => write!(f, "NuBus"),
            Self::Pc98c20AddOnCard => write!(f, "PC-98/C20 add-on card"),
            Self::Pc98c24AddOnCard => write!(f, "PC-98/C24 add-on card"),
            Self::Pc98eAddOnCard => write!(f, "PC-98/E add-on card"),
            Self::Pc98LocalBusAddOnCard => write!(f, "PC-98/Local bus add-on card"),
            Self::CxlAddOnCard => write!(f, "CXL add-on card"),
            Self::Undefined(t) => write!(f, "Undefined: {}", t),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MemoryArrayUse {
    Other,
    Unknown,
    SystemMemory,
    VideoMemory,
    FlashMemory,
    NonVolatileRam,
    CacheMemory,
    Undefined(u8),
}

impl Default for MemoryArrayUse {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for MemoryArrayUse {
    fn from(_type: u8) -> Self {
        match _type {
            1 => Self::Other,
            2 => Self::Unknown,
            3 => Self::SystemMemory,
            4 => Self::VideoMemory,
            5 => Self::FlashMemory,
            6 => Self::NonVolatileRam,
            7 => Self::CacheMemory,
            t => Self::Undefined(t),
        }
    }
}

impl fmt::Display for MemoryArrayUse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::SystemMemory => write!(f, "System memory"),
            Self::VideoMemory => write!(f, "Video memory"),
            Self::FlashMemory => write!(f, "Flash memory"),
            Self::NonVolatileRam => write!(f, "Non-volatile RAM"),
            Self::CacheMemory => write!(f, "Cache memory"),
            Self::Undefined(t) => write!(f, "Undefined: {}", t),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MemoryArrayErrorCorrectionTypes {
    Other,
    Unknown,
    None,
    Parity,
    SingleBitEcc,
    MultiBitEcc,
    CRC,
    Undefined(u8),
}

impl Default for MemoryArrayErrorCorrectionTypes {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for MemoryArrayErrorCorrectionTypes {
    fn from(_type: u8) -> Self {
        match _type {
            1 => Self::Other,
            2 => Self::Unknown,
            3 => Self::None,
            4 => Self::Parity,
            5 => Self::SingleBitEcc,
            6 => Self::MultiBitEcc,
            7 => Self::CRC,
            t => Self::Undefined(t),
        }
    }
}

impl fmt::Display for MemoryArrayErrorCorrectionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::None => write!(f, "None"),
            Self::Parity => write!(f, "Parity"),
            Self::SingleBitEcc => write!(f, "Single-bit ECC"),
            Self::MultiBitEcc => write!(f, "Multi-bit ECC"),
            Self::CRC => write!(f, "CRC"),
            Self::Undefined(t) => write!(f, "Undefined: {}", t),
        }
    }
}

/// The `Physical Memory Array` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct PhysicalMemoryArray {
    pub handle: u16,
    /// Physical location of the Memory Array, whether on the system board or an add-in board
    pub location: MemoryArrayLocation,
    /// Function for which the array is used
    pub r#use: MemoryArrayUse,
    /// Primary hardware error correction or detection method supported by this memory array
    pub memory_error_correction: MemoryArrayErrorCorrectionTypes,
    /// Maximum memory capacity, in kilobytes, for this array
    /// If the capacity is not represented in this field, then
    /// this field contains 8000 0000h and the Extended
    /// Maximum Capacity field should be used. Values 2
    /// TB (8000 0000h) or greater must be represented
    /// in the Extended Maximum Capacity field.
    pub maximum_capacity: Option<u32>,
    /// Handle, or instance number, associated with any
    /// error that was previously detected for the array
    pub memory_error_information_handle: Option<u16>,
    /// Number of slots or sockets available for Memory Devices in this array
    /// This value represents the number of Memory Device structures that compose this Memory
    /// Array. Each Memory Device has a reference to the “owning” Memory Array.
    pub number_of_memory_devices: u16,
    /// Maximum memory capacity, in bytes, for this array.
    /// This field is only valid when the Maximum Capacity field contains 8000 0000h.
    /// When Maximum Capacity contains a value that is not 8000 0000h, Extended Maximum Capacity must contain zeros.
    pub extended_maximum_capacity: Option<u64>,
}

impl PhysicalMemoryArray {
    pub(crate) fn try_from(structure: RawStructure) -> Result<Self, MalformedStructureError> {
        let mut pma = PhysicalMemoryArray::default();
        let mut mem_pointer = 0;
        if structure.version > (2, 1).into() {
            pma.handle = structure.handle;
            pma.location = MemoryArrayLocation::from(structure.data[mem_pointer]);
            mem_pointer += 1;
            pma.r#use = MemoryArrayUse::from(structure.data[mem_pointer]);
            mem_pointer += 1;
            pma.memory_error_correction = MemoryArrayErrorCorrectionTypes::from(structure.data[mem_pointer]);
            mem_pointer += 1;
            pma.maximum_capacity = get_optional_dword(&mut mem_pointer, structure.data, 0x80000000)?;
            pma.memory_error_information_handle = get_optional_word(&mut mem_pointer, structure.data, 0xFFFE)?;
            pma.number_of_memory_devices = get_word(&mut mem_pointer, structure.data)?;
        }
        if structure.version > (2, 7).into() {
            pma.extended_maximum_capacity = if pma.maximum_capacity.is_none() {
                get_optional_qword(&mut mem_pointer, structure.data, 0)?
            } else {
                None
            };
        }
        Ok(pma)
    }
}

fn get_optional_qword(pointer: &mut usize, data: &[u8], none_val: u64) -> Result<Option<u64>, MalformedStructureError> {
    let word = get_qword(pointer, data)?;
    if word == none_val {
        Ok(None)
    } else {
        Ok(Some(word))
    }
}

fn get_optional_dword(pointer: &mut usize, data: &[u8], none_val: u32) -> Result<Option<u32>, MalformedStructureError> {
    let word = get_dword(pointer, data)?;
    if word == none_val {
        Ok(None)
    } else {
        Ok(Some(word))
    }
}

fn get_optional_word(pointer: &mut usize, data: &[u8], none_val: u16) -> Result<Option<u16>, MalformedStructureError> {
    let word = get_word(pointer, data)?;
    if word == none_val {
        Ok(None)
    } else {
        Ok(Some(word))
    }
}

fn get_word(pointer: &mut usize, data: &[u8]) -> Result<u16, MalformedStructureError> {
    let word = u16::from_le_bytes(
        data[*pointer..(*pointer + 2)]
            .try_into()
            .map_err(MalformedStructureError::InvalidSlice)?,
    );
    *pointer += 2;
    Ok(word)
}

fn get_dword(pointer: &mut usize, data: &[u8]) -> Result<u32, MalformedStructureError> {
    let dword = u32::from_le_bytes(
        data[*pointer..(*pointer + 4)]
            .try_into()
            .map_err(MalformedStructureError::InvalidSlice)?,
    );
    *pointer += 4;
    Ok(dword)
}

fn get_qword(pointer: &mut usize, data: &[u8]) -> Result<u64, MalformedStructureError> {
    let qword = u64::from_le_bytes(
        data[*pointer..(*pointer + 8)]
            .try_into()
            .map_err(MalformedStructureError::InvalidSlice)?,
    );
    *pointer += 8;
    Ok(qword)
}
