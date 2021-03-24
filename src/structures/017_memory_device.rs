//! Memory Device (Type 17)
//!
//! This structure describes a single memory device that is part of a larger [Physical Memory
//! Array](super::physical_memory_array "structures::physical_memory_array") (Type 16)
//! structure.


use crate::{
    InfoType,
    MalformedStructureError::{
        self,
        InvalidFormattedSectionLength,
    },
    RawStructure,
};


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorGranularity {
    Other,
    Unknown,
    DeviceLevel,
    MemoryLevel,
    Undefined(u8),
}

impl From<u8> for ErrorGranularity {
    fn from(_type: u8) -> ErrorGranularity {
        match _type {
            1 => ErrorGranularity::Other,
            2 => ErrorGranularity::Unknown,
            3 => ErrorGranularity::DeviceLevel,
            4 => ErrorGranularity::MemoryLevel,
            t => ErrorGranularity::Undefined(t),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorOperation {
    Other,
    Unknown,
    Read,
    Write,
    PartialWrite,
    Undefined(u8),
}

impl From<u8> for ErrorOperation {
    fn from(_type: u8) -> ErrorOperation {
        match _type {
            1 => ErrorOperation::Other,
            2 => ErrorOperation::Unknown,
            3 => ErrorOperation::Read,
            4 => ErrorOperation::Write,
            5 => ErrorOperation::PartialWrite,
            t => ErrorOperation::Undefined(t),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorType {
    Other,
    Unknown,
    Ok,
    BadRead,
    Parity,
    SingleBit,
    DoubleBit,
    MultiBit,
    Nibble,
    Checksum,
    Crc,
    CorrectedSingleBit,
    Corrected,
    Uncorrectable,
    Undefined(u8),
}

impl From<u8> for ErrorType {
    fn from(_type: u8) -> ErrorType {
        match _type {
            1 => ErrorType::Other,
            2 => ErrorType::Unknown,
            3 => ErrorType::Ok,
            4 => ErrorType::BadRead,
            5 => ErrorType::Parity,
            6 => ErrorType::SingleBit,
            7 => ErrorType::DoubleBit,
            8 => ErrorType::MultiBit,
            9 => ErrorType::Nibble,
            10 => ErrorType::Checksum,
            11 => ErrorType::Crc,
            12 => ErrorType::CorrectedSingleBit,
            13 => ErrorType::Corrected,
            14 => ErrorType::Uncorrectable,
            t => ErrorType::Undefined(t),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum FormFactor {
    Other,
    Unknown,
    Simm,
    Sip,
    Chip,
    Dip,
    Zip,
    ProprietaryCard,
    Dimm,
    Tsop,
    RowOfChips,
    Rimm,
    SoDimm,
    Srimm,
    FbDimm,
    Undefined(u8),
}

impl Default for FormFactor {
    fn default() -> Self {
        FormFactor::Unknown
    }
}

impl From<u8> for FormFactor {
    fn from(_type: u8) -> FormFactor {
        match _type {
            0 => FormFactor::Other,
            1 => FormFactor::Unknown,
            2 => FormFactor::Simm,
            3 => FormFactor::Sip,
            4 => FormFactor::Chip,
            5 => FormFactor::Dip,
            6 => FormFactor::Zip,
            7 => FormFactor::ProprietaryCard,
            8 => FormFactor::Dimm,
            9 => FormFactor::Dimm,
            10 => FormFactor::Tsop,
            11 => FormFactor::RowOfChips,
            12 => FormFactor::Rimm,
            13 => FormFactor::SoDimm,
            14 => FormFactor::Srimm,
            15 => FormFactor::FbDimm,
            t => FormFactor::Undefined(t),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MemoryTechnology {
    Other,
    Unknown,
    Dram,
    NvDimmN,
    NvDimmF,
    NvDimmP,
    IntelOptane,
    Undefined(u8),
}

impl Default for MemoryTechnology {
    fn default() -> Self {
        MemoryTechnology::Unknown
    }
}

impl From<u8> for MemoryTechnology {
    fn from(_type: u8) -> MemoryTechnology {
        match _type {
            1 => MemoryTechnology::Other,
            2 => MemoryTechnology::Unknown,
            3 => MemoryTechnology::Dram,
            4 => MemoryTechnology::NvDimmN,
            5 => MemoryTechnology::NvDimmF,
            6 => MemoryTechnology::NvDimmP,
            7 => MemoryTechnology::IntelOptane,
            t => MemoryTechnology::Undefined(t),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Other,
    Unknown,
    Dram,
    Edram,
    Vram,
    Sram,
    Ram,
    Rom,
    Flash,
    Eeprom,
    Feprom,
    Eprom,
    Cdram,
    ThreeDram,
    Sdram,
    Sgram,
    Rdram,
    Ddr,
    Ddr2,
    Ddr2FbDimm,
    Reserved,
    Ddr3,
    Fbd2,
    Ddr4,
    LpDdr,
    LpDdr2,
    LpDdr3,
    Undefined(u8),
}

impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}

impl From<u8> for Type {
    fn from(_type: u8) -> Type {
        match _type {
            1 => Type::Other,
            2 => Type::Unknown,
            3 => Type::Dram,
            4 => Type::Edram,
            5 => Type::Vram,
            6 => Type::Sram,
            7 => Type::Ram,
            8 => Type::Rom,
            9 => Type::Flash,
            10 => Type::Eeprom,
            11 => Type::Feprom,
            12 => Type::Eprom,
            13 => Type::Cdram,
            14 => Type::ThreeDram,
            15 => Type::Sdram,
            16 => Type::Sgram,
            17 => Type::Rdram,
            18 => Type::Ddr,
            19 => Type::Ddr2,
            20 => Type::Ddr2FbDimm,
            21 => Type::Reserved,
            22 => Type::Reserved,
            23 => Type::Reserved,
            24 => Type::Ddr3,
            25 => Type::Fbd2,
            26 => Type::Ddr4,
            27 => Type::LpDdr,
            28 => Type::LpDdr2,
            29 => Type::LpDdr3,
            t => Type::Undefined(t),
        }
    }
}

bitflags! {
    /// The memory device details
    pub struct Detail: u16 {
        const RESERVED =      0b0000000000000000;
        const OTHER =         0b0000000000000010;
        const UNKNOWN =       0b0000000000000100;
        const FAST_PAGED =    0b0000000000001000;
        const STATIC_COLUMN = 0b0000000000010000;
        const PSEUDO_STATIC = 0b0000000000100000;
        const RAMBUS =        0b0000000001000000;
        const SYNCHRONOUS =   0b0000000010000000;
        const CMOS =          0b0000000100000000;
        const EDO =           0b0000001000000000;
        const WINDOW_DRAM =   0b0000010000000000;
        const CACHE_DRAM =    0b0000100000000000;
        const NON_VOLATILE =  0b0001000000000000;
        const REGISTERED =    0b0010000000000000;
        const UNREGISTERED =  0b0100000000000000;
        const LRDIMM =        0b1000000000000000;
    }
}

impl Default for Detail {
    fn default() -> Self {
        Detail::UNKNOWN
    }
}

bitflags! {
    pub struct OperatingModes: u16 {
        const RESERVED =                    0b0000000000000000;
        const OTHER =                       0b0000000000000010;
        const UNKNOWN =                     0b0000000000000100;
        const VOLATILE =                    0b0000000000001000;
        const BYTE_ACCESSIBLE_PERSISTENT =  0b0000000000010000;
        const BLOCK_ACCESSIBLE_PERSISTENT = 0b0000000000100000;
    }
}

/// The `Memory Device` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct MemoryDevice<'buffer> {
    pub handle: u16,
    pub physical_memory_handle: u16,
    pub memory_error_handle: Option<u16>,
    /// Total width, in bits, of this memory device, including any check
    /// or error-correction bits. If there are no error-correction bits,
    /// this value should be equal to Data Width
    pub total_width: Option<u16>,
    /// Data width, in bits, of this memory device. A Data Width of 0 and a
    /// Total Width of 8 indicates that the device is being used solely to
    /// provide 8 error-correction bits
    pub data_width: Option<u16>,
    /// Size of the memory device. If the size is 32GB-1MB or greater, the
    /// field value is 7FFFh and the actual size is stored in the extended_size
    /// field.
    pub size: Option<u16>,
    pub form_factor: FormFactor,
    /// Identifies when the Memory Device is one of a set of Memory Devices
    /// that must be populated with all devices of the same type and size,
    /// and the set to which this device belongs
    pub device_set: Option<u8>,
    /// Identifies the physically-labeled socket or board position
    /// where the memory device is located
    pub device_locator: &'buffer str,
    /// Identifies the physically labeled bank where the memory device is located
    pub bank_locator: &'buffer str,
    pub memory_type: Type,
    pub type_detail: Detail,
    /// Identifies the maximum capable speed of the device, in megatransfers
    /// per second (MT/s)
    pub speed: Option<u16>,
    pub manufacturer: &'buffer str,
    pub serial: &'buffer str,
    pub asset_tag: &'buffer str,
    pub part_number: &'buffer str,
    pub attributes: u8,
    /// Extended size of the memory device (complements the Size field)
    pub extended_size: u32,
    /// Identifies the configured speed of the memory device, in
    /// megatransfers per second (MT/s)
    pub configured_memory_speed: Option<u16>,
    /// Minimum operating voltage for this device, in millivolts
    pub minimum_voltage: Option<u16>,
    /// Maximum operating voltage for this device, in millivolts
    pub maximum_voltage: Option<u16>,
    /// Configured voltage for this device, in millivolts
    pub configured_voltage: Option<u16>,
    /// Memory technology type for this memory device
    pub memory_technology: Option<MemoryTechnology>,
    /// The operating modes supported by this memory device
    pub operating_mode_capability: Option<OperatingModes>,
    pub firmware_version: Option<&'buffer str>,
    /// The two-byte module manufacturer ID found in the SPD of this memory
    /// device; LSB first.
    pub module_manufacturer: Option<u16>,
    /// The two-byte module product ID found in the SPD of this memory device;
    /// LSB first
    pub module_product_id: Option<u16>,
    /// The two-byte memory subsystem controller manufacturer ID found in the
    /// SPD of this memory device; LSB first
    pub memory_subsystem_controller_manufacturer_id: Option<u16>,
    /// The two-byte memory subsystem controller product ID found in the SPD
    /// of this memory device; LSB first
    pub memory_subsystem_controller_product_id: Option<u16>,
    /// Size of the Non-volatile portion of the memory device in Bytes, if any
    pub non_volatile_size: Option<u64>,
    /// Size of the Volatile portion of the memory device in Bytes, if any
    pub volatile_size: Option<u64>,
    /// Size of the Cache portion of the memory device in Bytes, if any.
    pub cache_size: Option<u64>,
    /// Size of the Logical memory device in Bytes
    pub logical_size: Option<u64>,
    /// Identifies the maximum capable speed of the device, in megatransfers per second
    pub extended_speed: Option<u32>,
    /// Identifies the configured speed of the memory device, in megatransfers per second
    pub extended_configured_memory_speed: Option<u32>,
}

impl<'a> MemoryDevice<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<MemoryDevice<'a>, MalformedStructureError> {
        let data_len = structure.data.len() + 4;
        let handle = structure.handle;
        match ((structure.version.major, structure.version.minor), data_len) {
            (v, l) if v == (2, 1) && l != 0x15 => {
               Err(InvalidFormattedSectionLength(InfoType::MemoryDevice, handle, "", 0x15))
            },
            (v, l) if v == (2, 3) && l != 0x1B => {
               Err(InvalidFormattedSectionLength(InfoType::MemoryDevice, handle, "", 0x1B))
            },
            (v, l) if v == (2, 6) && l != 0x1C => {
               Err(InvalidFormattedSectionLength(InfoType::MemoryDevice, handle, "", 0x1C))
            },
            (v, l) if v == (2, 7) && l != 0x22 => {
               Err(InvalidFormattedSectionLength(InfoType::MemoryDevice, handle, "", 0x22))
            },
            (v, l) if v == (2, 8) && l != 0x28 => {
               Err(InvalidFormattedSectionLength(InfoType::MemoryDevice, handle, "", 0x28))
            },
            (v, l) if v == (3, 2) && l != 0x54 => {
               Err(InvalidFormattedSectionLength(InfoType::MemoryDevice, handle, "", 0x54))
            },
            (v, l) if v >= (3, 3) && l != 0x5C => {
               Err(InvalidFormattedSectionLength(InfoType::MemoryDevice, handle, "", 0x5C))
            },
            _ => {
                Ok(MemoryDevice {
                    handle,
                    physical_memory_handle: structure.get::<u16>(0x04)?,
                    memory_error_handle: structure.get::<u16>(0x06)
                        .ok().filter(|v| v != &0xFFFE),
                    total_width: structure.get::<u16>(0x08)
                        .ok().filter(|v| v != &0xFFFF),
                    data_width: structure.get::<u16>(0x0A)
                        .ok().filter(|v| v != &0xFFFF),
                    size: structure.get::<u16>(0x0C)
                        .ok().filter(|v| v != &0xFFFF),
                    form_factor: structure.get::<u8>(0x0E)?.into(),
                    device_set: structure.get::<u8>(0x0F)?.into(),
                    device_locator: structure.get_string(0x10)?,
                    bank_locator: structure.get_string(0x11)?,
                    memory_type: structure.get::<u8>(0x12)?.into(),
                    type_detail: Detail::from_bits_truncate(structure.get::<u16>(0x13)?),
                    speed: structure.get::<u16>(0x15)
                        .ok().filter(|v| v != &0x0000),
                    manufacturer: structure.get_string(0x17)?,
                    serial: structure.get_string(0x18)?,
                    asset_tag: structure.get_string(0x19)?,
                    part_number: structure.get_string(0x1A)?,
                    attributes: structure.get::<u8>(0x1B)?,
                    extended_size: structure.get::<u32>(0x1C)?,
                    configured_memory_speed: structure.get::<u16>(0x20)
                        .ok().filter(|v| v != &0x0000),
                    minimum_voltage: structure.get::<u16>(0x22)
                        .ok().filter(|v| v != &0x0000),
                    maximum_voltage: structure.get::<u16>(0x24)
                        .ok().filter(|v| v != &0x0000),
                    configured_voltage: structure.get::<u16>(0x26)
                        .ok().filter(|v| v != &0x0000),
                    memory_technology: structure.get::<u8>(0x28)
                        .ok().map(Into::into),
                    operating_mode_capability: structure.get::<u16>(0x29)
                        .ok().map(OperatingModes::from_bits_truncate),
                    firmware_version: structure.get_string(0x2B).ok(),
                    module_manufacturer: structure.get::<u16>(0x2C).ok(),
                    module_product_id: structure.get::<u16>(0x2E).ok(),
                    memory_subsystem_controller_manufacturer_id: structure.get::<u16>(0x30).ok(),
                    memory_subsystem_controller_product_id: structure.get::<u16>(0x32).ok(),
                    non_volatile_size: structure.get::<u64>(0x34).ok(),
                    volatile_size: structure.get::<u64>(0x3C).ok(),
                    cache_size: structure.get::<u64>(0x44).ok(),
                    logical_size: structure.get::<u64>(0x4C).ok(),
                    extended_speed: structure.get::<u32>(0x54).ok(),
                    extended_configured_memory_speed: structure.get::<u32>(0x58).ok(),
                })
            },
        }
    }
}
