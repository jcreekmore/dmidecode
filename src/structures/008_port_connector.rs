//! Port Connector Information (Type 8)
//!
//! Information in this structure defines the attributes of a system port connector (for example,
//! parallel, serial, keyboard, or mouse ports). The port’s type and connector information are
//! provided. One structure is present for each port provided by the system.

use core::fmt;

use crate::{
    MalformedStructureError::{
        self,
    },
    RawStructure,
};

/// The `Port Connector Information` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq,)]
pub struct PortConnector<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// Internal Reference Designator, that is, internal to the system enclosure\
    /// EXAMPLE: ‘J101’, 0
    pub internal_reference_designator: &'a str,
    /// Internal Connector type
    pub internal_connector_type: ConnectorType,
    /// String number for the External Reference Designation external to the system enclosure\
    /// EXAMPLE: ‘COM A’, 0
    pub external_reference_designator:  &'a str,
    /// External Connector type
    pub external_connector_type: ConnectorType,
    /// Describes the function of the port
    pub port_type: PortType
}

#[doc(hidden)]

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq,)]
pub enum ConnectorType {
    None,
    Centronics,
    MiniCentronics,
    Proprietary,
    Db25PinMale,
    Db25PinFemale,
    Db15PinMale,
    Db15PinFemale,
    Db9PinMale,
    Db9PinFemale,
    Rj11,
    Rj45,
    MiniScsi,
    MiniDin,
    MicroDin,
    Ps2,
    Infrared,
    HpHil,
    AccessBus,
    SsaScsi,
    CircularDin8Male,
    CircularDin8Female,
    OnBoardIde,
    OnBoardFloppy,
    DualInline9,
    DualInline25,
    DualInline50,
    DualInline68,
    OnBoardSoundInputFromCdRom,
    MiniCentronicsType14,
    MiniCentronicsType26,
    MiniJack,
    Bnc,
    Ieee1394,
    SasSataPlugReceptacle,
    UsbTypeCReceptacle,
    Pc98,
    Pc98Hireso,
    PcH98,
    Pc98Note,
    Pc98Full,
    Other,
    Undefined(u8),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq,)]
pub enum PortType {
    None,
    ParallelPortXtAtCompatible,
    ParallelPortPs2,
    ParallelPortEcp,
    ParallelPortEpp,
    ParallelPortEcpEpp,
    SerialPortXtAtCompatible,
    SerialPort16450Compatible,
    SerialPort16550Compatible,
    SerialPort16550ACompatible,
    ScsiPort,
    MidiPort,
    JoyStickPort,
    KeyboardPort,
    MousePort,
    SsaScsi,
    Usb,
    FireWire,
    PcmciaType1,
    PcmciaType2,
    PcmciaType3,
    Cardbus,
    AccessBusPort,
    Scsi2,
    ScsiWide,
    Pc98,
    Pc98Hireso,
    PcH98,
    VideoPort,
    AudioPort,
    ModemPort,
    NetworkPort,
    Sata,
    Sas,
    MultiFunctionDisplayPort,
    Thunderbolt,
    Intel8251Compatible,
    Intel8251FifoCompatible,
    Other,
    Undefined(u8),
}


impl<'a> PortConnector<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<PortConnector<'a>, MalformedStructureError> {
        #[repr(C)]
        #[repr(packed)]
        struct PortConnectorPacked {
            internal_reference_designator: u8,
            internal_connector_type: u8,
            external_reference_designator: u8,
            external_connector_type: u8,
            port_type: u8,
        }
        let_as_struct!(packed, PortConnectorPacked, structure.data);

        Ok(PortConnector{
            handle: structure.handle,
            internal_reference_designator: structure.find_string(packed.internal_reference_designator)?,
            internal_connector_type: packed.internal_connector_type.into(),
            external_reference_designator: structure.find_string(packed.external_reference_designator)?,
            external_connector_type: packed.external_connector_type.into(),
            port_type: packed.port_type.into(),
        })

    }
}

impl From<u8> for ConnectorType {
    fn from(byte: u8) -> ConnectorType {
        match byte {
            0x00 => Self::None,
            0x01 => Self::Centronics,
            0x02 => Self::MiniCentronics,
            0x03 => Self::Proprietary,
            0x04 => Self::Db25PinMale,
            0x05 => Self::Db25PinFemale,
            0x06 => Self::Db15PinMale,
            0x07 => Self::Db15PinFemale,
            0x08 => Self::Db9PinMale,
            0x09 => Self::Db9PinFemale,
            0x0A => Self::Rj11,
            0x0B => Self::Rj45,
            0x0C => Self::MiniScsi,
            0x0D => Self::MiniDin,
            0x0E => Self::MicroDin,
            0x0F => Self::Ps2,
            0x10 => Self::Infrared,
            0x11 => Self::HpHil,
            0x12 => Self::AccessBus,
            0x13 => Self::SsaScsi,
            0x14 => Self::CircularDin8Male,
            0x15 => Self::CircularDin8Female,
            0x16 => Self::OnBoardIde,
            0x17 => Self::OnBoardFloppy,
            0x18 => Self::DualInline9,
            0x19 => Self::DualInline25,
            0x1A => Self::DualInline50,
            0x1B => Self::DualInline68,
            0x1C => Self::OnBoardSoundInputFromCdRom,
            0x1D => Self::MiniCentronicsType14,
            0x1E => Self::MiniCentronicsType26,
            0x1F => Self::MiniJack,
            0x20 => Self::Bnc,
            0x21 => Self::Ieee1394,
            0x22 => Self::SasSataPlugReceptacle,
            0x23 => Self::UsbTypeCReceptacle,
            0xA0 => Self::Pc98,
            0xA1 => Self::Pc98Hireso,
            0xA2 => Self::PcH98,
            0xA3 => Self::Pc98Note,
            0xA4 => Self::Pc98Full,
            0xFF => Self::Other,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None                       => write!(f, "None"),
            Self::Centronics                 => write!(f, "Centronics"),
            Self::MiniCentronics             => write!(f, "Mini Centronics"),
            Self::Proprietary                => write!(f, "Proprietary"),
            Self::Db25PinMale                => write!(f, "DB-25 pin male"),
            Self::Db25PinFemale              => write!(f, "DB-25 pin female"),
            Self::Db15PinMale                => write!(f, "DB-15 pin male"),
            Self::Db15PinFemale              => write!(f, "DB-15 pin female"),
            Self::Db9PinMale                 => write!(f, "DB-9 pin male"),
            Self::Db9PinFemale               => write!(f, "DB-9 pin female"),
            Self::Rj11                       => write!(f, "RJ-11"),
            Self::Rj45                       => write!(f, "RJ-45"),
            Self::MiniScsi                   => write!(f, "50-pin MiniSCSI"),
            Self::MiniDin                    => write!(f, "Mini-DIN"),
            Self::MicroDin                   => write!(f, "Micro-DIN"),
            Self::Ps2                        => write!(f, "PS/2"),
            Self::Infrared                   => write!(f, "Infrared"),
            Self::HpHil                      => write!(f, "HP-HIL"),
            Self::AccessBus                  => write!(f, "Access Bus (USB)"),
            Self::SsaScsi                    => write!(f, "SSA SCSI"),
            Self::CircularDin8Male           => write!(f, "Circular DIN-8 male"),
            Self::CircularDin8Female         => write!(f, "Circular DIN-8 female"),
            Self::OnBoardIde                 => write!(f, "On Board IDE"),
            Self::OnBoardFloppy              => write!(f, "On Board Floppy"),
            Self::DualInline9                => write!(f, "9-pin Dual Inline (pin 10 cut)"),
            Self::DualInline25               => write!(f, "25-pin Dual Inline (pin 26 cut)"),
            Self::DualInline50               => write!(f, "50-pin Dual Inline"),
            Self::DualInline68               => write!(f, "68-pin Dual Inline"),
            Self::OnBoardSoundInputFromCdRom => write!(f, "On Board Sound Input from CD-ROM"),
            Self::MiniCentronicsType14       => write!(f, "Mini-Centronics Type-14"),
            Self::MiniCentronicsType26       => write!(f, "Mini-Centronics Type-26"),
            Self::MiniJack                   => write!(f, "Mini-jack (headphones)"),
            Self::Bnc                        => write!(f, "BNC"),
            Self::Ieee1394                   => write!(f, "1394"),
            Self::SasSataPlugReceptacle      => write!(f, "SAS/SATA Plug Receptacle"),
            Self::UsbTypeCReceptacle         => write!(f, "USB Type-C Receptacle"),
            Self::Pc98                       => write!(f, "PC-98"),
            Self::Pc98Hireso                 => write!(f, "PC-98Hireso"),
            Self::PcH98                      => write!(f, "PC-H98"),
            Self::Pc98Note                   => write!(f, "PC-98Note"),
            Self::Pc98Full                   => write!(f, "PC-98Full"),
            Self::Other                      => write!(f, "Other – Use Reference Designator Strings to supply information."),
            Self::Undefined(v)               => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for PortType {
    fn from(byte: u8) -> PortType {
        match byte {
            0x00 => PortType::None,
            0x01 => PortType::ParallelPortXtAtCompatible,
            0x02 => PortType::ParallelPortPs2,
            0x03 => PortType::ParallelPortEcp,
            0x04 => PortType::ParallelPortEpp,
            0x05 => PortType::ParallelPortEcpEpp,
            0x06 => PortType::SerialPortXtAtCompatible,
            0x07 => PortType::SerialPort16450Compatible,
            0x08 => PortType::SerialPort16550Compatible,
            0x09 => PortType::SerialPort16550ACompatible,
            0x0A => PortType::ScsiPort,
            0x0B => PortType::MidiPort,
            0x0C => PortType::JoyStickPort,
            0x0D => PortType::KeyboardPort,
            0x0E => PortType::MousePort,
            0x0F => PortType::SsaScsi,
            0x10 => PortType::Usb,
            0x11 => PortType::FireWire,
            0x12 => PortType::PcmciaType1,
            0x13 => PortType::PcmciaType2,
            0x14 => PortType::PcmciaType3,
            0x15 => PortType::Cardbus,
            0x16 => PortType::AccessBusPort,
            0x17 => PortType::Scsi2,
            0x18 => PortType::ScsiWide,
            0x19 => PortType::Pc98,
            0x1A => PortType::Pc98Hireso,
            0x1B => PortType::PcH98,
            0x1C => PortType::VideoPort,
            0x1D => PortType::AudioPort,
            0x1E => PortType::ModemPort,
            0x1F => PortType::NetworkPort,
            0x20 => PortType::Sata,
            0x21 => PortType::Sas,
            0x22 => PortType::MultiFunctionDisplayPort,
            0x23 => PortType::Thunderbolt,
            0xA0 => PortType::Intel8251Compatible,
            0xA1 => PortType::Intel8251FifoCompatible,
            0xFF => PortType::Other,
            v => PortType::Undefined(v),
        }
    }
}
impl fmt::Display for PortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortType::None                       => write!(f, "None"),
            PortType::ParallelPortXtAtCompatible => write!(f, "Parallel Port XT/AT Compatible"),
            PortType::ParallelPortPs2            => write!(f, "Parallel Port PS/2"),
            PortType::ParallelPortEcp            => write!(f, "Parallel Port ECP"),
            PortType::ParallelPortEpp            => write!(f, "Parallel Port EPP"),
            PortType::ParallelPortEcpEpp         => write!(f, "Parallel Port ECP/EPP"),
            PortType::SerialPortXtAtCompatible   => write!(f, "Serial Port XT/AT Compatible"),
            PortType::SerialPort16450Compatible  => write!(f, "Serial Port 16450 Compatible"),
            PortType::SerialPort16550Compatible  => write!(f, "Serial Port 16550 Compatible"),
            PortType::SerialPort16550ACompatible => write!(f, "Serial Port 16550A Compatible"),
            PortType::ScsiPort                   => write!(f, "SCSI Port"),
            PortType::MidiPort                   => write!(f, "MIDI Port"),
            PortType::JoyStickPort               => write!(f, "Joy Stick Port"),
            PortType::KeyboardPort               => write!(f, "Keyboard Port"),
            PortType::MousePort                  => write!(f, "Mouse Port"),
            PortType::SsaScsi                    => write!(f, "SSA SCSI"),
            PortType::Usb                        => write!(f, "USB"),
            PortType::FireWire                   => write!(f, "FireWire (IEEE P1394)"),
            PortType::PcmciaType1                => write!(f, "PCMCIA Type I2"),
            PortType::PcmciaType2                => write!(f, "PCMCIA Type II"),
            PortType::PcmciaType3                => write!(f, "PCMCIA Type III"),
            PortType::Cardbus                    => write!(f, "Cardbus"),
            PortType::AccessBusPort              => write!(f, "Access Bus Port"),
            PortType::Scsi2                      => write!(f, "SCSI II"),
            PortType::ScsiWide                   => write!(f, "SCSI Wide"),
            PortType::Pc98                       => write!(f, "PC-98"),
            PortType::Pc98Hireso                 => write!(f, "PC-98-Hireso"),
            PortType::PcH98                      => write!(f, "PC-H98"),
            PortType::VideoPort                  => write!(f, "Video Port"),
            PortType::AudioPort                  => write!(f, "Audio Port"),
            PortType::ModemPort                  => write!(f, "Modem Port"),
            PortType::NetworkPort                => write!(f, "Network Port"),
            PortType::Sata                       => write!(f, "SATA"),
            PortType::Sas                        => write!(f, "SAS"),
            PortType::MultiFunctionDisplayPort   => write!(f, "MFDP (Multi-Function Display Port)"),
            PortType::Thunderbolt                => write!(f, "Thunderbolt"),
            PortType::Intel8251Compatible        => write!(f, "8251 Compatible"),
            PortType::Intel8251FifoCompatible    => write!(f, "8251 FIFO Compatible"),
            PortType::Other                      => write!(f, "Other"),
            PortType::Undefined(v)               => write!(f, "Undefined: {}", v),
        }
    }
}


#[cfg(test)]
mod test {
    use std::prelude::v1::*;
    use pretty_assertions::{assert_eq,};
    #[test]
    fn connector_type() {
        use super::ConnectorType;
        let samples = &[ 
            (0x01, ConnectorType::Centronics, "Centronics"),
            (0x12, ConnectorType::AccessBus, "Access Bus (USB)"),
            (0xA3, ConnectorType::Pc98Note, "PC-98Note"),
            (0xFE, ConnectorType::Undefined(254), "Undefined: 254"),
            (0xFF, ConnectorType::Other, "Other – Use Reference Designator Strings to supply information."),
        ];
        let result = samples.iter().map(|v| Into::into(v.0)).collect::<Vec<_>>();
        assert_eq!(
            samples.iter().map(|(_, v, s)| (v, (*s).into())).collect::<Vec<_>>(),
            result.iter().map(|r| (r, format!("{}", r))).collect::<Vec<_>>(),
        );

    }
    #[test]
    fn port_type() {
        use super::PortType;
        let samples = &[ 
            (0x00, PortType::None, "None"),
            (0x11, PortType::FireWire, "FireWire (IEEE P1394)"),
            (0xA1, PortType::Intel8251FifoCompatible, "8251 FIFO Compatible"),
            (0xFF, PortType::Other, "Other"),
            (0xFE, PortType::Undefined(254), "Undefined: 254"),
        ];
        let result = samples.iter().map(|v| Into::into(v.0)).collect::<Vec<_>>();
        assert_eq!(
            samples.iter().map(|(_, v, s)| (v, (*s).into())).collect::<Vec<_>>(),
            result.iter().map(|r| (r, format!("{}", r))).collect::<Vec<_>>(),
        );

    }
    #[test]
    fn port_connector() {
        use crate::{RawStructure,InfoType};
        use super::{PortConnector, ConnectorType, PortType};
        let sample = PortConnector {
            handle: 8,
            internal_reference_designator: "J1A1",
            internal_connector_type: ConnectorType::None,
            external_reference_designator: "Keyboard",
            external_connector_type: ConnectorType::Ps2,
            port_type: PortType::KeyboardPort,
        };
        let structure = RawStructure {
            version: (0, 0).into(),
            info: InfoType::PortConnector,
            length: 0,
            handle: 0x0008,
            // Remove 4 bytes from `dmidecode -H 8 -u` 'Header and Data'
            data: &[0x01,0x00,0x02,0x0F,0x0D],
            strings: &[
                // J1A1
                0x4A,0x31,0x41,0x31,0x00,
                // Keyboard
                0x4B,0x65,0x79,0x62,0x6F,0x61,0x72,0x64,0x00,
            ],
        };
        let result = PortConnector::try_from(structure).unwrap();
        assert_eq!(sample, result);
    }
    #[test]
    fn dmi_bin() {
        use crate::{Structure, EntryPoint,};
        use super::*;
        const DMIDECODE_BIN: &'static [u8] = include_bytes!("../../tests/data/dmi.0.bin");
        let entry_point = EntryPoint::search(DMIDECODE_BIN).unwrap();
        let connectors = entry_point
            .structures(&DMIDECODE_BIN[(entry_point.smbios_address() as usize)..])
            .filter_map(|s| s.ok().filter(|s| matches!(s, Structure::PortConnector(_))))
            .collect::<Vec<_>>();

        let usb_sample = PortConnector {
            handle: 0x0800,
            internal_reference_designator: "Internal USB port 1",
            internal_connector_type: ConnectorType::AccessBus,
            external_reference_designator: "",
            external_connector_type: ConnectorType::None,
            port_type: PortType::Usb,
        };
        let usb_result = connectors.iter()
            .find_map(|s| {
                match s {
                    Structure::PortConnector(pc) if pc.handle == 0x0800 => Some(pc),
                    _ => None,
                }
            })
            .unwrap();
        assert_eq!(&usb_sample, usb_result, "USB");
        assert_eq!("Access Bus (USB)", format!("{}", usb_result.internal_connector_type), "USB: Internal Connector Type");
        assert_eq!("None", format!("{}", usb_result.external_connector_type), "USB: External Connector Type");
        assert_eq!("USB", format!("{}", usb_result.port_type), "USB: Port Type");

        let rj45_sample = PortConnector {
            handle: 0x080A,
            internal_reference_designator: "",
            internal_connector_type: ConnectorType::None,
            external_reference_designator: "4",
            external_connector_type: ConnectorType::Rj45,
            port_type: PortType::NetworkPort,
        };
        let rj45_result = connectors.iter()
            .find_map(|s| {
                match s {
                    Structure::PortConnector(pc) if pc.handle == 0x080A => Some(pc),
                    _ => None,
                }
            })
            .unwrap();
        assert_eq!(&rj45_sample, rj45_result, "RJ-45");
        assert_eq!("None", format!("{}", rj45_result.internal_connector_type), "RJ-45: Internal Connector Type");
        assert_eq!("RJ-45", format!("{}", rj45_result.external_connector_type), "RJ-45: External Connector Type");
        assert_eq!("Network Port", format!("{}", rj45_result.port_type), "RJ-45: Port Type");
    }
}
