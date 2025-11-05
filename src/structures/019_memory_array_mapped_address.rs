//! Memory Array Mapped Address (Type 19)
//!
//! This structure provides the address mapping for a Physical Memory Array.
//! One structure is present for each contiguous address range described.

use crate::{
    InfoType,
    MalformedStructureError::{self, InvalidFormattedSectionLength},
    RawStructure,
};

/// Main struct for *Memory Array Mapped Address (Type 19)*
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemoryArrayMappedAddress {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Physical address, in kilobytes, of a range of memory mapped to the specified Physical
    /// Memory Array.\
    /// When the field value is FFFF FFFFh, the actual address is stored in the Extended Starting
    /// Address field. When this field contains a valid address, Ending Address must also contain a
    /// valid address. When this field contains FFFF FFFFh, Ending Address must also contain FFFF
    /// FFFFh
    pub starting_address: u32,
    /// Physical ending address of the last kilobyte of a range of addresses mapped to the
    /// specified Physical Memory Array.\
    /// When the field value is FFFF FFFFh and the Starting Address field also contains FFFF FFFFh,
    /// the actual address is stored in the Extended Ending Address field. When this field contains
    /// a valid address, Starting Address must also contain a valid address.
    pub ending_address: u32,
    /// Handle, or instance number, associated with the Physical Memory Array to which this address
    /// range is mapped.\
    /// Multiple address ranges can be mapped to a single Physical Memory Array.
    pub memory_array_handle: u16,
    /// Number of Memory Devices that form a single row of memory for the address partition defined
    /// by this structure.
    pub partition_width: u8,
    /// Physical address, in bytes, of a range of memory mapped to the specified Physical Memory
    /// Array.\
    /// This field is valid when Starting Address contains the value FFFF FFFFh. If Starting
    /// Address contains a value other than FFFF FFFFh, this field contains zeros. When this field
    /// contains a valid address, Extended Ending Address must also contain a valid address.
    pub extended_starting_address: Option<u64>,
    /// Physical ending address, in bytes, of the last of a range of addresses mapped to the
    /// specified Physical Memory Array.\
    /// This field is valid when both Starting Address and Ending Address contain the value FFFF
    /// FFFFh. If Ending Address contains a value other than FFFF FFFFh, this field contains zeros.
    /// When this field contains a valid address, Extended Starting Address must also contain a
    /// valid address.
    pub extended_ending_address: Option<u64>,
}

impl<'a> MemoryArrayMappedAddress {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let handle = structure.handle;
        match (structure.version.major, structure.version.minor) {
            v if ((2, 1)..(2, 7)).contains(&v) && structure.length != 0x0F => Err(InvalidFormattedSectionLength(
                InfoType::MemoryArrayMappedAddress,
                handle,
                "",
                0x0F,
            )),
            v if v >= (2, 7) && structure.length != 0x1F => Err(InvalidFormattedSectionLength(
                InfoType::MemoryArrayMappedAddress,
                handle,
                "",
                0x1F,
            )),
            _ => Ok(Self {
                handle,
                starting_address: structure.get::<u32>(0x04)?,
                ending_address: structure.get::<u32>(0x08)?,
                memory_array_handle: structure.get::<u16>(0x0C)?,
                partition_width: structure.get::<u8>(0x0E)?,
                extended_starting_address: structure.get::<u64>(0x0F).ok(),
                extended_ending_address: structure.get::<u64>(0x17).ok(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq as pretty_assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn memory_array_mapped_address() {
        use super::*;
        use crate::{InfoType, RawStructure};

        let length = 31;
        let (data, strings) =
            include_bytes!("../../tests/data/02daadcd/entries/19-0/bin")[4..].split_at(length as usize - 4);
        let structure = RawStructure {
            version: (2, 7).into(),
            info: InfoType::MemoryArrayMappedAddress,
            length,
            handle: 0x0027,
            data,
            strings,
        };
        let sample = MemoryArrayMappedAddress {
            handle: 0x0027,
            starting_address: 0,
            ending_address: 0x0207C000,
            memory_array_handle: 0x0026,
            partition_width: 255,
            extended_starting_address: Some(0),
            extended_ending_address: Some(0),
        };
        let result = MemoryArrayMappedAddress::try_from(structure).unwrap();
        pretty_assert_eq!(sample, result, "MemoryArrayMappedAddress");
    }
}
