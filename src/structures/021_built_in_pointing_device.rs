//! Built-in Pointing Device (Type 21)
//!
//! This structure describes the attributes of the built-in pointing device for the system.\
//! The presence of this structure does not imply that the built-in pointing device is active for
//! the system’s use.

use core::fmt;

use crate::{
    InfoType,
    MalformedStructureError::{self, InvalidFormattedSectionLength},
    RawStructure,
};

/// Main struct for *Built-in Pointing Device (Type 21)*
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BuiltInPointingDevice {
    /// Specifies the structure’s handle
    pub handle: u16,
    pub type_: Type,
    pub interface: Interface,
    /// Number of buttons on the pointing device.\
    /// If the device has three buttons, the field value is 03h.
    pub number_of_buttons: u8,
}

/// Type of pointing device
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Other,
    Unknown,
    Mouse,
    TrackBall,
    TrackPoint,
    GlidePoint,
    TouchPad,
    TouchScreen,
    OpticalSensor,
    Undefined(u8),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Interface {
    Other,
    Unknown,
    Serial,
    /// PS/2
    Ps2,
    Infrared,
    /// HP-HIL
    HpHil,
    /// Bus mouse
    BusMouse,
    /// ADB (Apple Desktop Bus)
    Adb,
    /// Bus mouse DB-9
    BusMouseDb9,
    /// Bus mouse micro-DIN
    BusMouseMicroDin,
    /// USB
    Usb,
    Undefined(u8),
}

impl<'a> BuiltInPointingDevice {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<Self, MalformedStructureError> {
        let handle = structure.handle;
        match (structure.version.major, structure.version.minor) {
            v if v >= (2, 1) && structure.length != 0x07 => Err(InvalidFormattedSectionLength(
                InfoType::BuiltInPointingDevice,
                handle,
                "",
                0x07,
            )),
            _ => Ok(Self {
                handle,
                type_: structure.get::<u8>(0x04)?.into(),
                interface: structure.get::<u8>(0x05)?.into(),
                number_of_buttons: structure.get::<u8>(0x06)?,
            }),
        }
    }
}

impl From<u8> for Type {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Mouse,
            0x04 => Self::TrackBall,
            0x05 => Self::TrackPoint,
            0x06 => Self::GlidePoint,
            0x07 => Self::TouchPad,
            0x08 => Self::TouchScreen,
            0x09 => Self::OpticalSensor,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Mouse => write!(f, "Mouse"),
            Self::TrackBall => write!(f, "Track Ball"),
            Self::TrackPoint => write!(f, "Track Point"),
            Self::GlidePoint => write!(f, "Glide Point"),
            Self::TouchPad => write!(f, "Touch Pad"),
            Self::TouchScreen => write!(f, "Touch Screen"),
            Self::OpticalSensor => write!(f, "Optical Sensor"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for Interface {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Serial,
            0x04 => Self::Ps2,
            0x05 => Self::Infrared,
            0x06 => Self::HpHil,
            0x07 => Self::BusMouse,
            0x08 => Self::Adb,
            0xA0 => Self::BusMouseDb9,
            0xA1 => Self::BusMouseMicroDin,
            0xA2 => Self::Usb,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Serial => write!(f, "Serial"),
            Self::Ps2 => write!(f, "PS/2"),
            Self::Infrared => write!(f, "Infrared"),
            Self::HpHil => write!(f, "HP-HIL"),
            Self::BusMouse => write!(f, "Bus mouse"),
            Self::Adb => write!(f, "ADB (Apple Desktop Bus)"),
            Self::BusMouseDb9 => write!(f, "Bus mouse DB-9"),
            Self::BusMouseMicroDin => write!(f, "Bus mouse micro-DIN"),
            Self::Usb => write!(f, "USB"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;

    #[test]
    fn type_() {
        use super::Type;

        let sample = &[
            "Undefined: 0",
            "Other",
            "Unknown",
            "Mouse",
            "Track Ball",
            "Track Point",
            "Glide Point",
            "Touch Pad",
            "Touch Screen",
            "Optical Sensor",
        ];
        for (n, &s) in sample.iter().enumerate() {
            assert_eq!(s, format!("{:#}", Type::from(n as u8)));
        }
    }

    #[test]
    fn interface() {
        use super::Interface;

        let sample = &[
            "Undefined: 0",
            "Other",
            "Unknown",
            "Serial",
            "PS/2",
            "Infrared",
            "HP-HIL",
            "Bus mouse",
            "ADB (Apple Desktop Bus)",
        ];
        for (n, &s) in sample.iter().enumerate() {
            assert_eq!(s, format!("{:#}", Interface::from(n as u8)));
        }
        let sample = &["Bus mouse DB-9", "Bus mouse micro-DIN", "USB"];
        for n in 0xA0..(0xA0 + sample.len()) {
            assert_eq!(sample[n - 0xA0], format!("{:#}", Interface::from(n as u8)));
        }
    }

    #[test]
    fn built_in_pointing_device() {
        use super::*;
        use crate::{InfoType, RawStructure};

        let length = 7;
        let (data, strings) =
            include_bytes!("../../tests/data/________/entries/21-0/bin")[4..].split_at(length as usize - 4);
        let structure = RawStructure {
            version: (2, 7).into(),
            info: InfoType::BuiltInPointingDevice,
            length,
            handle: 0xAAAA,
            data,
            strings,
        };
        let sample = BuiltInPointingDevice {
            handle: 0xAAAA,
            type_: Type::Mouse,
            interface: Interface::Serial,
            number_of_buttons: 3,
        };
        let result = BuiltInPointingDevice::try_from(structure).unwrap();
        assert_eq!(sample, result, "BuiltInPointingDevice");
    }
}
