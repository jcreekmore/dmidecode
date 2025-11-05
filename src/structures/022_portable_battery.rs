//! Portable Battery (Type 22)
//!
//! This structure describes the attributes of the portable battery or batteries for the system.
//! The structure contains the static attributes for the group. Each structure describes a single
//! battery pack’s attributes.

use core::fmt;

use crate::{
    InfoType,
    MalformedStructureError::{self, InvalidFormattedSectionLength},
    RawStructure,
};

/// Main struct for *Portable Battery (Type 22)*
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PortableBattery<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Location of the battery
    pub location: &'a str,
    /// Company name that manufactured the battery
    pub manufacturer: &'a str,
    pub manufacture_date: ManufactureDate<'a>,
    pub serial_number: SerialNumber<'a>,
    /// String that names the battery device
    pub device_name: &'a str,
    pub device_chemistry: DeviceChemistry<'a>,
    pub design_capacity: DesignCapacity,
    /// Design voltage of the battery in mVolts.\
    /// If the value is unknown, the field contains 0.
    pub design_voltage: u16,
    /// String that contains the Smart Battery Data Specification version number supported by this
    /// battery.\
    /// If the battery does not support the function, no string is supplied.
    pub sbds_version_number: &'a str,
    /// Maximum error (as a percentage in the range 0 to 100) in the Watt-hour data reported by the
    /// battery, indicating an upper bound on how much additional energy the battery might have
    /// above the energy it reports having.\
    /// If the value is unknown, the field contains FFh.
    pub maximum_error_in_battery_data: u8,
    pub oem_specific: Option<u32>,
}

/// Date on which the battery was manufactured.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManufactureDate<'a> {
    None,
    Basic(&'a str),
    SmartBatteryDataSpecification { year: u16, month: u8, date: u8 },
}

/// Serial number for the batter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SerialNumber<'a> {
    None,
    Basic(&'a str),
    SmartBatteryDataSpecification(u16),
}

/// Design capacity of the battery in mWatt-hours
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DesignCapacity {
    Unknown,
    Data { value: u16, multiplier: u8 },
}

/// Identifies the battery chemistry.\
///
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeviceChemistry<'a> {
    Other,
    Unknown,
    /// Lead Acid
    LeadAcid,
    /// Nickel Cadmium
    NickelCadmium,
    /// Nickel metal hydride
    NickelMetalHydride,
    /// Lithium-ion
    LithiumIon,
    /// Zinc air
    ZincAir,
    /// Lithium Polymer
    LithiumPolymer,
    Undefined(u8),
    /// String that identifies the battery chemistry (for example, “PbAc”)
    SmartBatteryDataSpecification(&'a str),
}

impl<'a> PortableBattery<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let handle = structure.handle;
        match (structure.version.major, structure.version.minor) {
            (2, 1) if structure.length != 0x10 => Err(InvalidFormattedSectionLength(
                InfoType::PortableBattery,
                handle,
                "",
                0x10,
            )),
            v if v >= (2, 2) && structure.length != 0x1A => Err(InvalidFormattedSectionLength(
                InfoType::PortableBattery,
                handle,
                "",
                0x1A,
            )),
            _ => Ok(Self {
                handle,
                location: structure.get_string(0x04)?,
                manufacturer: structure.get_string(0x05)?,
                manufacture_date: ManufactureDate::new(
                    structure
                        .get::<u8>(0x06)
                        .ok()
                        .filter(|idx| idx != &0)
                        .and_then(|idx| structure.find_string(idx).ok()),
                    structure.get::<u16>(0x12).ok(),
                ),
                serial_number: SerialNumber::new(
                    structure
                        .get::<u8>(0x07)
                        .ok()
                        .filter(|idx| idx != &0)
                        .and_then(|idx| structure.find_string(idx).ok()),
                    structure.get::<u16>(0x10).ok(),
                ),
                device_name: structure.get_string(0x08)?,
                device_chemistry: DeviceChemistry::new(structure.get::<u8>(0x09)?, structure.get_string(0x14).ok()),
                design_capacity: DesignCapacity::new(structure.get::<u16>(0x0A)?, structure.get::<u8>(0x15).ok()),
                design_voltage: structure.get::<u16>(0x0C)?,
                sbds_version_number: structure.get_string(0x0E)?,
                maximum_error_in_battery_data: structure.get::<u8>(0x0F)?,
                oem_specific: structure.get::<u32>(0x16).ok(),
            }),
        }
    }
}

impl<'a> ManufactureDate<'a> {
    fn new(basic: Option<&'a str>, sbds: Option<u16>) -> Self {
        match (basic, sbds) {
            (Some(s), _) => Self::Basic(s),
            (None, Some(date)) => Self::SmartBatteryDataSpecification {
                year: ((date & 0b1111_1110_0000_0000) >> 9) + 1980,
                month: ((date & 0b0000_0001_1110_0000) >> 5) as u8,
                date: (date & 0b0000_0000_0001_1111) as u8,
            },
            _ => Self::None,
        }
    }
}
impl fmt::Display for ManufactureDate<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::Basic(s) => write!(f, "{s}"),
            Self::SmartBatteryDataSpecification { year, month, date } =>
            // ISO 8601
            {
                write!(f, "{year:04}-{month:02}-{date:02}")
            }
        }
    }
}

impl<'a> SerialNumber<'a> {
    fn new(basic: Option<&'a str>, sbds: Option<u16>) -> Self {
        match (basic, sbds) {
            (Some(s), _) => Self::Basic(s),
            (None, Some(word)) => Self::SmartBatteryDataSpecification(word),
            _ => Self::None,
        }
    }
}
impl fmt::Display for SerialNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::Basic(s) => write!(f, "{s}"),
            Self::SmartBatteryDataSpecification(word) => write!(f, "{word:#04X}"),
        }
    }
}

impl<'a> DeviceChemistry<'a> {
    fn new(basic: u8, sbds: Option<&'a str>) -> Self {
        match basic {
            0x01 => Self::Other,
            0x02 => {
                if let Some(s) = sbds {
                    Self::SmartBatteryDataSpecification(s)
                } else {
                    Self::Unknown
                }
            }
            0x03 => Self::LeadAcid,
            0x04 => Self::NickelCadmium,
            0x05 => Self::NickelMetalHydride,
            0x06 => Self::LithiumIon,
            0x07 => Self::ZincAir,
            0x08 => Self::LithiumPolymer,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for DeviceChemistry<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::LeadAcid => write!(f, "Lead Acid"),
            Self::NickelCadmium => write!(f, "Nickel Cadmium"),
            Self::NickelMetalHydride => write!(f, "Nickel metal hydride"),
            Self::LithiumIon => write!(f, "Lithium-ion"),
            Self::ZincAir => write!(f, "Zinc air"),
            Self::LithiumPolymer => write!(f, "Lithium Polymer"),
            Self::Undefined(v) => write!(f, "Undefined: {v}"),
            Self::SmartBatteryDataSpecification(s) => write!(f, "{s}"),
        }
    }
}

impl DesignCapacity {
    fn new(value: u16, multipler: Option<u8>) -> Self {
        if value == 0 {
            Self::Unknown
        } else {
            Self::Data {
                value,
                multiplier: multipler.unwrap_or(1),
            }
        }
    }
}
impl From<DesignCapacity> for u64 {
    fn from(dc: DesignCapacity) -> Self {
        match dc {
            DesignCapacity::Unknown => 0,
            DesignCapacity::Data { value, multiplier } => (value * multiplier as u16).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq as pretty_assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn manufacture_date() {
        use super::ManufactureDate;

        pretty_assert_eq!("", format!("{}", ManufactureDate::new(None, None)), "Empty");
        pretty_assert_eq!(
            "07/17/2019",
            format!("{}", ManufactureDate::new(Some("07/17/2019"), None)),
            "Basic"
        );
        pretty_assert_eq!(
            "2000-02-01",
            format!("{}", ManufactureDate::new(None, Some(0x2841))),
            "SBDS"
        );
    }

    #[test]
    fn serial_number() {
        use super::SerialNumber;

        pretty_assert_eq!("", format!("{}", SerialNumber::new(None, None)), "Empty");
        pretty_assert_eq!(
            "S/N 1111",
            format!("{}", SerialNumber::new(Some("S/N 1111"), None)),
            "Basic"
        );
        pretty_assert_eq!("0xBEAF", format!("{}", SerialNumber::new(None, Some(0xBEAF))), "SBDS");
    }

    #[test]
    fn device_chemistry() {
        use super::DeviceChemistry;

        let sample = &[
            "Undefined: 0",
            "Other",
            "Unknown",
            "Lead Acid",
            "Nickel Cadmium",
            "Nickel metal hydride",
            "Lithium-ion",
            "Zinc air",
            "Lithium Polymer",
        ];
        for (n, &s) in sample.iter().enumerate() {
            let sbds = None;
            pretty_assert_eq!(s, format!("{}", DeviceChemistry::new(n as u8, sbds)));
            if n == 0x02 {
                let sbds = Some("PbAc");
                pretty_assert_eq!("PbAc", format!("{:#}", DeviceChemistry::new(n as u8, sbds)));
            }
        }
    }

    #[test]
    fn design_capacity() {
        use super::DesignCapacity;

        pretty_assert_eq!(0u64, DesignCapacity::new(0, None).into(), "Unknown");
        pretty_assert_eq!(0u64, DesignCapacity::new(0, Some(42)).into(), "Unknown");
        pretty_assert_eq!(4800u64, DesignCapacity::new(4800, None).into(), "w/o multiplier");
        pretty_assert_eq!(9600u64, DesignCapacity::new(4800, Some(2)).into(), "With multiplier");
    }

    #[test]
    fn portable_battery() {
        use super::*;
        use crate::{InfoType, RawStructure};

        let length = 26;
        let (data, strings) =
            include_bytes!("../../tests/data/________/entries/22-0/bin")[4..].split_at(length as usize - 4);
        let structure = RawStructure {
            version: (3, 2).into(),
            info: InfoType::PortableBattery,
            length,
            handle: 0x002B,
            data,
            strings,
        };
        let sample = PortableBattery {
            handle: 0x002B,
            location: "Front",
            manufacturer: "LGC",
            manufacture_date: ManufactureDate::SmartBatteryDataSpecification {
                year: 2020,
                month: 7,
                date: 1,
            },
            serial_number: SerialNumber::SmartBatteryDataSpecification(0x058B),
            device_name: "5B10W13930",
            device_chemistry: DeviceChemistry::SmartBatteryDataSpecification("LiP"),
            design_capacity: DesignCapacity::Data {
                value: 5100,
                multiplier: 10,
            },
            design_voltage: 15400,
            sbds_version_number: "03.01",
            maximum_error_in_battery_data: 0xFF,
            oem_specific: Some(0),
        };
        let result = PortableBattery::try_from(structure).unwrap();
        pretty_assert_eq!(sample, result, "PortableBattery");
    }
}
