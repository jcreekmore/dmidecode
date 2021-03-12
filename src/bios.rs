//! BIOS related information
//!
//! [BIOS Information](Bios "Bios") (Type 0) and [BIOS Language Information](BiosLanguage
//! "BiosLanguage") (Type 13)

use core::fmt;

use crate::bitfield::{BitField, FlagType, Layout};
use crate::{MalformedStructureError, RawStructure,};

/// BIOS Information (Type 0)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct Bios<'buffer> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// BIOS Vendor’s Name
    pub vendor: &'buffer str,
    /// BIOS Version. This value is a free-form string that may contain Core and OEM version
    /// information
    pub bios_version: &'buffer str,
    /// Segment location of BIOS starting address
    pub bios_starting_address_segment: u16,
    /// String number of the BIOS release date.  The date string, if supplied, is in either
    /// mm/dd/yy or mm/dd/yyyy format. If the year portion of the string is two digits, the year is
    /// assumed to be 19yy
    /// NOTE: The mm/dd/yyyy format is required for SMBIOS version 2.3 and later
    pub bios_release_date: &'buffer str,
    /// The size of the physical device containing the BIOS. 
    pub rom_size: RomSize,
    /// Defines which functions the BIOS supports: PCI, PCMCIA, Flash, etc. 
    pub bios_characteristics: Characteristics,
    /// For version 2.1 and later implementations one Extensions Byte defined
    pub bios_characteristics_exttension_1: Option<CharacteristicsExtension1>,
    /// For version 2.4 and later implementations two Extensions Bytes defined
    pub bios_characteristics_exttension_2: Option<CharacteristicsExtension2>,
    /// System BIOS Revision
    pub bios_revision: Option<BiosRevision>,
    /// Embedded Controller Firmware Revision
    pub firmware_revision: Option<FirmwareRevision>,
}

/// BIOS Characteristics 
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct Characteristics(u64);

/// BIOS Characteristics Extension Byte 1 layout. This information, available for SMBIOS 946
/// version 2.1 and later, appears at offset 12h within the BIOS Information structure.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct CharacteristicsExtension1(u8);

/// BIOS Characteristics for Extension Byte 2 layout. This information, available for 950 SMBIOS
/// version 2.4 and later, appears at offset 13h within the BIOS Information structure.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct CharacteristicsExtension2(u8);

/// BIOS Revision assembled from *System BIOS Major Release* and *System BIOS Minor Release* fields
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct BiosRevision {
    pub major: u8,
    pub minor: u8
}

/// Firmware Revision assembled from *Embedded Controller Firmware Major Release* and
/// *Embedded Controller Firmware Minor Release* fields
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct FirmwareRevision {
    pub major: u8,
    pub minor: u8
}

/// The size of the physical device containing the BIOS.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct RomSize {
    /// Size (n) where 64K * (n+1) is the size of the physical device containing the BIOS, in
    /// bytes.  FFh - size is 16MB or greater
    pub basic: u8,
    /// Extended size of the physical device(s) containing the BIOS, rounded up if needed.
    pub extended: Option<u16>,
}


/// BIOS Language Information (Type 13) 
#[derive(Clone, Debug, Eq, Hash, PartialEq, )]
pub struct BiosLanguage<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Available languages
    pub installable_languages: InstallableLanguages<'a>,
    //pub installable_languages: &'a dyn Iterator<Item = &'a str>,
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


impl<'buffer> Bios<'buffer> {
    pub(crate) fn try_from(structure: RawStructure<'buffer>) -> Result<Bios<'buffer>, MalformedStructureError> {
        #[repr(C)]
        #[repr(packed)]
        struct BiosPacked_3_1 {
            vendor: u8,
            bios_version: u8,
            bios_starting_address_segment: u16,
            bios_release_date: u8,
            bios_rom_size: u8,
            bios_characteristics: u64,
            bios_characteristics_exttension_1: u8,
            bios_characteristics_exttension_2: u8,
            system_bios_major_release: u8,
            system_bios_minor_release: u8,
            embedded_controller_firmware_major_release: u8,
            embedded_controller_firmware_minor_release: u8,
            extended_bios_rom_size: u16
        }

        #[repr(C)]
        #[repr(packed)]
        struct BiosPacked_2_4 {
            vendor: u8,
            bios_version: u8,
            bios_starting_address_segment: u16,
            bios_release_date: u8,
            bios_rom_size: u8,
            bios_characteristics: u64,
            bios_characteristics_exttension_1: u8,
            bios_characteristics_exttension_2: u8,
            system_bios_major_release: u8,
            system_bios_minor_release: u8,
            embedded_controller_firmware_major_release: u8,
            embedded_controller_firmware_minor_release: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct BiosPacked_2_0 {
            vendor: u8,
            bios_version: u8,
            bios_starting_address_segment: u16,
            bios_release_date: u8,
            bios_rom_size: u8,
            bios_characteristics: u64,
        }

        match structure.version {
            v if v >= (3, 1).into() => {
                let_as_struct!(packed, BiosPacked_3_1, structure.data);
                Ok(Bios{
                    handle: structure.handle,
                    vendor: structure.find_string(packed.vendor)?,
                    bios_version: structure.find_string(packed.bios_version)?,
                    bios_starting_address_segment: packed.bios_starting_address_segment,
                    bios_release_date: structure.find_string(packed.bios_release_date)?,
                    rom_size: RomSize { basic: packed.bios_rom_size, extended: Some(packed.extended_bios_rom_size) },
                    bios_characteristics: Characteristics(packed.bios_characteristics),
                    bios_characteristics_exttension_1: Some(CharacteristicsExtension1(packed.bios_characteristics_exttension_1)),
                    bios_characteristics_exttension_2: Some(CharacteristicsExtension2(packed.bios_characteristics_exttension_2)),
                    bios_revision:
                        Some(
                            BiosRevision {
                                major: packed.system_bios_major_release,
                                minor: packed.system_bios_minor_release,
                            }
                        ),
                    firmware_revision:
                        Some(
                            FirmwareRevision {
                                major: packed.embedded_controller_firmware_major_release,
                                minor: packed.embedded_controller_firmware_minor_release,
                            }
                        ),
                })
            },
            v if v >= (2, 4).into() => {
                let_as_struct!(packed, BiosPacked_2_4, structure.data);
                Ok(Bios{
                    handle: structure.handle,
                    vendor: structure.find_string(packed.vendor)?,
                    bios_version: structure.find_string(packed.bios_version)?,
                    bios_starting_address_segment: packed.bios_starting_address_segment,
                    bios_release_date: structure.find_string(packed.bios_release_date)?,
                    rom_size: RomSize { basic: packed.bios_rom_size, extended: None },
                    bios_characteristics: Characteristics(packed.bios_characteristics),
                    bios_characteristics_exttension_1: Some(CharacteristicsExtension1(packed.bios_characteristics_exttension_1)),
                    bios_characteristics_exttension_2: Some(CharacteristicsExtension2(packed.bios_characteristics_exttension_2)),
                    bios_revision:
                        Some(
                            BiosRevision {
                                major: packed.system_bios_major_release,
                                minor: packed.system_bios_minor_release,
                            }
                        ),
                    firmware_revision:
                        Some(
                            FirmwareRevision {
                                major: packed.embedded_controller_firmware_major_release,
                                minor: packed.embedded_controller_firmware_minor_release,
                            }
                        ),
                })
            },
            _ => {
                let_as_struct!(packed, BiosPacked_2_0, structure.data);
                Ok(Bios{
                    handle: structure.handle,
                    vendor: structure.find_string(packed.vendor)?,
                    bios_version: structure.find_string(packed.bios_version)?,
                    bios_starting_address_segment: packed.bios_starting_address_segment,
                    bios_release_date: structure.find_string(packed.bios_release_date)?,
                    rom_size: RomSize { basic: packed.bios_rom_size, extended: None },
                    bios_characteristics: Characteristics(packed.bios_characteristics),
                    bios_characteristics_exttension_1: None,
                    bios_characteristics_exttension_2: None,
                    bios_revision: None,
                    firmware_revision: None,
                })
            },
        }
    }
}

impl<'a> BitField<'a> for Characteristics {
    type Size = u64;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 64;
        "Reserved",
        "Reserved",
        "Unknown",
        "BIOS characteristics not supported"
            "BIOS Characteristics are not supported",
        "ISA is supported",
        "MCA is supported",
        "EISA is supported",
        "PCI is supported",
        "PC card (PCMCIA) is supported",
        "PNP is supported"
            "Plug and Play is supported",
        "APM is supported",
        "BIOS is upgradeable"
            "BIOS is upgradeable (Flash)",
        "BIOS shadowing is allowed",
        "VLB is supported"
            "VL-VESA is supported",
        "ESCD support is available",
        "Boot from CD is supported",
        "Selectable boot is supported",
        "BIOS ROM is socketed"
            "BIOS ROM is socketed (e.g. PLCC or SOP socket)",
        "Boot from PC card (PCMCIA) is supported",
        "EDD is supported"
            "EDD specification is supported",
        "Japanese floppy for NEC 9800 1.2 MB is supported (int 13h)"
            "Int 13h — Japanese floppy for NEC 9800 1.2 MB (3.5”, 1K bytes/sector, 360 RPM) is supported",
        "Japanese floppy for Toshiba 1.2 MB is supported (int 13h)"
            "Int 13h — Japanese floppy for Toshiba 1.2 MB (3.5”, 360 RPM) is supported",
        "5.25\"/360 kB floppy services are supported (int 13h)"
            "Int 13h — 5.25” / 360 KB floppy services are supported",
        "5.25\"/1.2 MB floppy services are supported (int 13h)"
            "Int 13h — 5.25” /1.2 MB floppy services are supported",
        "3.5\"/720 kB floppy services are supported (int 13h)"
            "Int 13h — 3.5” / 720 KB floppy services are supported",
        "3.5\"/2.88 MB floppy services are supported (int 13h)"
            "Int 13h — 3.5” / 2.88 MB floppy services are supported",
        "Print screen service is supported (int 5h)"
            "Int 5h, print screen Service is supported",
        "8042 keyboard services are supported (int 9h)"
            "Int 9h, 8042 keyboard services are supported",
        "Serial services are supported (int 14h)"
            "Int 14h, serial services are supported",
        "Printer services are supported (int 17h)"
            "Int 17h, printer services are supported",
        "CGA/mono video services are supported (int 10h)"
            "Int 10h, CGA/Mono Video Services are supported",
        "NEC PC-98",
        "Reserved for BIOS vendor": 16,
        "Reserved for system vendor": 16,
    );
}

impl<'a> BitField<'a> for CharacteristicsExtension1 {
    type Size = u8;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 8;
        "ACPI is supported",
        "USB legacy is supported"
            "USB Legacy is supported",
        "AGP is supported",
        "I2O boot is supported",
        "LS-120 SuperDisk boot is supported",
        "ATAPI ZIP drive boot is supported",
        "IEEE 1394 boot is supported"
            "1394 boot is supported",
        "Smart battery is supported",
    );
}

impl<'a> BitField<'a> for CharacteristicsExtension2 {
    type Size = u8;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 8;
        "BIOS boot specification is supported"
            "BIOS Boot specification is supported",
        "Function key-initiated network boot is supported"
            "Function key-initiated network service boot is supported. When function \
            key-uninitiated network service boot is not supported, a network adapter option ROM \
            may choose to offer this functionality on its own, thus offering this capability to \
            legacy systems. When the function is supported, the network adapter option ROM \
            shall not offer this capability",
        "Targeted content distribution is supported"
            "Enable targeted content distribution. The manufacturer has ensured that the SMBIOS \
            data is useful in identifying the computer for targeted delivery of model-specific \
            software and firmware content through third-party content distribution services",
         "UEFI is supported",
         "System is a virtual machine"
            "SMBIOS table describes a virtual machine. (If this bit is not set, no inference \
            can be made about the virtuality of the system.)",
         "Reserved for future assignment": 3,
    );
}

impl From<RomSize> for u64 {
    fn from(rom_size: RomSize) -> Self {
        if rom_size.basic != 0xFF {
            (rom_size.basic + 1) as u64 * (64 << 10)
        }
        else if let Some(extended) = rom_size.extended {
            let unit = (extended >> 14) & 0b11;
            let size = (extended & 0x3fff) as u64;
            match unit {
                0b00 => size << 20,
                0b01 => size << 30,
                _ => unimplemented!(),
            }
        }
        else {
            unreachable!();
        }
    }
}

impl fmt::Display for BiosRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.major == 0xFF && self.minor == 0xFF {
            write!(f, "N/A")
        } else {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }
}

impl fmt::Display for FirmwareRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.major == 0xFF && self.minor == 0xFF {
            write!(f, "N/A")
        } else {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }
}

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
    
    use crate::bitfield::Position;
    use super::*;

    const PRIMES: &[usize] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61];
    const DMIDECODE_BIN: &'static [u8] = include_bytes!("../tests/data/dmi.0.bin");
    lazy_static! {
        static ref ENTRY_POINT: crate::EntryPoint = crate::EntryPoint::search(DMIDECODE_BIN).unwrap();
    }

    #[test]
    fn characteristics() {
        let sample = PRIMES.iter().cloned().collect::<Vec<_>>();
        let qword = sample.iter().map(|&p| Position(p)).collect();
        let result = Characteristics(qword)
            .iter()
            .filter_map(|f| if f.is_set { Some(*f.position) } else { None })
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Positions");

        let sample = vec!["ISA is supported","EISA is supported"];
        let qword = 0b0101_0000;
        let iter =  Characteristics(qword)
            .significants();
        let result = iter
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Significant values, default formatting ({:064b})", qword);
        let result = iter
            .map(|f| format!("{:#}", f))
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Significant values, alternative formatting ({:064b})", qword);

        let sample = vec![
            ("Reserved for BIOS vendor", 32..=47),
            ("Reserved for system vendor", 48..=63),
        ];
        let result = Characteristics(0).reserved()
            .map(|v| (v.description, v.range))
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Reserved fields");
    }
    #[test]
    fn characteristics_extension1() {
        let sample = PRIMES.iter().cloned().take_while(|&x| x < 8).collect::<Vec<_>>();
        let byte = sample.iter().map(|&p| Position(p)).collect();
        let result = Characteristics(byte)
            .iter()
            .filter_map(|f| if f.is_set { Some(*f.position) } else { None })
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Positions");

        let dflt_sample = vec!["ACPI is supported","IEEE 1394 boot is supported"];
        let alt_sample = vec!["ACPI is supported","1394 boot is supported"];
        let byte = 0b0100_0001;
        let iter =  CharacteristicsExtension1(byte)
            .significants();
        let dflt_result = iter
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>();
        assert_eq!(dflt_sample, dflt_result, "Significant values, default formatting ({:08b})", byte);
        let alt_result = iter
            .map(|f| format!("{:#}", f))
            .collect::<Vec<_>>();
        assert_eq!(alt_sample, alt_result, "Significant values, alternative formatting ({:08b})", byte);

        let result = CharacteristicsExtension1(0).reserved().count();
        assert_eq!(0, result, "Reserved fields");
    }
    #[test]
    fn characteristics_extension2() {
        let sample = PRIMES.iter().cloned().take_while(|&x| x < 8).collect::<Vec<_>>();
        let byte = sample.iter().map(|&p| Position(p)).collect();
        let result = Characteristics(byte)
            .iter()
            .filter_map(|f| if f.is_set { Some(*f.position) } else { None })
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Positions");

        let short_sample = vec!["UEFI is supported","System is a virtual machine"];
        let long_sample = vec!["UEFI is supported","SMBIOS table describes a virtual machine. (If this bit is not set, no inference can be made about the virtuality of the system.)"];
        let byte = 0b0001_1000;
        let iter =  CharacteristicsExtension2(byte)
            .significants();
        let result = iter
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>();
        assert_eq!(short_sample, result, "Significant values, default formatting ({:08b})", byte);
        let result = iter
            .map(|f| format!("{:#}", f))
            .collect::<Vec<_>>();
        assert_eq!(long_sample, result, "Significant values, alternative formatting ({:08b})", byte);

        let sample = vec![("Reserved for future assignment", 5..=7)];
        let result = CharacteristicsExtension2(0).reserved()
            .map(|v| (v.description, v.range))
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Reserved fields");
    }
    #[test]
    fn rom_size() {
        let data = &[
            (8<<20,            0x7F, None        ),// 8 MB
            ((16<<20) - 65536, 0xFE, None        ),// ~16 MB, Last of basic
            (16<<20,           0xFF, Some(0x0010)),// 16 MB
            (64<<20,           0xFF, Some(64)    ),// 64 MB
            (48<<30,           0xFF, Some(0x4030)),// 48 GB
        ];
        let sample: Vec<u64> = data.iter()
            .map(|(size, ..)| *size)
            .collect();
        let result: Vec<u64> = data.iter()
            .map(|(_, basic, extended)| RomSize { basic: *basic, extended: *extended }.into())
            .collect();
        assert_eq!(sample, result, "ROM Size");
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
    fn dmi_bin_full_bios_structure() {
        let bios_sample = Bios {
                    handle: 0,
                    vendor: "Dell Inc.",
                    bios_version: "2.8.2",
                    bios_starting_address_segment: 0xF000,
                    bios_release_date: "08/27/2020",
                    rom_size: RomSize { basic: 0xFF, extended: Some(32) },
                    bios_characteristics:
                        Characteristics([
                            Position(4),
                            Position(7),
                            Position(9),
                            Position(11),
                            Position(12),
                            Position(15),
                            Position(16),
                            Position(19),
                            Position(21),
                            Position(22),
                            Position(23),
                            Position(24),
                            Position(27),
                            Position(28),
                            Position(30),
                            // Flags below are for reserved fields
                            Position(48),
                            Position(49),
                            Position(50),
                            Position(51),
                            Position(52),
                        ].iter().collect()),
                    bios_characteristics_exttension_1:
                        Some(
                            CharacteristicsExtension1([
                                Position(0),
                                Position(1),
                            ].iter().collect())
                        ),
                    bios_characteristics_exttension_2:
                        Some(
                            CharacteristicsExtension2([
                                Position(0),
                                Position(1),
                                Position(2),
                                Position(3),
                            ].iter().collect())
                        ),
                    bios_revision:
                        Some(
                            BiosRevision { major: 2, minor: 8 }
                        ),
                    firmware_revision:
                        Some(
                            FirmwareRevision { major: 0xFF, minor: 0xFF }
                        ),
        };
        let bios_result = ENTRY_POINT
            .structures(&DMIDECODE_BIN[(ENTRY_POINT.smbios_address() as usize)..])
            .find_map(|s| {
                if let Ok(crate::Structure::Bios(bios)) = s {
                    Some(bios)
                } else {
                    None
                }
            }).unwrap();
        assert_eq!(bios_sample, bios_result, "Full BIOS Struct");
    }

    #[test]
    fn dmi_bin_all_characteristics() {
        let all_characteristics_sample = vec![
                "ISA is supported",
                "PCI is supported",
                "PNP is supported",
                "BIOS is upgradeable",
                "BIOS shadowing is allowed",
                "Boot from CD is supported",
                "Selectable boot is supported",
                "EDD is supported",
                "Japanese floppy for Toshiba 1.2 MB is supported (int 13h)",
                "5.25\"/360 kB floppy services are supported (int 13h)",
                "5.25\"/1.2 MB floppy services are supported (int 13h)",
                "3.5\"/720 kB floppy services are supported (int 13h)",
                "8042 keyboard services are supported (int 9h)",
                "Serial services are supported (int 14h)",
                "CGA/mono video services are supported (int 10h)",
                "ACPI is supported",
                "USB legacy is supported",
                "BIOS boot specification is supported",
                "Function key-initiated network boot is supported",
                "Targeted content distribution is supported",
                "UEFI is supported",
        ];
        let bios_result = ENTRY_POINT
            .structures(&DMIDECODE_BIN[(ENTRY_POINT.smbios_address() as usize)..])
            .find_map(|s| {
                if let Ok(crate::Structure::Bios(bios)) = s {
                    Some(bios)
                } else {
                    None
                }
            }).unwrap();
        let all_char_result = bios_result.bios_characteristics
            .significants()
            .chain(bios_result.bios_characteristics_exttension_1.unwrap().significants())
            .chain(bios_result.bios_characteristics_exttension_2.unwrap().significants())
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>();
        assert_eq!(all_characteristics_sample, all_char_result, "Characteristics as in dmidecode tool");
    }

    #[test]
    fn dmi_bin_revisions() {
        let bios_revision = "2.8";
        let firmware_revision = "N/A";
        let bios_result = ENTRY_POINT
            .structures(&DMIDECODE_BIN[(ENTRY_POINT.smbios_address() as usize)..])
            .find_map(|s| {
                if let Ok(crate::Structure::Bios(bios)) = s {
                    Some(bios)
                } else {
                    None
                }
            }).unwrap();
        assert_eq!(bios_revision, format!("{}", bios_result.bios_revision.unwrap()), "BIOS Revision");
        assert_eq!(firmware_revision, format!("{}", bios_result.firmware_revision.unwrap()), "Firmware Revision");
    }

    #[test]
    fn dmi_bin_bios_size() {
        let size = 32u64 << 20;
        let bios_result = ENTRY_POINT
            .structures(&DMIDECODE_BIN[(ENTRY_POINT.smbios_address() as usize)..])
            .find_map(|s| {
                if let Ok(crate::Structure::Bios(bios)) = s {
                    Some(bios)
                } else {
                    None
                }
            }).unwrap();
        assert_eq!(size, bios_result.rom_size.into(), "ROM BIOS size");
    }

    #[test]
    fn dmi_bin_bios_language_structure() {
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
