//! System Configuration Options (Type 12)
//!
//! This SMBIOS structure contains information required to configure the baseboard’s Jumpers and
//! Switches.

use crate::{
    InfoType,
    MalformedStructureError::{self, InvalidStringIndex},
    RawStructure, StructureStrings,
};

/// Contains an iterator through configuration strings
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SystemConfigurationOptions<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Configuration strings
    pub strings: StructureStrings<'a>,
}

impl<'a> SystemConfigurationOptions<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let count: u8 = structure.get::<u8>(0x04)?;
        let strings = structure.strings();
        if count as usize != strings.count() {
            Err(InvalidStringIndex(
                InfoType::SystemConfigurationOptions,
                structure.handle,
                count,
            ))
        } else {
            Ok(SystemConfigurationOptions {
                handle: structure.handle,
                strings,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq as pretty_assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn system_configuration_options() {
        use super::*;
        use crate::{InfoType, RawStructure};

        let sample = vec!["ConfigOptions1", "ConfigOptions2", "ConfigOptions3"];
        let structure = RawStructure {
            version: (3, 4).into(),
            info: InfoType::SystemConfigurationOptions,
            length: 0x05,
            handle: 0x001F,
            // Remove 4 bytes from `dmidecode -H 8 -u` 'Header and Data'
            data: &[
                0x03, // Strings count
            ],
            strings: &[
                // ConfigOptions1
                0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x4F, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x73, 0x31,
                0x00, // ConfigOptions2
                0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x4F, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x73, 0x32,
                0x00, // ConfigOptions3
                0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x4F, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x73, 0x33, 0x00,
            ],
        };
        let result = SystemConfigurationOptions::try_from(structure).unwrap();

        pretty_assert_eq!(sample, result.strings.collect::<Vec<_>>());
    }

    #[test]
    fn dmi_bin() {
        use super::*;
        use crate::{EntryPoint, Structure, StructureStrings};
        const DMIDECODE_BIN: &[u8] = include_bytes!("../../tests/data/dmi.0.bin");
        let entry_point = EntryPoint::search(DMIDECODE_BIN).unwrap();
        let oem_strings = entry_point
            .structures(&DMIDECODE_BIN[(entry_point.smbios_address() as usize)..])
            .filter_map(|s| {
                if let Err(ref s) = s {
                    println!("{s}");
                }
                s.ok().filter(|s| matches!(s, Structure::SystemConfigurationOptions(_)))
            })
            .collect::<Vec<_>>();

        let sample = SystemConfigurationOptions {
            handle: 0x0C00,
            strings: StructureStrings::new(&[
                // NVRAM_CLR: Clear user settable NVRAM areas and set defaults
                0x4E, 0x56, 0x52, 0x41, 0x4D, 0x5F, 0x43, 0x4C, 0x52, 0x3A, 0x20, 0x43, 0x6C, 0x65, 0x61, 0x72, 0x20,
                0x75, 0x73, 0x65, 0x72, 0x20, 0x73, 0x65, 0x74, 0x74, 0x61, 0x62, 0x6C, 0x65, 0x20, 0x4E, 0x56, 0x52,
                0x41, 0x4D, 0x20, 0x61, 0x72, 0x65, 0x61, 0x73, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x73, 0x65, 0x74, 0x20,
                0x64, 0x65, 0x66, 0x61, 0x75, 0x6C, 0x74, 0x73, 0x00, // PWRD_EN: Close to enable password
                0x50, 0x57, 0x52, 0x44, 0x5F, 0x45, 0x4E, 0x3A, 0x20, 0x43, 0x6C, 0x6F, 0x73, 0x65, 0x20, 0x74, 0x6F,
                0x20, 0x65, 0x6E, 0x61, 0x62, 0x6C, 0x65, 0x20, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6F, 0x72, 0x64, 0x00,
                0x00,
            ]),
        };
        let result = oem_strings
            .iter()
            .find_map(|s| match s {
                Structure::SystemConfigurationOptions(os) => Some(os),
                _ => None,
            })
            .unwrap();
        pretty_assert_eq!(&sample, result, "Sample\n{:?}\nResult\n{:?}", sample, result);

        let string_sample = vec![
            "NVRAM_CLR: Clear user settable NVRAM areas and set defaults",
            "PWRD_EN: Close to enable password",
        ];
        pretty_assert_eq!(string_sample, result.strings.collect::<Vec<_>>(), "Strings");
    }
}
