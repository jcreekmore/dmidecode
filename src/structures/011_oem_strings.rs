//! OEM Strings (Type 11)
//!
//! This SMBIOS structure contains free-form strings defined by the OEM. Examples of this are part
//! numbers for system reference documents, contact information for the manufacturer, etc.


use crate::{
    MalformedStructureError,
    RawStructure,
    StructureStrings,
};

/// Contains free-form strings defined by the OEM
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OemStrings<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// OEM defined strings
    pub strings: StructureStrings<'a>
}


impl<'a> OemStrings<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let strings = structure.strings();
        Ok(OemStrings {
            handle: structure.handle,
            strings,
        })
    }
}


#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    use pretty_assertions::{assert_eq,};

    #[test]
    fn oem_strings() {
        use crate::{InfoType, RawStructure};
        use super::*;

        let sample = vec!["$HUA001UK10000","$HUA0464","$XXX0000"];
        let structure = RawStructure {
            version: (3, 4).into(),
            info: InfoType::OemStrings,
            length: 0x05,
            handle: 0x001E,
            // Remove 4 bytes from `dmidecode -H 8 -u` 'Header and Data'
            data: &[ 
                0x03, // Strings count
            ],
            strings: &[
                // $HUA001UK10000
                0x24, 0x48, 0x55, 0x41, 0x30, 0x30, 0x31, 0x55, 0x4B, 0x31, 0x30, 0x30, 0x30, 0x30,
                0x00,
                // $HUA0464
                0x24, 0x48, 0x55, 0x41, 0x30, 0x34, 0x36, 0x34,
                0x00,
                // $XXX0000
                0x24, 0x58, 0x58, 0x58, 0x30, 0x30, 0x30, 0x30,
                0x00, 0x00
            ],
        };
        let result = OemStrings::try_from(structure)
            .unwrap();

        assert_eq!(sample, result.strings.collect::<Vec<_>>());
    }

    #[test]
    fn dmi_bin() {
        use crate::{Structure, EntryPoint, StructureStrings};
        use super::*;
        const DMIDECODE_BIN: &'static [u8] = include_bytes!("../../tests/data/dmi.0.bin");
        let entry_point = EntryPoint::search(DMIDECODE_BIN).unwrap();
        let oem_strings = entry_point
            .structures(&DMIDECODE_BIN[(entry_point.smbios_address() as usize)..])
            .filter_map(|s| {
                if let Err(ref s) = s {
                    println!("{}", s);
                }
                s.ok().filter(|s| matches!(s, Structure::OemStrings(_)))
            })
        .collect::<Vec<_>>();

        let sample = OemStrings {
            handle: 0x0B00,
            strings: StructureStrings::new(&[
                // Dell System
                0x44, 0x65, 0x6C, 0x6C, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x00,
                // 5[0000]
                0x35, 0x5B, 0x30, 0x30, 0x30, 0x30, 0x5D, 0x00,
                // 14[1]
                0x31, 0x34, 0x5B, 0x31, 0x5D, 0x00,
                // 26[0]
                0x32, 0x36, 0x5B, 0x30, 0x5D, 0x00,
                // 17[20106865E85AE75B]
                0x31, 0x37, 0x5B, 0x32, 0x30, 0x31, 0x30, 0x36, 0x38, 0x36, 0x35, 0x45, 0x38, 0x35,
                0x41, 0x45, 0x37, 0x35, 0x42, 0x5D, 0x00,
                // 17[201559E55BE4282A]
                0x31, 0x37, 0x5B, 0x32, 0x30, 0x31, 0x35, 0x35, 0x39, 0x45, 0x35, 0x35, 0x42, 0x45,
                0x34, 0x32, 0x38, 0x32, 0x41, 0x5D, 0x00,
                // 18[0]
                0x31, 0x38, 0x5B, 0x30, 0x5D, 0x00,
                // 19[1]
                0x31, 0x39, 0x5B, 0x31, 0x5D, 0x00,
                // 19[1]
                0x31, 0x39, 0x5B, 0x31, 0x5D, 0x00,
                // 
                0x00,
            ]), 
        };
        let result = oem_strings.iter()
            .find_map(|s| {
                match s {
                    Structure::OemStrings(os) => Some(os),
                    _ => None,
                }
            }).unwrap();
        assert_eq!(&sample, result, "Sample\n{:?}\nResult\n{:?}", sample, result);

        let string_sample = vec![
            "Dell System",
            "5[0000]",
            "14[1]",
            "26[0]",
            "17[20106865E85AE75B]",
            "17[201559E55BE4282A]",
            "18[0]",
            "19[1]",
            "19[1]",
        ];
        assert_eq!(string_sample, result.strings.collect::<Vec<_>>(), "Strings"); 
    }
}
