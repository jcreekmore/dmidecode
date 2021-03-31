//! Memory Device Mapped Address (Type 20)
//!
//! This structure maps memory address space usually to a device-level granularity.\
//! One structure is present for each contiguous address range described.


use crate::{
    InfoType,
    MalformedStructureError::{
        self,
        InvalidFormattedSectionLength,
    },
    RawStructure,
};


/// Main struct for *Memory Device Mapped Address (Type 20)*
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MemoryDeviceMappedAddress {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Physical address, in kilobytes, of a range of memory mapped to the referenced Memory
    /// Device.\
    /// When the field value is FFFF FFFFh the actual address is stored in the Extended Starting
    /// Address field. When this field contains a valid address, Ending Address must also contain a
    /// valid address. When this field contains FFFF FFFFh, Ending Address must also contain FFFF
    /// FFFFh.
    pub starting_address: u32,
    /// Physical ending address of the last kilobyte of a range of addresses mapped to the
    /// referenced Memory Device.\
    /// When the field value is FFFF FFFFh the actual address is stored in the Extended Ending
    /// Address field. When this field contains a valid address, Starting Address must also contain
    /// a valid address.
    pub ending_address: u32,
    /// Handle, or instance number, associated with the Memory Device structure to which this
    /// address range is mapped.\
    /// Multiple address ranges can be mapped to a single Memory Device.
    pub memory_device_handle: u16,
    /// Handle, or instance number, associated with the Memory Array Mapped Address structure to
    /// which this device address range is mapped.\
    /// Multiple address ranges can be mapped to a single Memory Array Mapped Address.
    pub memory_array_mapped_address_handle: u16,
    /// Position of the referenced Memory Device in a row of the address partition.\
    /// For example, if two 8-bit devices form a 16-bit row, this field’s value is either 1 or 2.\
    /// The value 0 is reserved. If the position is unknown, the field contains FFh.
    pub partition_row_position: u8,
    /// Position of the referenced Memory Device in an interleave.\
    /// The value 0 indicates non-interleaved, 1 indicates first interleave position, 2 the second
    /// interleave position, and so on. If the position is unknown, the field contains FFh.
    pub interleave_position: u8,
    /// Maximum number of consecutive rows from the referenced Memory Device that are accessed in a
    /// single interleaved transfer.\
    /// If the device is not part of an interleave, the field contains 0; if the interleave
    /// configuration is unknown, the value is FFh.
    pub interleaved_data_depth: u8,
    /// Physical address, in bytes, of a range of memory mapped to the referenced Memory Device.\
    /// This field is valid when Starting Address contains the value FFFF FFFFh. If Starting
    /// Address contains a value other than FFFF FFFFh, this field contains zeros. When this field
    /// contains a valid address, Extended Ending Address must also contain a valid address.
    pub extended_starting_address: Option<u64>,
    /// Physical ending address, in bytes, of the last of a range of addresses mapped to the
    /// referenced Memory Device.\
    /// This field is valid when both Starting Address and Ending Address contain the value FFFF
    /// FFFFh. If Ending Address contains a value other than FFFF FFFFh, this field contains zeros.
    /// When this field contains a valid address, Extended Starting Address must also contain a
    /// valid address.
    pub extended_ending_address: Option<u64>,
}


impl<'a> MemoryDeviceMappedAddress {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let handle = structure.handle;
        match (structure.version.major, structure.version.minor) {
            v if ((2, 1)..(2, 7)).contains(&v) && structure.length != 0x13 =>
                Err(InvalidFormattedSectionLength(InfoType::MemoryDeviceMappedAddress, handle, "", 0x13)),
            v if v >= (2, 7) && structure.length != 0x23 =>
                Err(InvalidFormattedSectionLength(InfoType::MemoryDeviceMappedAddress, handle, "", 0x23)),
            _ => {
                Ok(Self {
                    handle,
                    starting_address: structure.get::<u32>(0x04)?,
                    ending_address: structure.get::<u32>(0x08)?,
                    memory_device_handle: structure.get::<u16>(0x0C)?,
                    memory_array_mapped_address_handle: structure.get::<u16>(0x0E)?,
                    partition_row_position: structure.get::<u8>(0x10)?,
                    interleave_position: structure.get::<u8>(0x11)?,
                    interleaved_data_depth: structure.get::<u8>(0x12)?,
                    extended_starting_address: structure.get::<u64>(0x13).ok(),
                    extended_ending_address: structure.get::<u64>(0x1B).ok(),
                })
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    use pretty_assertions::{assert_eq,};

    #[test]
    fn memory_device_mapped_address() {
        use crate::{
            InfoType,
            RawStructure,
        };
        use super::*;

        let length = 35;
        let (data, strings) = include_bytes!("../../tests/data/02daadcd/entries/20-0/bin")[4..]
            .split_at(length as usize - 4);
        let structure = RawStructure {
            version: (2, 7).into(),
            info: InfoType::MemoryDeviceMappedAddress,
            length,
            handle: 0x0029,
            data,
            strings,
        };
        let sample = MemoryDeviceMappedAddress {
            handle: 0x0029,
            starting_address: 0,
            ending_address: 0xFFFFFF,
            memory_device_handle: 0x0028,
            memory_array_mapped_address_handle: 0x0027,
            partition_row_position: 0,
            interleave_position: 0xFF,
            interleaved_data_depth: 0xFF,
            extended_starting_address: Some(0),
            extended_ending_address: Some(0),
        };
        let result = MemoryDeviceMappedAddress::try_from(structure)
            .unwrap();
        assert_eq!(sample, result, "MemoryDeviceMappedAddress");
    }
}
