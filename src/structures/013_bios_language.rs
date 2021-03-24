//! BIOS Language Information (Type 13)
//!
//! The information in this structure defines the installable language attributes of the BIOS.

use crate::bitfield::{BitField, FlagType, Layout};
use crate::{MalformedStructureError, RawStructure,};


/// The `BIOS Language Information` table defined in the SMBIOS specification.
#[derive(Clone, Debug, Eq, Hash, PartialEq, )]
pub struct BiosLanguage<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Available languages
    pub installable_languages: InstallableLanguages<'a>,
    /// Flags
    pub flags: Option<LanguageFlags>,
    /// String number (one-based) of the currently installed language
    pub current_language: u8,
}

/// An iterator through available languages. Each available language has a description string.
#[derive(Clone, Debug, Eq, Hash, PartialEq, )]
pub struct InstallableLanguages<'a> {
    structure: RawStructure<'a>,
    index: u8,
}

/// BIOS Language flags
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct LanguageFlags(u8);


impl<'a> BiosLanguage<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<BiosLanguage<'a>, MalformedStructureError> {
        #[repr(C)]
        #[repr(packed)]
        struct BiosLanguagePacked_2_1 {
            installable_languages: u8,
            flags: u8,
            reserved: [u8; 15],
            current_language: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct BiosLanguagePacked_2_0 {
            installable_languages: u8,
            reserved: [u8; 15],
            current_language: u8,
        }

        match structure.version {
            v if v >= (2, 1).into() => {
                let_as_struct!(packed, BiosLanguagePacked_2_1, structure.data);
                Ok(BiosLanguage{
                    handle: structure.handle,
                    installable_languages: InstallableLanguages::new(structure),
                    flags: Some(LanguageFlags(packed.flags)),
                    current_language: packed.current_language,
                })
            },
            _ => {
                let_as_struct!(packed, BiosLanguagePacked_2_0, structure.data);
                Ok(BiosLanguage{
                    handle: structure.handle,
                    installable_languages: InstallableLanguages::new(structure),
                    flags: None,
                    current_language: packed.current_language,
                })

            },
        }
    }
}

impl<'a> InstallableLanguages<'a> {
    fn new(structure: RawStructure<'a>) -> Self {
        // String number is one-based
        Self { structure, index: 1 }
    }
}
impl<'a> Iterator for InstallableLanguages<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        self.structure.find_string(index).ok()
    }
}

impl<'a> BitField<'a> for LanguageFlags {
    type Size = u8;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 8;
        "Current Language strings use the abbreviated format"
            "If the bit is 0, each language string is in the form “ISO 639-1 Language Name | ISO \
            3166-1-alpha-2 Territory Name | Encoding Method”.\nIf the bit is 1, each language \
            string consists of the two-character “ISO 639-1 Language Name” directly followed by the \
            two-character “ISO 3166-1-alpha-2 Territory Name”.",
        "Reserved": 7,
    );
}


#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    use lazy_static::lazy_static;
    use pretty_assertions::{assert_eq,};
    
    use super::*;

    const DMIDECODE_BIN: &'static [u8] = include_bytes!("../../tests/data/dmi.0.bin");
    lazy_static! {
        static ref ENTRY_POINT: crate::EntryPoint = crate::EntryPoint::search(DMIDECODE_BIN).unwrap();
    }

    #[test]
    fn installable_languages() {
        use crate::InfoType;
        let sample = vec![
            "en|US|iso8859-1",
            "fr|FR|iso8859-1",
            "es|ES|iso8859-1",
            "de|DE|iso8859-1",
            "it|IT|iso8859-1",
            "da|DK|iso8859-1",
            "fi|FI|iso8859-1",
            "nl|NL|iso8859-1",
            "no|NO|iso8859-1",
            "pt|PT|iso8859-1",
            "sv|SE|iso8859-1",
            "ja|JP|unicode",
            "zh|CN|unicode",
        ];
        let structure = RawStructure {
            version: (0, 0).into(),
            info: InfoType::BiosLanguage,
            length: 0x1A,
            handle: 0,
            data: &[],
            strings: &[
                // "en|US|iso8859-1"
                0x65,0x6E,0x7C,0x55,0x53,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "fr|FR|iso8859-1"
                0x66,0x72,0x7C,0x46,0x52,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "es|ES|iso8859-1"
                0x65,0x73,0x7C,0x45,0x53,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "de|DE|iso8859-1"
                0x64,0x65,0x7C,0x44,0x45,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "it|IT|iso8859-1"
                0x69,0x74,0x7C,0x49,0x54,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "da|DK|iso8859-1"
                0x64,0x61,0x7C,0x44,0x4B,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "fi|FI|iso8859-1"
                0x66,0x69,0x7C,0x46,0x49,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "nl|NL|iso8859-1"
                0x6E,0x6C,0x7C,0x4E,0x4C,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "no|NO|iso8859-1"
                0x6E,0x6F,0x7C,0x4E,0x4F,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "pt|PT|iso8859-1"
                0x70,0x74,0x7C,0x50,0x54,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "sv|SE|iso8859-1"
                0x73,0x76,0x7C,0x53,0x45,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,
                // "ja|JP|unicode"
                0x6A,0x61,0x7C,0x4A,0x50,0x7C,0x75,0x6E,0x69,0x63,0x6F,0x64,0x65,0x00,
                // "zh|CN|unicode"
                0x7A,0x68,0x7C,0x43,0x4E,0x7C,0x75,0x6E,0x69,0x63,0x6F,0x64,0x65,0x00,
                ],
        };
        let result = InstallableLanguages::new(structure);
        assert_eq!(sample, result.collect::<Vec<_>>(), "Installable language list");
    }

    #[test]
    fn dmi_bin() {
        use crate::InfoType;
        let bios_language_result = ENTRY_POINT
            .structures(&DMIDECODE_BIN[(ENTRY_POINT.smbios_address() as usize)..])
            .find_map(|s| {
                if let Ok(crate::Structure::BiosLanguage(bl)) = s {
                    Some(bl)
                } else {
                    None
                }
            }).unwrap();
        let bios_language_sample = 
            BiosLanguage {
                handle: 0x0D00,
                installable_languages:
                    InstallableLanguages::new(
                        RawStructure {
                            version: (3, 2).into(),
                            info: InfoType::BiosLanguage,
                            length: 0x16,
                            handle: 0x0D00,
                            data: &[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                            strings: &[0x65,0x6E,0x7C,0x55,0x53,0x7C,0x69,0x73,0x6F,0x38,0x38,0x35,0x39,0x2D,0x31,0x00,0x00]
                        }
                    ),
                flags:
                    Some(LanguageFlags([
                    ].iter().collect())),
                current_language: 1,
            };
        assert_eq!(bios_language_sample, bios_language_result, "BIOS language structure");
    }
}
