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
    pub asset: &'buffer str,
    pub feature_flags: BaseBoardFlags,
    pub location_in_chassis: &'buffer str,
    pub chassis_handle: u16,
    pub board_type: BoardType,
}


impl<'buffer> BaseBoard<'buffer> {
    pub(crate) fn try_from(structure: super::RawStructure<'buffer>) -> Result<BaseBoard<'buffer>, super::MalformedStructureError> {
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

        Ok(BaseBoard {
            handle: structure.handle,
            manufacturer: structure.find_string(packed.manufacturer)?,
            product: structure.find_string(packed.product)?,
            version: structure.find_string(packed.version)?,
            serial: structure.find_string(packed.serial)?,
            asset: structure.find_string(packed.asset)?,
            feature_flags: BaseBoardFlags::from_bits_truncate(packed.feature_flags),
            location_in_chassis: structure.find_string(packed.location_in_chassis)?,
            chassis_handle: packed.chassis_handle,
            board_type: packed.board_type.into(),
        })
    }
}
