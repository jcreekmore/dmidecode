//! System Enclosure or Chassis (Type 3)
//!
//! The information in this structure defines attributes of the system’s mechanical
//! enclosure(s). For example, if a system included a separate enclosure for its peripheral
//! devices, two structures would be returned: one for the main system enclosure and the second for
//! the peripheral device enclosure.

use core::fmt;
use core::hash::{Hash, Hasher};
use core::slice::Chunks;

use crate::{MalformedStructureError, RawStructure};

// Each SMBIOS structure begins with a four-byte header
const STRUCTURE_HEADER_LENGTH: u8 = 0x4;

/// System Enclosure or Chassis structure
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enclosure<'buffer> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Manufacturer string is non-null
    pub manufacturer: &'buffer str,
    /// Chassis lock is present
    pub chassis_lock: bool,
    /// Type field identifies the type of enclosure. (Unknown is disallowed.)
    pub enclosure_type: EnclosureType,
    /// Version
    pub version: &'buffer str,
    /// Serial Number
    pub serial_number: &'buffer str,
    /// Asset Tag Number
    pub asset_tag_number: &'buffer str,
    /// State of the enclosure when it was last booted;
    pub boot_up_state: Option<State>,
    /// State of the enclosure’s power supply (or supplies) when last booted
    pub power_supply_state: Option<State>,
    /// Thermal state of the enclosure when last booted
    pub thermal_state: Option<State>,
    /// Physical security status of the enclosure when last booted
    pub security_status: Option<SecurityStatus>,
    /// OEM- or BIOS vendor-specific information
    pub oem_defined: Option<u32>,
    /// Height of the enclosure , in 'U's A U is a standard unit of measure for the height of a
    /// rack or rack-mountable component and is equal to 1.75 inches or 4.445 cm. A value of 00h
    /// indicates that the enclosure height is unspecified.
    pub height: Option<u8>,
    /// Number of power cords associated with the enclosure or chassis A value of 00h indicates
    /// that the number is unspecified.
    pub power_cords_number: Option<u8>,
    /// Each Contained Element record consists of sub-fields that further describe elements
    /// contained by the chassis
    pub contained_elements: Option<ContainedElements<'buffer>>,
    /// Number of null-terminated string describing the chassis or enclosure SKU number
    pub sku_number: Option<&'buffer str>,
}

/// System Enclosure or Chassis Type
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnclosureType {
    Other,
    Unknown,
    Desktop,
    LowProfileDesktop,
    PizzaBox,
    MiniTower,
    Tower,
    Portable,
    Laptop,
    Notebook,
    HandHeld,
    DockingStation,
    AllInOne,
    SubNotebook,
    SpaceSaving,
    LunchBox,
    MainServerChassis,
    ExpansionChassis,
    SubChassis,
    BusExpansionChassis,
    PeripheralChassis,
    RaidChassis,
    RackMountChassis,
    SealedCasePc,
    /// When this value is specified by an SMBIOS implementation, the physical chassis associated
    /// with this structure supports multiple, independently reporting physical systems—regardless
    /// of the chassis' current configuration. Systems in the same physical chassis are required to
    /// report the same value in this structure's Serial Number field.  For a chassis that may also
    /// be configured as either a single system or multiple physical systems, the Multi-system
    /// chassis value is reported even if the chassis is currently configured as a single system.
    /// This allows management applications to recognize the multisystem potential of the chassis.
    MultiSystemChassis,
    CompactPci,
    AdvancedTca,
    /// An SMBIOS implementation for a Blade would contain a Type 3 Chassis structure for the
    /// individual Blade system as well as one for the Blade Enclosure that completes the Blade
    /// system.
    Blade,
    /// A Blade Enclosure is a specialized chassis that contains a set of Blades. It provides much
    /// of the non-core computing infrastructure for a set of Blades (power, cooling, networking,
    /// etc.). A Blade Enclosure may itself reside inside a Rack or be a standalone chassis.
    BladeEnclosure,
    Tablet,
    Convertible,
    Detachable,
    IotGateway,
    EmbeddedPc,
    MiniPc,
    StickPc,
    Undefined(u8),
}

/// System Enclosure or Chassis States
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum State {
    Other,
    Unknown,
    Safe,
    Warning,
    Critical,
    NonRecoverable,
    Undefined(u8),
}

/// System Enclosure or Chassis Security Status
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SecurityStatus {
    Other,
    Unknown,
    None,
    ExternalInterfaceLockedOut,
    ExternalInterfaceEnabled,
    Undefined(u8),
}

/// Elements, possibly defined by other SMBIOS structures, present in this chassis
#[derive(Clone, Debug)]
pub struct ContainedElements<'buffer> {
    chunks: Chunks<'buffer, u8>,
    count: u8,
    record_length: u8,
}

/// Each Contained Element record consists of sub-fields that further describe elements contained
/// by the chassis.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContainedElement {
    /// Specifies the type of element associated with this record
    type_: ContainedElementType,
    /// Specifies the minimum number of the element type that can be installed in the chassis for
    /// the chassis to properly operate, in the range 0 to 254. The value 255 (0FFh) is reserved
    /// for future definition.
    minimum: u8,
    /// Specifies the maximum number of the element type that can be installed in the chassis, in
    /// the range 1 to 255. The value 0 is reserved for future definition.
    maximum: u8,
}

/// Identifies whether the Type contains an SMBIOS Baseboard Type enumeration or an SMBIOS
/// structure type enumeration.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ContainedElementType {
    BoardType(crate::baseboard::BoardType),
    InfoType(crate::InfoType),
}

impl<'buffer> Enclosure<'buffer> {
    pub(crate) fn try_from(structure: RawStructure<'buffer>) -> Result<Enclosure<'buffer>, MalformedStructureError> {
        #[repr(C)]
        #[repr(packed)]
        struct EnclosurePacked_2_3 {
            manufacturer: u8,
            enclosure_type: u8,
            version: u8,
            serial_number: u8,
            asset_tag_number: u8,
            boot_up_state: u8,
            power_supply_state: u8,
            thermal_state: u8,
            security_status: u8,
            oem_defined: u32,
            height: u8,
            power_cords_number: u8,
            contained_element_count: u8,
            contained_element_record_length: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct EnclosurePacked_2_1 {
            manufacturer: u8,
            enclosure_type: u8,
            version: u8,
            serial_number: u8,
            asset_tag_number: u8,
            boot_up_state: u8,
            power_supply_state: u8,
            thermal_state: u8,
            security_status: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct EnclosurePacked_2_0 {
            manufacturer: u8,
            enclosure_type: u8,
            version: u8,
            serial_number: u8,
            asset_tag_number: u8,
        }

        struct RawEnclosureType(u8);
        impl RawEnclosureType {
            fn new(byte: u8) -> Self {
                RawEnclosureType(byte)
            }
            fn get_lock(&self) -> bool {
                (self.0 & 0b1000_0000) != 0
            }
            fn get_type(&self) -> EnclosureType {
                (self.0 & 0b0111_1111).into()
            }
        }

        match structure.version {
            v if v >= (2, 7).into() => {
                let_as_struct!(packed, EnclosurePacked_2_3, structure.data);
                let enclosure_type = RawEnclosureType::new(packed.enclosure_type);
                let sku_number_string_field = 0x15
                    + packed.contained_element_count * packed.contained_element_record_length
                    - STRUCTURE_HEADER_LENGTH;
                let sku_number = structure
                    .data
                    .get(sku_number_string_field as usize)
                    .ok_or(crate::MalformedStructureError::BadSize(
                        sku_number_string_field as u32,
                        1,
                    ))
                    .and_then(|string_field| structure.find_string(*string_field))?;
                Ok(Enclosure {
                    handle: structure.handle,
                    manufacturer: structure.find_string(packed.manufacturer)?,
                    chassis_lock: enclosure_type.get_lock(),
                    enclosure_type: enclosure_type.get_type(),
                    version: structure.find_string(packed.version)?,
                    serial_number: structure.find_string(packed.serial_number)?,
                    asset_tag_number: structure.find_string(packed.asset_tag_number)?,
                    boot_up_state: Some(packed.boot_up_state.into()),
                    power_supply_state: Some(packed.power_supply_state.into()),
                    thermal_state: Some(packed.thermal_state.into()),
                    security_status: Some(packed.security_status.into()),
                    oem_defined: Some(packed.oem_defined),
                    height: Some(packed.height),
                    power_cords_number: Some(packed.power_cords_number),
                    contained_elements: Some(ContainedElements::new(
                        structure.data,
                        packed.contained_element_count,
                        packed.contained_element_record_length,
                    )),
                    sku_number: Some(sku_number),
                })
            }
            v if v >= (2, 3).into() => {
                let_as_struct!(packed, EnclosurePacked_2_3, structure.data);
                let enclosure_type = RawEnclosureType::new(packed.enclosure_type);
                Ok(Enclosure {
                    handle: structure.handle,
                    manufacturer: structure.find_string(packed.manufacturer)?,
                    chassis_lock: enclosure_type.get_lock(),
                    enclosure_type: enclosure_type.get_type(),
                    version: structure.find_string(packed.version)?,
                    serial_number: structure.find_string(packed.serial_number)?,
                    asset_tag_number: structure.find_string(packed.asset_tag_number)?,
                    boot_up_state: Some(packed.boot_up_state.into()),
                    power_supply_state: Some(packed.power_supply_state.into()),
                    thermal_state: Some(packed.thermal_state.into()),
                    security_status: Some(packed.security_status.into()),
                    oem_defined: Some(packed.oem_defined),
                    height: Some(packed.height),
                    power_cords_number: Some(packed.power_cords_number),
                    contained_elements: Some(ContainedElements::new(
                        structure.data,
                        packed.contained_element_count,
                        packed.contained_element_record_length,
                    )),
                    sku_number: None,
                })
            }
            v if v >= (2, 1).into() => {
                let_as_struct!(packed, EnclosurePacked_2_1, structure.data);
                let enclosure_type = RawEnclosureType::new(packed.enclosure_type);

                Ok(Enclosure {
                    handle: structure.handle,
                    manufacturer: structure.find_string(packed.manufacturer)?,
                    chassis_lock: enclosure_type.get_lock(),
                    enclosure_type: enclosure_type.get_type(),
                    version: structure.find_string(packed.version)?,
                    serial_number: structure.find_string(packed.serial_number)?,
                    asset_tag_number: structure.find_string(packed.asset_tag_number)?,
                    boot_up_state: Some(packed.boot_up_state.into()),
                    power_supply_state: Some(packed.power_supply_state.into()),
                    thermal_state: Some(packed.thermal_state.into()),
                    security_status: Some(packed.security_status.into()),
                    oem_defined: None,
                    height: None,
                    power_cords_number: None,
                    contained_elements: None,
                    sku_number: None,
                })
            }
            _ => {
                let_as_struct!(packed, EnclosurePacked_2_0, structure.data);
                let enclosure_type = RawEnclosureType::new(packed.enclosure_type);

                Ok(Enclosure {
                    handle: structure.handle,
                    manufacturer: structure.find_string(packed.manufacturer)?,
                    chassis_lock: enclosure_type.get_lock(),
                    enclosure_type: enclosure_type.get_type(),
                    version: structure.find_string(packed.version)?,
                    serial_number: structure.find_string(packed.serial_number)?,
                    asset_tag_number: structure.find_string(packed.asset_tag_number)?,
                    boot_up_state: None,
                    power_supply_state: None,
                    thermal_state: None,
                    security_status: None,
                    oem_defined: None,
                    height: None,
                    power_cords_number: None,
                    contained_elements: None,
                    sku_number: None,
                })
            }
        }
    }
}

impl From<u8> for EnclosureType {
    fn from(byte: u8) -> EnclosureType {
        match byte {
            0x01 => EnclosureType::Other,
            0x02 => EnclosureType::Unknown,
            0x03 => EnclosureType::Desktop,
            0x04 => EnclosureType::LowProfileDesktop,
            0x05 => EnclosureType::PizzaBox,
            0x06 => EnclosureType::MiniTower,
            0x07 => EnclosureType::Tower,
            0x08 => EnclosureType::Portable,
            0x09 => EnclosureType::Laptop,
            0x0A => EnclosureType::Notebook,
            0x0B => EnclosureType::HandHeld,
            0x0C => EnclosureType::DockingStation,
            0x0D => EnclosureType::AllInOne,
            0x0E => EnclosureType::SubNotebook,
            0x0F => EnclosureType::SpaceSaving,
            0x10 => EnclosureType::LunchBox,
            0x11 => EnclosureType::MainServerChassis,
            0x12 => EnclosureType::ExpansionChassis,
            0x13 => EnclosureType::SubChassis,
            0x14 => EnclosureType::BusExpansionChassis,
            0x15 => EnclosureType::PeripheralChassis,
            0x16 => EnclosureType::RaidChassis,
            0x17 => EnclosureType::RackMountChassis,
            0x18 => EnclosureType::SealedCasePc,
            0x19 => EnclosureType::MultiSystemChassis,
            0x1A => EnclosureType::CompactPci,
            0x1B => EnclosureType::AdvancedTca,
            0x1C => EnclosureType::Blade,
            0x1D => EnclosureType::BladeEnclosure,
            0x1E => EnclosureType::Tablet,
            0x1F => EnclosureType::Convertible,
            0x20 => EnclosureType::Detachable,
            0x21 => EnclosureType::IotGateway,
            0x22 => EnclosureType::EmbeddedPc,
            0x23 => EnclosureType::MiniPc,
            0x24 => EnclosureType::StickPc,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for EnclosureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Desktop => write!(f, "Desktop"),
            Self::LowProfileDesktop => write!(f, "Low Profile Desktop"),
            Self::PizzaBox => write!(f, "Pizza Box"),
            Self::MiniTower => write!(f, "Mini Tower"),
            Self::Tower => write!(f, "Tower"),
            Self::Portable => write!(f, "Portable"),
            Self::Laptop => write!(f, "Laptop"),
            Self::Notebook => write!(f, "Notebook"),
            Self::HandHeld => write!(f, "Hand Held"),
            Self::DockingStation => write!(f, "Docking Station"),
            Self::AllInOne => write!(f, "All in One"),
            Self::SubNotebook => write!(f, "Sub Notebook"),
            Self::SpaceSaving => write!(f, "Space-saving"),
            Self::LunchBox => write!(f, "Lunch Box"),
            Self::MainServerChassis => write!(f, "Main Server Chassis"),
            Self::ExpansionChassis => write!(f, "Expansion Chassis"),
            Self::SubChassis => write!(f, "SubChassis"),
            Self::BusExpansionChassis => write!(f, "Bus Expansion Chassis"),
            Self::PeripheralChassis => write!(f, "Peripheral Chassis"),
            Self::RaidChassis => write!(f, "RAID Chassis"),
            Self::RackMountChassis => write!(f, "Rack Mount Chassis"),
            Self::SealedCasePc => write!(f, "Sealed-case PC"),
            Self::MultiSystemChassis => write!(f, "Multi-system chassis"),
            Self::CompactPci => write!(f, "Compact PCI"),
            Self::AdvancedTca => write!(f, "Advanced TCA"),
            Self::Blade => write!(f, "Blade"),
            Self::BladeEnclosure => write!(f, "Blade Enclosure"),
            Self::Tablet => write!(f, "Tablet"),
            Self::Convertible => write!(f, "Convertible"),
            Self::Detachable => write!(f, "Detachable"),
            Self::IotGateway => write!(f, "IoT Gateway"),
            Self::EmbeddedPc => write!(f, "Embedded PC"),
            Self::MiniPc => write!(f, "Mini PC"),
            Self::StickPc => write!(f, "Stick PC"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for State {
    fn from(byte: u8) -> State {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Safe,
            0x04 => Self::Warning,
            0x05 => Self::Critical,
            0x06 => Self::NonRecoverable,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Safe => write!(f, "Safe"),
            Self::Warning => write!(f, "Warning"),
            Self::Critical => write!(f, "Critical"),
            Self::NonRecoverable => write!(f, "Non-recoverable"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for SecurityStatus {
    fn from(byte: u8) -> SecurityStatus {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::None,
            0x04 => Self::ExternalInterfaceLockedOut,
            0x05 => Self::ExternalInterfaceEnabled,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for SecurityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::None => write!(f, "None"),
            Self::ExternalInterfaceLockedOut => write!(f, "External interface locked out"),
            Self::ExternalInterfaceEnabled => write!(f, "External interface enabled"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl<'buffer> ContainedElements<'buffer> {
    fn new(data: &'buffer [u8], count: u8, record_length: u8) -> Self {
        if count == 0 || record_length == 0 {
            Default::default()
        } else {
            let length = (count * record_length) as usize;
            // 15h offset from SMBIOS Specification
            let offset = 0x15 - STRUCTURE_HEADER_LENGTH as usize;
            let chunks = data
                .get(offset..(offset + length))
                .unwrap_or_default()
                .chunks(record_length as usize);
            Self {
                chunks,
                count,
                record_length,
            }
        }
    }
    pub fn count(&self) -> u8 {
        self.count
    }
}
impl<'buffer> Default for ContainedElements<'buffer> {
    fn default() -> Self {
        Self {
            chunks: [].chunks(usize::MAX),
            count: 0,
            record_length: 0,
        }
    }
}
impl<'buffer> PartialEq for ContainedElements<'buffer> {
    fn eq(&self, other: &Self) -> bool {
        self.chunks.clone().eq(other.chunks.clone())
            && self.count == other.count
            && self.record_length == other.record_length
    }
}
impl<'buffer> Eq for ContainedElements<'buffer> {}
impl<'buffer> Hash for ContainedElements<'buffer> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chunks.clone().for_each(|c| c.hash(state));
        self.count.hash(state);
        self.record_length.hash(state);
    }
}
impl<'buffer> Iterator for ContainedElements<'buffer> {
    type Item = ContainedElement;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next().map(|a| a.into())
    }
}

impl From<&[u8]> for ContainedElement {
    fn from(data: &[u8]) -> ContainedElement {
        #[repr(C)]
        #[repr(packed)]
        struct ContainedElement_2_3 {
            type_: u8,
            minimum: u8,
            maximum: u8,
        }
        let_as_struct!(packed, ContainedElement_2_3, data);
        ContainedElement {
            type_: packed.type_.into(),
            minimum: packed.minimum,
            maximum: packed.maximum,
        }
    }
}
impl fmt::Display for ContainedElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}-{})", self.type_, self.minimum, self.maximum)
    }
}

impl From<u8> for ContainedElementType {
    fn from(byte: u8) -> ContainedElementType {
        let val = byte & 0b0111_1111;
        if byte & 0b1000_0000 == 0 {
            Self::BoardType(val.into())
        } else {
            Self::InfoType(val.into())
        }
    }
}
impl fmt::Display for ContainedElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BoardType(board) => write!(f, "Baseboard type: {}", board),
            Self::InfoType(info) => write!(f, "Structure type: {}", info),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    #[test]
    fn enclosure_type() {
        use super::EnclosureType::*;
        for i in 1..=0x24 {
            let (e, s) = match i {
                0x01 => (Other, "Other".into()),
                0x09 => (Laptop, "Laptop".into()),
                0x18 => (SealedCasePc, "Sealed-case PC".into()),
                0x22 => (EmbeddedPc, "Embedded PC".into()),
                v @ 0xF0..=0xFF => (Undefined(v), format!("Undefined: {}", v)),
                _ => continue,
            };
            assert_eq!(e, i.into(), "{:#x}", i);
            assert_eq!(s, format!("{}", e));
        }
    }

    #[test]
    fn state() {
        use super::State::*;
        for i in 0..=0xFF {
            let (e, s) = match i {
                0x01 => (Other, "Other".into()),
                0x04 => (Warning, "Warning".into()),
                0x06 => (NonRecoverable, "Non-recoverable".into()),
                v @ 0xF0..=0xFF => (Undefined(v), format!("Undefined: {}", v)),
                _ => continue,
            };
            assert_eq!(e, i.into(), "{:#x}", i);
            assert_eq!(s, format!("{}", e));
        }
    }

    #[test]
    fn security_status() {
        use super::SecurityStatus::*;
        for i in 0..=0xFF {
            let (e, s) = match i {
                0x01 => (Other, "Other".into()),
                0x03 => (None, "None".into()),
                0x05 => (ExternalInterfaceEnabled, "External interface enabled".into()),
                v @ 0xF0..=0xFF => (Undefined(v), format!("Undefined: {}", v)),
                _ => continue,
            };
            assert_eq!(e, i.into(), "{:#x}", i);
            assert_eq!(s, format!("{}", e));
        }
    }

    #[test]
    fn contained_element() {
        use super::{ContainedElement, ContainedElementType};
        let data = &[
            // Type contains an SMBIOS structure type
            (
                [0b1000_1001, 1, 2],
                ContainedElement {
                    type_: ContainedElementType::InfoType(crate::InfoType::SystemSlots),
                    minimum: 1,
                    maximum: 2,
                },
                "Structure type: System Slots (1-2)",
            ),
            // Type contains an SMBIOS Baseboard Type
            (
                [0b0000_0100, 1, 2],
                ContainedElement {
                    type_: ContainedElementType::BoardType(crate::baseboard::BoardType::ConnectivitySwitch),
                    minimum: 1,
                    maximum: 2,
                },
                "Baseboard type: Connectivity Switch (1-2)",
            ),
        ];
        for (array, contained_element, display) in data {
            let v = &ContainedElement::from(&array[..]);
            assert_eq!(contained_element, v);
            assert_eq!(format!("{}", display), format!("{}", v));
        }
    }

    #[test]
    fn contained_elements() {
        use super::{ContainedElement, ContainedElementType, ContainedElements};
        let structure_data = [
            0x01, 0x97, 0x00, 0x02, 0x00, 0x03, 0x03, 0x03, 0x02, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02,
            0x03, // we are interested in six bytes below
            0x91, 0x01, 0x02, 0x07, 0x03, 0x04, //
            0x03,
        ];
        let mut contained_elements = ContainedElements::new(&structure_data, 2, 3);
        if let Some(el) = contained_elements.next() {
            assert_eq!(
                ContainedElement {
                    type_: ContainedElementType::InfoType(crate::InfoType::MemoryDevice),
                    minimum: 1,
                    maximum: 2
                },
                el
            );
        }
        if let Some(el) = contained_elements.next() {
            assert_eq!(
                ContainedElement {
                    type_: ContainedElementType::BoardType(crate::baseboard::BoardType::IoModule),
                    minimum: 3,
                    maximum: 4
                },
                el
            );
        }
        assert_eq!(contained_elements.next(), None);
    }

    #[test]
    fn dmi_bin() {
        use super::*;
        const DMIDECODE_BIN: &'static [u8] = include_bytes!("../../tests/data/dmi.0.bin");
        let entry_point = crate::EntryPoint::search(DMIDECODE_BIN).unwrap();
        let enc = entry_point
            .structures(&DMIDECODE_BIN[(entry_point.smbios_address() as usize)..])
            .find_map(|s| {
                if let Ok(crate::Structure::Enclosure(enc)) = s {
                    Some(enc)
                } else {
                    None
                }
            })
            .unwrap();
        let sample = Enclosure {
            handle: 768,
            manufacturer: "Dell Inc.",
            chassis_lock: true,
            enclosure_type: EnclosureType::RackMountChassis,
            version: "",
            serial_number: "XXXXXXX",
            asset_tag_number: "",
            boot_up_state: Some(State::Safe),
            power_supply_state: Some(State::Safe),
            thermal_state: Some(State::Safe),
            security_status: Some(SecurityStatus::Unknown),
            oem_defined: Some(0x01010101),
            height: Some(2),
            power_cords_number: Some(2),
            contained_elements: Some(ContainedElements {
                chunks: [145, 1, 2, 3, 255, 0].chunks(3),
                count: 2,
                record_length: 3,
            }),
            sku_number: Some("SKU Number"),
        };

        assert_eq!(sample, enc);
        assert_eq!(format!("{}", enc.manufacturer), "Dell Inc.", "Manufacturer");
        assert_eq!(format!("{}", enc.enclosure_type), "Rack Mount Chassis", "Type");
        assert_eq!(format!("{}", enc.chassis_lock), "true", "Lock");
        assert_eq!(format!("{}", enc.version), "", "Version");
        assert_eq!(format!("{}", enc.serial_number), "XXXXXXX", "Serial Number");
        assert_eq!(format!("{}", enc.asset_tag_number), "", "Asset Tag");
        assert_eq!(
            enc.boot_up_state.map(|v| format!("{}", v)),
            Some("Safe".into()),
            "Boot-up State"
        );
        assert_eq!(
            enc.power_supply_state.map(|v| format!("{}", v)),
            Some("Safe".into()),
            "Power Supply State"
        );
        assert_eq!(
            enc.thermal_state.map(|v| format!("{}", v)),
            Some("Safe".into()),
            "Thermal State"
        );
        assert_eq!(
            enc.security_status.map(|v| format!("{}", v)),
            Some("Unknown".into()),
            "Security Status"
        );
        assert_eq!(
            enc.oem_defined.map(|v| format!("{:#010X}", v)),
            Some("0x01010101".into()),
            "OEM Information"
        );
        assert_eq!(enc.height, Some(2), "Height");
        assert_eq!(enc.power_cords_number, Some(2), "Number Of Power Cords");
        assert_eq!(
            enc.contained_elements
                .clone()
                .and_then(|mut ce| ce.nth(0).map(|s| format!("{}", s))),
            Some("Structure type: Memory Device (1-2)".into()),
            "Number Of Power Cords"
        );
        assert_eq!(
            enc.contained_elements
                .clone()
                .and_then(|mut ce| ce.nth(1).map(|s| format!("{}", s))),
            Some("Baseboard type: Server Blade (255-0)".into()),
            "Number Of Power Cords"
        );
        assert_eq!(
            enc.sku_number.map(|v| format!("{}", v)),
            Some("SKU Number".into()),
            "SKU Number"
        );
    }
}
