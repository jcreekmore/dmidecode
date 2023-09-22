//! Group Associations (Type 14)
//!
//! The Group Associations SMBIOS structure is provided for OEMs who want to specify the arrangement or
//! hierarchy of certain components (including other Group Associations) within the system. For
//! example, you can use the Group Associations structure to indicate that two CPUs share a common
//! external cache system.

use crate::{
    InfoType,
    MalformedStructureError::{self, InvalidFormattedSectionLength},
    RawStructure, TryFromBytes,
};

/// Named group with member items
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GroupAssociations<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// String describing the group
    pub group_name: &'a str,
    /// Items iterator
    pub items: GroupItems<'a>,
}

/// An iterator through certain components
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GroupItems<'a> {
    data: &'a [u8],
    index: usize,
}

/// Group member
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GroupItem {
    /// Item (Structure) Type of this member
    pub type_: u8,
    /// Handle corresponding to this structure
    pub handle: u16,
}

impl<'a> GroupAssociations<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let handle = structure.handle;
        let slice =
            structure
                .get_slice(0x05, structure.length as usize - 0x05)
                .ok_or(InvalidFormattedSectionLength(
                    InfoType::GroupAssociations,
                    handle,
                    "",
                    structure.length,
                ))?;
        Ok(GroupAssociations {
            handle,
            group_name: structure.get_string(0x04)?,
            items: GroupItems::new(slice),
        })
    }
}

impl<'a> GroupItems<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, index: 0 }
    }
}
impl<'a> Iterator for GroupItems<'a> {
    type Item = GroupItem;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.index;
        let end = start + 3;
        let slice = self.data.get(start..end)?;
        self.index = end;
        let type_ = *slice.first()?;
        let handle = slice.get(1..).and_then(|s| u16::try_from_bytes(s).ok())?;
        Some(GroupItem { type_, handle })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn group_items() {
        use super::{GroupItem, GroupItems};

        let data = &[4, 0x00, 0x04, 7, 0x00, 0x07, 7, 0x01, 0x07, 7, 0x02, 0x07];
        let result = GroupItems::new(data);
        let sample = vec![
            GroupItem {
                type_: 4,
                handle: 0x0400,
            },
            GroupItem {
                type_: 7,
                handle: 0x0700,
            },
            GroupItem {
                type_: 7,
                handle: 0x0701,
            },
            GroupItem {
                type_: 7,
                handle: 0x0702,
            },
        ];

        assert_eq!(sample, result.collect::<Vec<_>>());
    }

    #[test]
    fn group_associations() {
        use super::*;
        use crate::{InfoType, RawStructure};

        let sample = vec![
            GroupItem { type_: 4, handle: 0x08 },
            GroupItem { type_: 4, handle: 0x0A },
            GroupItem { type_: 7, handle: 0x09 },
        ];
        let structure = RawStructure {
            version: (3, 4).into(),
            info: InfoType::GroupAssociations,
            length: 14,
            handle: 0x0028,
            // Remove 4 bytes from `dmidecode -H 8 -u` 'Header and Data'
            data: &[
                0x01, // String number
                0x04, // First CPU
                0x08, 0x00, // CPU Structure’s Handle
                0x04, // Second CPU
                0x0A, 0x00, // CPU Structure’s Handle
                0x07, // Shared cache
                0x09, 0x00, // Cache Structure’s Handle
            ],
            strings: &[
                // Dual-Processor CPU Complex
                0x44, 0x75, 0x61, 0x6c, 0x2d, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x20, 0x43, 0x50,
                0x55, 0x20, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x78, 0x00, 0x00,
            ],
        };
        let result = GroupAssociations::try_from(structure).unwrap();

        assert_eq!("Dual-Processor CPU Complex", result.group_name, "Group name");
        assert_eq!(sample, result.items.collect::<Vec<_>>(), "Items");
    }
}
