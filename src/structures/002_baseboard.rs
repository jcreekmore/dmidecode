//! Baseboard (or Module) Information (Type 2)
//!
//! The information in this structure defines attributes of a system baseboard (for example, a
//! motherboard, planar, server blade, or other standard system module).
use core::fmt;

use bitflags::bitflags;

use crate::{MalformedStructureError, RawStructure};

/// The baseboard type defined in the SMBIOS specification.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BoardType {
    Unknown,
    Other,
    ServerBlade,
    ConnectivitySwitch,
    SystemManagementModule,
    ProcessorModule,
    IoModule,
    MemoryModule,
    DaughterBoard,
    MotherBoard,
    ProcessorMemoryModule,
    ProcessorIoModule,
    InterconnectBoard,
    Undefined(u8),
}

impl From<u8> for BoardType {
    fn from(_type: u8) -> BoardType {
        match _type {
            1 => BoardType::Unknown,
            2 => BoardType::Other,
            3 => BoardType::ServerBlade,
            4 => BoardType::ConnectivitySwitch,
            5 => BoardType::SystemManagementModule,
            6 => BoardType::ProcessorModule,
            7 => BoardType::IoModule,
            8 => BoardType::MemoryModule,
            9 => BoardType::DaughterBoard,
            10 => BoardType::MotherBoard,
            11 => BoardType::ProcessorMemoryModule,
            12 => BoardType::ProcessorIoModule,
            13 => BoardType::InterconnectBoard,
            t => BoardType::Undefined(t),
        }
    }
}
impl fmt::Display for BoardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardType::Unknown => write!(f, "Unknown"),
            BoardType::Other => write!(f, "Other"),
            BoardType::ServerBlade => write!(f, "Server Blade"),
            BoardType::ConnectivitySwitch => write!(f, "Connectivity Switch"),
            BoardType::SystemManagementModule => write!(f, "System Management Module"),
            BoardType::ProcessorModule => write!(f, "Processor Module"),
            BoardType::IoModule => write!(f, "I/O Module"),
            BoardType::MemoryModule => write!(f, "Memory Module"),
            BoardType::DaughterBoard => write!(f, "Daughter board"),
            BoardType::MotherBoard => {
                write!(f, "Motherboard (includes processor, memory, and I/O)")
            }
            BoardType::ProcessorMemoryModule => write!(f, "Processor/Memory Module"),
            BoardType::ProcessorIoModule => write!(f, "Processor/IO Module"),
            BoardType::InterconnectBoard => write!(f, "Interconnect board"),
            BoardType::Undefined(t) => write!(f, "Undefined: {}", t),
        }
    }
}

bitflags! {
    /// The baseboard characteristic flags defined in the SMBIOS specification.
    pub struct BaseBoardFlags: u8 {
        const HOSTING = 0b0000_0001;
        const REQUIRES_DAUGHTER = 0b0000_0010;
        const IS_REMOVABLE = 0b0000_0100;
        const IS_REPLACEABLE = 0b0000_1000;
        const IS_HOT_SWAPPABLE = 0b0001_0000;
    }
}

/// The `BaseBoard` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BaseBoard<'buffer> {
    pub handle: u16,
    pub manufacturer: &'buffer str,
    pub product: &'buffer str,
    pub version: &'buffer str,
    pub serial: &'buffer str,
    pub asset: Option<&'buffer str>,
    pub feature_flags: Option<BaseBoardFlags>,
    pub location_in_chassis: Option<&'buffer str>,
    pub chassis_handle: Option<u16>,
    pub board_type: Option<BoardType>,
}

impl<'buffer> BaseBoard<'buffer> {
    pub(crate) fn try_from(structure: RawStructure<'buffer>) -> Result<BaseBoard<'buffer>, MalformedStructureError> {
        #[repr(C)]
        #[repr(packed)]
        struct BaseBoardPacked {
            manufacturer: u8,
            product: u8,
            version: u8,
            serial: u8,
            asset: u8,
            feature_flags: u8,
            location_in_chassis: u8,
            chassis_handle: u16,
            board_type: u8,
        }

        let_as_struct!(packed, BaseBoardPacked, structure.data);

        let asset = if structure.data.len() > 4 {
            Some(structure.find_string(packed.asset)?)
        } else {
            None
        };
        let feature_flags = if structure.data.len() > 5 {
            Some(BaseBoardFlags::from_bits_truncate(packed.feature_flags))
        } else {
            None
        };
        let location_in_chassis = if structure.data.len() > 6 {
            Some(structure.find_string(packed.location_in_chassis)?)
        } else {
            None
        };
        let chassis_handle = if structure.data.len() > 7 {
            Some(packed.chassis_handle)
        } else {
            None
        };
        let board_type = if structure.data.len() > 9 {
            Some(packed.board_type.into())
        } else {
            None
        };

        Ok(BaseBoard {
            handle: structure.handle,
            manufacturer: structure.find_string(packed.manufacturer)?,
            product: structure.find_string(packed.product)?,
            version: structure.find_string(packed.version)?,
            serial: structure.find_string(packed.serial)?,
            asset,
            feature_flags,
            location_in_chassis,
            chassis_handle,
            board_type,
        })
    }
}
