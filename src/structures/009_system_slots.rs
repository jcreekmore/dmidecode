//! System Slots (Type 9)
//!
//! Information in this structure defines the attributes of a system slot. One structure is
//! provided for each slot in the system.

use core::fmt;
use core::hash::{Hash, Hasher};
use core::slice::Chunks;

use crate::{
    bitfield::{BitField, FlagType, Layout},
    InfoType,
    MalformedStructureError::{self, InvalidFormattedSectionLength},
    RawStructure,
};

/// The `System Slots` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SystemSlots<'a> {
    /// Specifies the structure’s handle
    pub handle: u16,
    /// String number for reference designation\
    /// EXAMPLE: ‘PCI-1’,0
    pub slot_designation: &'a str,
    /// Slot Type field
    pub slot_type: SlotType,
    /// Slot Data Bus Width field
    pub slot_data_bus_width: SlotWidth,
    /// Current Usage field
    pub current_usage: CurrentUsage,
    /// Slot Length field
    pub slot_length: SlotLength,
    /// The Slot ID field of the System Slot structure provides a mechanism to correlate the
    /// physical attributes of the slot to its logical access method (which varies based on the
    /// Slot Type field).
    pub slot_id: u16,
    /// Slot Characteristics 1 field
    pub slot_characteristics_1: SlotCharacteristics1,
    /// Slot Characteristics 2 field
    pub slot_characteristics_2: Option<SlotCharacteristics2>,
    /// Segment Group Number is defined in the PCI Firmware Specification. The value is 0 for a
    /// single-segment topology.
    pub segment_group_number: Option<u16>,
    /// For PCI Express slots, Bus Number refer to the endpoint in the slot, not the upstream switch.
    pub bus_number: Option<u8>,
    /// For PCI Express slots, Device/Function Number refer to the endpoint in the slot, not the
    /// upstream switch.
    pub device_and_function_number: Option<DeviceAndFunctionNumber>,
    /// Indicate electrical bus width of base Segment/Bus/Device/Function/Width
    pub data_bus_width: Option<u8>,
    /// Because some slots can be partitioned into smaller electrical widths, additional peer
    /// device Segment/Bus/Device/Function are defined.\
    /// This definition does not cover children devices i.e., devices behind a PCIe bridge in the slot.
    pub peer_devices: Option<PeerDevices<'a>>,
    /// The contents of this field depend on what is contained in the Slot Type field. For Slot
    /// Type of C4h this field must contain the numeric value of the PCI Express Generation (e.g.,
    /// Gen6 would be 06h). For other PCI Express Slot Types, this field may be used but it is not
    /// required (if not used it should be set to 00h). For all other Slot Types, this field
    /// should be set to 00h.
    pub slot_information: Option<u8>,
    /// This field indicates the physical width of the slot whereas [Slot Data Bus
    /// Width](SlotWidth) indicates the electrical width of the slot
    pub slot_physical_width: Option<SlotWidth>,
    /// The Slot Pitch field contains a numeric value that indicates the pitch of the slot in units
    /// of 1/100 millimeter.
    pub slot_pitch: Option<SlotPitch>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SlotType {
    Other,
    Unknown,
    /// ISA
    Isa,
    /// MCA
    Mca,
    /// EISA
    Eisa,
    /// PCI
    Pci,
    /// PC Card (PCMCIA)
    PcCard,
    /// VL-VESA
    VlVesa,
    Proprietary,
    ProcessorCardSlot,
    ProprietaryMemoryCardSlot,
    /// I/O Riser Card Slot
    IoRiserCardSlot,
    /// NuBus
    Nubus,
    /// PCI – 66MHz Capable
    Pci66Mhz,
    /// AGP
    Agp,
    /// AGP 2X
    Agp2x,
    /// AGP 4X
    Agp4x,
    /// PCI-X
    PciX,
    /// AGP 8X
    Agp8x,
    /// M.2 Socket 1-DP (Mechanical Key A)
    M2Socket1DP,
    /// M.2 Socket 1-SD (Mechanical Key E)
    M2Socket1SD,
    /// M.2 Socket 2 (Mechanical Key B)
    M2Socket2,
    /// M.2 Socket 3 (Mechanical Key M)
    M2Socket3,
    /// MXM Type I
    MxmType1,
    /// MXM Type II
    MxmType2,
    /// MXM Type III (standard connector)
    MxmType3,
    /// MXM Type III (HE connector)
    MxmType3He,
    /// MXM Type IV
    MxmType4,
    /// MXM 3.0 Type A
    Mxm3TypeA,
    /// MXM 3.0 Type B
    Mxm3TypeB,
    /// PCI Express Gen 2 SFF-8639 (U.2)
    U2PciExpressGen2,
    /// PCI Express Gen 3 SFF-8639 (U.2)
    U2PciExpressGen3,
    /// PCI Express Mini 52-pin (CEM spec. 2.0) with bottom-side keep-outs. Use Slot Length field value
    /// 03h (short length) for "half-Mini card" -only support, 04h (long length) for "full-Mini card"
    /// or dual support.
    PciExpressMini52pin1,
    /// PCI Express Mini 52-pin (CEM spec. 2.0) without bottom-side keep-outs. Use Slot Length
    /// field value 03h (short length) for "half-Mini card" -only support, 04h (long length) for
    /// "full-Mini card" or dual support.
    PciExpressMini52pin2,
    /// PCI Express Mini 76-pin (CEM spec. 2.0) Corresponds to Display-Mini card.
    PciExpressMini76pin,
    /// PCI Express Gen 4 SFF-8639 (U.2)
    U2PciExpressGen4,
    /// PCI Express Gen 5 SFF-8639 (U.2)
    U2PciExpressGen5,
    /// OCP NIC 3.0 Small Form Factor (SFF)
    OcpNic3Small,
    /// OCP NIC 3.0 Large Form Factor (LFF)
    OcpNic3Large,
    /// OCP NIC Prior to 3.0
    OcpNicPriorTo3,
    /// CXL Flexbus 1.0 (deprecated)
    ///
    CxlFlexbus1,
    /// PC-98/C20
    Pc98C20,
    /// PC-98/C24
    Pc98C24,
    /// PC-98/E
    Pc98E,
    /// PC-98/Local Bus
    Pc98LocalBus,
    /// PC-98/Card
    Pc98Card,
    /// PCI Express
    PciExpress,
    /// PCI Express x1
    PciExpressX1,
    /// PCI Express x2
    PciExpressX2,
    /// PCI Express x4
    PciExpressX4,
    /// PCI Express x8
    PciExpressX8,
    /// PCI Express x16
    PciExpressX16,
    /// PCI Express Gen 2
    PciExpressGen2,
    /// PCI Express Gen 2 x1
    PciExpressGen2x1,
    /// PCI Express Gen 2 x2
    PciExpressGen2x2,
    /// PCI Express Gen 2 x4
    PciExpressGen2x4,
    /// PCI Express Gen 2 x8
    PciExpressGen2x8,
    /// PCI Express Gen 2 x16
    PciExpressGen2x16,
    /// PCI Express Gen 3
    PciExpressGen3,
    /// PCI Express Gen 3 x1
    PciExpressGen3x1,
    /// PCI Express Gen 3 x2
    PciExpressGen3x2,
    /// PCI Express Gen 3 x4
    PciExpressGen3x4,
    /// PCI Express Gen 3 x8
    PciExpressGen3x8,
    /// PCI Express Gen 3 x16
    PciExpressGen3x16,
    /// PCI Express Gen 4
    PciExpressGen4,
    /// PCI Express Gen 4 x1
    PciExpressGen4x1,
    /// PCI Express Gen 4 x2
    PciExpressGen4x2,
    /// PCI Express Gen 4 x4
    PciExpressGen4x4,
    /// PCI Express Gen 4 x8
    PciExpressGen4x8,
    /// PCI Express Gen 4 x16
    PciExpressGen4x16,
    /// PCI Express Gen 5
    PciExpressGen5,
    /// PCI Express Gen 5 x1
    PciExpressGen5x1,
    /// PCI Express Gen 5 x2
    PciExpressGen5x2,
    /// PCI Express Gen 5 x4
    PciExpressGen5x4,
    /// PCI Express Gen 5 x8
    PciExpressGen5x8,
    /// PCI Express Gen 5 x16
    PciExpressGen5x16,
    /// PCI Express Gen 6 and Beyond
    PciExpressGen6,
    /// Enterprise and Datacenter 1U E1 Form Factor Slot (EDSFF E1.S, E1.L). See specifications
    /// SFF-TA-1006 and SFF-TA-1007 for more details on values for slot length and pitch.
    E1FormFactorSlot,
    /// Enterprise and Datacenter 3" E3 Form Factor Slot (EDSFF E3.S, E3.L). See specification
    /// SFF-TA-1008 for details on values for slot length and pitch.
    E3FormFactorSlot,
    Undefined(u8),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SlotWidth {
    Other,
    Unknown,
    /// 8 bit
    Byte,
    /// 16 bit
    Word,
    /// 32 bit
    Dword,
    /// 64 bit
    Qword,
    /// 128 bit
    Dqword,
    /// 1x or x1
    X1,
    /// 2x or x2
    X2,
    /// 4x or x4
    X4,
    /// 8x or x8
    X8,
    /// 12x or x12
    X12,
    /// 16x or x16
    X16,
    /// 32x or x32
    X32,
    Undefined(u8),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CurrentUsage {
    Other,
    Unknown,
    Available,
    /// In use
    InUse,
    /// Unavailable (e.g., connected to a processor that is not installed)
    Unavailable,
    Undefined(u8),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SlotLength {
    Other,
    Unknown,
    ShortLength,
    LongLength,
    /// 2.5" drive form factor
    DriveFormFactor2_5,
    /// 3.5" drive form factor
    DriveFormFactor3_5,
    Undefined(u8),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SlotCharacteristics1(u8);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SlotCharacteristics2(u8);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Device {
    /// Segment Group Number is defined in the PCI Firmware Specification. The value is 0 for a
    /// single-segment topology.
    pub segment_group_number: u16,
    /// For PCI Express slots, Bus Number refer to the endpoint in the slot, not the upstream switch.
    pub bus_number: u8,
    /// For PCI Express slots, Device/Function Number refer to the endpoint in the slot, not the
    /// upstream switch.
    pub device_and_function_number: DeviceAndFunctionNumber,
    /// Indicate electrical bus width of peer Segment/Bus/Device/Function/Width
    pub data_bus_width: u8,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeviceAndFunctionNumber(u8, u8);

// Used in 2 Base Device and in Peer Devices
#[repr(C)]
#[repr(packed)]
struct DevicePacked {
    segment_group_number: u16,
    bus_number: u8,
    dev_and_fun_number: u8,
    data_bus_width: u8,
}

/// An iterator over Peer Segment/Bus/Device/Function/Width groups
#[derive(Clone, Debug)]
pub struct PeerDevices<'a>(Chunks<'a, u8>);

/// The Slot Pitch field contains a numeric value that indicates the pitch of the slot in units of
/// 1/100 millimeter.
///
/// The pitch is defined by each slot/card specification, but typically
/// describes add-in card to add-in card pitch.  For EDSFF slots, the pitch is defined in
/// SFF-TA-1006 table 7.1, SFF-TA-1007 table 7.1 (add-in card to add-in card pitch), and
/// SFF-TA-1008 table 6-1 (SSD to SSD pitch).  For example, if the pitch for the slot is 12.5 mm,
/// the value 1250 would be used.  A value of 0 implies that the slot pitch is not given or is
/// unknown.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SlotPitch(u16);

impl<'a> SystemSlots<'a> {
    pub(crate) fn try_from(structure: RawStructure<'a>) -> Result<SystemSlots<'a>, MalformedStructureError> {
        let data_len = structure.data.len() + 4;
        let handle = structure.handle;
        match ((structure.version.major, structure.version.minor), data_len) {
            (v, l) if ((2, 0)..(2, 1)).contains(&v) && l != 0x0C => {
                Err(InvalidFormattedSectionLength(InfoType::SystemSlots, handle, "", 0x0C))
            }
            (v, l) if ((2, 1)..(2, 6)).contains(&v) && l != 0x0D => {
                Err(InvalidFormattedSectionLength(InfoType::SystemSlots, handle, "", 0x0D))
            }
            (v, l) if ((2, 6)..(3, 2)).contains(&v) && l != 0x11 => {
                Err(InvalidFormattedSectionLength(InfoType::SystemSlots, handle, "", 0x11))
            }
            (v, l) if v >= (3, 2) && l < 0x11 => Err(InvalidFormattedSectionLength(
                InfoType::SystemSlots,
                handle,
                "minimum of ",
                0x11,
            )),
            _ => {
                let peer_grouping_count: u8 = structure.get::<u8>(0x12).unwrap_or(0);
                let n = peer_grouping_count as usize;
                Ok(SystemSlots {
                    handle,
                    slot_designation: structure.get_string(0x04)?,
                    slot_type: structure.get::<u8>(0x05)?.into(),
                    slot_data_bus_width: structure.get::<u8>(0x06)?.into(),
                    current_usage: structure.get::<u8>(0x07)?.into(),
                    slot_length: structure.get::<u8>(0x08)?.into(),
                    slot_id: structure.get::<u16>(0x09)?,
                    slot_characteristics_1: structure.get::<u8>(0x0B)?.into(),
                    slot_characteristics_2: structure.get::<u8>(0x0C).ok().map(Into::into),
                    segment_group_number: structure
                        .get::<u16>(0x0D)
                        .ok()
                        // For slots that do not have bus/device/function information FFh should be populated
                        .filter(|v| v != &0xFFFF),
                    bus_number: structure
                        .get::<u8>(0x0F)
                        .ok()
                        // For slots that do not have bus/device/function information FFh should be populated
                        .filter(|v| v != &0xFF),
                    device_and_function_number: structure
                        .get::<u8>(0x10)
                        .ok()
                        // For slots that do not have bus/device/function information FFh should be populated
                        .filter(|v| v != &0xFF)
                        .map(Into::into),
                    data_bus_width: structure.get::<u8>(0x11).ok(),
                    peer_devices: structure.get_slice(0x13, 5 * n).map(Into::into),
                    /// According to (SMBIOS Reference Specification
                    /// 3.4)[https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf]
                    /// fields below starts from offset 14h + 5*n, that looks like mistake.
                    /// It shoud start from 13h + 5*n, because *Peer (S/B/D/F/Width)
                    /// groups* field may has zero length
                    slot_information: structure.get::<u8>(0x14 + 5 * n).ok(),
                    slot_physical_width: structure.get::<u8>(0x15 + 5 * n).ok().map(Into::into),
                    slot_pitch: structure.get::<u16>(0x16 + 5 * n).ok().map(Into::into),
                })
            }
        }
    }
}

impl From<u8> for SlotType {
    fn from(byte: u8) -> SlotType {
        match byte {
            0x01 => SlotType::Other,
            0x02 => SlotType::Unknown,
            0x03 => SlotType::Isa,
            0x04 => SlotType::Mca,
            0x05 => SlotType::Eisa,
            0x06 => SlotType::Pci,
            0x07 => SlotType::PcCard,
            0x08 => SlotType::VlVesa,
            0x09 => SlotType::Proprietary,
            0x0A => SlotType::ProcessorCardSlot,
            0x0B => SlotType::ProprietaryMemoryCardSlot,
            0x0C => SlotType::IoRiserCardSlot,
            0x0D => SlotType::Nubus,
            0x0E => SlotType::Pci66Mhz,
            0x0F => SlotType::Agp,
            0x10 => SlotType::Agp2x,
            0x11 => SlotType::Agp4x,
            0x12 => SlotType::PciX,
            0x13 => SlotType::Agp8x,
            0x14 => SlotType::M2Socket1DP,
            0x15 => SlotType::M2Socket1SD,
            0x16 => SlotType::M2Socket2,
            0x17 => SlotType::M2Socket3,
            0x18 => SlotType::MxmType1,
            0x19 => SlotType::MxmType2,
            0x1A => SlotType::MxmType3,
            0x1B => SlotType::MxmType3He,
            0x1C => SlotType::MxmType4,
            0x1D => SlotType::Mxm3TypeA,
            0x1E => SlotType::Mxm3TypeB,
            0x1F => SlotType::PciExpressGen2,
            0x20 => SlotType::PciExpressGen3,
            0x21 => SlotType::PciExpressMini52pin1,
            0x22 => SlotType::PciExpressMini52pin2,
            0x23 => SlotType::PciExpressMini76pin,
            0x24 => SlotType::PciExpressGen4,
            0x25 => SlotType::PciExpressGen5,
            0x26 => SlotType::OcpNic3Small,
            0x27 => SlotType::OcpNic3Large,
            0x28 => SlotType::OcpNicPriorTo3,
            0x30 => SlotType::CxlFlexbus1,
            0xA0 => SlotType::Pc98C20,
            0xA1 => SlotType::Pc98C24,
            0xA2 => SlotType::Pc98E,
            0xA3 => SlotType::Pc98LocalBus,
            0xA4 => SlotType::Pc98Card,
            0xA5 => SlotType::PciExpress,
            0xA6 => SlotType::PciExpressX1,
            0xA7 => SlotType::PciExpressX2,
            0xA8 => SlotType::PciExpressX4,
            0xA9 => SlotType::PciExpressX8,
            0xAA => SlotType::PciExpressX16,
            0xAB => SlotType::PciExpressGen2,
            0xAC => SlotType::PciExpressGen2x1,
            0xAD => SlotType::PciExpressGen2x2,
            0xAE => SlotType::PciExpressGen2x4,
            0xAF => SlotType::PciExpressGen2x8,
            0xB0 => SlotType::PciExpressGen2x16,
            0xB1 => SlotType::PciExpressGen3,
            0xB2 => SlotType::PciExpressGen3x1,
            0xB3 => SlotType::PciExpressGen3x2,
            0xB4 => SlotType::PciExpressGen3x4,
            0xB5 => SlotType::PciExpressGen3x8,
            0xB6 => SlotType::PciExpressGen3x16,
            0xB8 => SlotType::PciExpressGen4,
            0xB9 => SlotType::PciExpressGen4x1,
            0xBA => SlotType::PciExpressGen4x2,
            0xBB => SlotType::PciExpressGen4x4,
            0xBC => SlotType::PciExpressGen4x8,
            0xBD => SlotType::PciExpressGen4x16,
            0xBE => SlotType::PciExpressGen5,
            0xBF => SlotType::PciExpressGen5x1,
            0xC0 => SlotType::PciExpressGen5x2,
            0xC1 => SlotType::PciExpressGen5x4,
            0xC2 => SlotType::PciExpressGen5x8,
            0xC3 => SlotType::PciExpressGen5x16,
            0xC4 => SlotType::PciExpressGen6,
            0xC5 => SlotType::E1FormFactorSlot,
            0xC6 => SlotType::E3FormFactorSlot,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for SlotType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_alt = f.alternate();
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Isa => write!(f, "ISA"),
            Self::Mca => write!(f, "MCA"),
            Self::Eisa => write!(f, "EISA"),
            Self::Pci => write!(f, "PCI"),
            Self::PcCard => write!(f, "PC Card (PCMCIA)"),
            Self::VlVesa => write!(f, "VL-VESA"),
            Self::Proprietary => write!(f, "Proprietary"),
            Self::ProcessorCardSlot => write!(f, "Processor Card Slot"),
            Self::ProprietaryMemoryCardSlot => write!(f, "Proprietary Memory Card Slot"),
            Self::IoRiserCardSlot => write!(f, "I/O Riser Card Slot"),
            Self::Nubus => write!(f, "NuBus"),
            Self::Pci66Mhz => write!(f, "PCI – 66MHz Capable"),
            Self::Agp => write!(f, "AGP"),
            Self::Agp2x => write!(f, "AGP 2X"),
            Self::Agp4x => write!(f, "AGP 4X"),
            Self::PciX => write!(f, "PCI-X"),
            Self::Agp8x => write!(f, "AGP 8X"),
            Self::M2Socket1DP => write!(f, "M.2 Socket 1-DP (Mechanical Key A)"),
            Self::M2Socket1SD => write!(f, "M.2 Socket 1-SD (Mechanical Key E)"),
            Self::M2Socket2 => write!(f, "M.2 Socket 2 (Mechanical Key B)"),
            Self::M2Socket3 => write!(f, "M.2 Socket 3 (Mechanical Key M)"),
            Self::MxmType1 => write!(f, "MXM Type I"),
            Self::MxmType2 => write!(f, "MXM Type II"),
            Self::MxmType3 => write!(f, "MXM Type III (standard connector)"),
            Self::MxmType3He => write!(f, "MXM Type III (HE connector)"),
            Self::MxmType4 => write!(f, "MXM Type IV"),
            Self::Mxm3TypeA => write!(f, "MXM 3.0 Type A"),
            Self::Mxm3TypeB => write!(f, "MXM 3.0 Type B"),
            Self::U2PciExpressGen2 => write!(f, "PCI Express Gen 2 SFF-8639 (U.2)"),
            Self::U2PciExpressGen3 => write!(f, "PCI Express Gen 3 SFF-8639 (U.2)"),
            Self::PciExpressMini52pin1 => {
                if is_alt {
                    write!(
                        f,
                        "PCI Express Mini 52-pin (CEM spec. 2.0) with bottom-side keep-outs. \
                        Use Slot Length field value 03h (short length) for \"half-Mini card\" -only \
                        support, 04h (long length) for \"full-Mini card\" or dual support."
                    )
                } else {
                    write!(f, "PCI Express Mini 52-pin with bottom-side keep-outs")
                }
            }
            Self::PciExpressMini52pin2 => {
                if is_alt {
                    write!(
                        f,
                        "PCI Express Mini 52-pin (CEM spec. 2.0) without bottom-side keep-outs. \
                        Use Slot Length field value 03h (short length) for \"half-Mini card\" -only \
                        support, 04h (long length) for \"full-Mini card\" or dual support."
                    )
                } else {
                    write!(f, "PCI Express Mini 52-pin without bottom-side keep-outs")
                }
            }
            Self::PciExpressMini76pin => {
                if is_alt {
                    write!(
                        f,
                        "PCI Express Mini 76-pin (CEM spec. 2.0) Corresponds to Display-Mini card."
                    )
                } else {
                    write!(f, "PCI Express Mini 76-pin")
                }
            }
            Self::U2PciExpressGen4 => write!(f, "PCI Express Gen 4 SFF-8639 (U.2)"),
            Self::U2PciExpressGen5 => write!(f, "PCI Express Gen 5 SFF-8639 (U.2)"),
            Self::OcpNic3Small => write!(f, "OCP NIC 3.0 Small Form Factor (SFF)"),
            Self::OcpNic3Large => write!(f, "OCP NIC 3.0 Large Form Factor (LFF)"),
            Self::OcpNicPriorTo3 => write!(f, "OCP NIC Prior to 3.0"),
            Self::CxlFlexbus1 => write!(f, "CXL Flexbus 1.0 (deprecated)"),
            Self::Pc98C20 => write!(f, "PC-98/C20"),
            Self::Pc98C24 => write!(f, "PC-98/C24"),
            Self::Pc98E => write!(f, "PC-98/E"),
            Self::Pc98LocalBus => write!(f, "PC-98/Local Bus"),
            Self::Pc98Card => write!(f, "PC-98/Card"),
            Self::PciExpress => write!(f, "PCI Express"),
            Self::PciExpressX1 => write!(f, "PCI Express x1"),
            Self::PciExpressX2 => write!(f, "PCI Express x2"),
            Self::PciExpressX4 => write!(f, "PCI Express x4"),
            Self::PciExpressX8 => write!(f, "PCI Express x8"),
            Self::PciExpressX16 => write!(f, "PCI Express x16"),
            Self::PciExpressGen2 => write!(f, "PCI Express Gen 2"),
            Self::PciExpressGen2x1 => write!(f, "PCI Express Gen 2 x1"),
            Self::PciExpressGen2x2 => write!(f, "PCI Express Gen 2 x2"),
            Self::PciExpressGen2x4 => write!(f, "PCI Express Gen 2 x4"),
            Self::PciExpressGen2x8 => write!(f, "PCI Express Gen 2 x8"),
            Self::PciExpressGen2x16 => write!(f, "PCI Express Gen 2 x16"),
            Self::PciExpressGen3 => write!(f, "PCI Express Gen 3"),
            Self::PciExpressGen3x1 => write!(f, "PCI Express Gen 3 x1"),
            Self::PciExpressGen3x2 => write!(f, "PCI Express Gen 3 x2"),
            Self::PciExpressGen3x4 => write!(f, "PCI Express Gen 3 x4"),
            Self::PciExpressGen3x8 => write!(f, "PCI Express Gen 3 x8"),
            Self::PciExpressGen3x16 => write!(f, "PCI Express Gen 3 x16"),
            Self::PciExpressGen4 => write!(f, "PCI Express Gen 4"),
            Self::PciExpressGen4x1 => write!(f, "PCI Express Gen 4 x1"),
            Self::PciExpressGen4x2 => write!(f, "PCI Express Gen 4 x2"),
            Self::PciExpressGen4x4 => write!(f, "PCI Express Gen 4 x4"),
            Self::PciExpressGen4x8 => write!(f, "PCI Express Gen 4 x8"),
            Self::PciExpressGen4x16 => write!(f, "PCI Express Gen 4 x16"),
            Self::PciExpressGen5 => write!(f, "PCI Express Gen 5"),
            Self::PciExpressGen5x1 => write!(f, "PCI Express Gen 5 x1"),
            Self::PciExpressGen5x2 => write!(f, "PCI Express Gen 5 x2"),
            Self::PciExpressGen5x4 => write!(f, "PCI Express Gen 5 x4"),
            Self::PciExpressGen5x8 => write!(f, "PCI Express Gen 5 x8"),
            Self::PciExpressGen5x16 => write!(f, "PCI Express Gen 5 x16"),
            Self::PciExpressGen6 => write!(f, "PCI Express Gen 6 and Beyond"),
            Self::E1FormFactorSlot => {
                if is_alt {
                    write!(
                        f,
                        "Enterprise and Datacenter 1U E1 Form Factor Slot (EDSFF E1.S, E1.L). \
                        See specifications SFF-TA-1006 and SFF-TA-1007 for more details on values \
                        for slot length and pitch."
                    )
                } else {
                    write!(f, "EDSFF E1")
                }
            }
            Self::E3FormFactorSlot => {
                if is_alt {
                    write!(
                        f,
                        "Enterprise and Datacenter 3\" E3 Form Factor Slot (EDSFF E3.S, \
                        E3.L). See specification SFF-TA-1008 for details on values for slot length \
                        and pitch."
                    )
                } else {
                    write!(f, "EDSFF E3")
                }
            }
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for SlotWidth {
    fn from(byte: u8) -> SlotWidth {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Byte,
            0x04 => Self::Word,
            0x05 => Self::Dword,
            0x06 => Self::Qword,
            0x07 => Self::Dqword,
            0x08 => Self::X1,
            0x09 => Self::X2,
            0x0A => Self::X4,
            0x0B => Self::X8,
            0x0C => Self::X12,
            0x0D => Self::X16,
            0x0E => Self::X32,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for SlotWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Byte => write!(f, "8 bit"),
            Self::Word => write!(f, "16 bit"),
            Self::Dword => write!(f, "32 bit"),
            Self::Qword => write!(f, "64 bit"),
            Self::Dqword => write!(f, "128 bit"),
            Self::X1 => write!(f, "1x or x1"),
            Self::X2 => write!(f, "2x or x2"),
            Self::X4 => write!(f, "4x or x4"),
            Self::X8 => write!(f, "8x or x8"),
            Self::X12 => write!(f, "12x or x12"),
            Self::X16 => write!(f, "16x or x16"),
            Self::X32 => write!(f, "32x or x32"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for CurrentUsage {
    fn from(byte: u8) -> CurrentUsage {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::Available,
            0x04 => Self::InUse,
            0x05 => Self::Unavailable,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for CurrentUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_alt = f.alternate();
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Available => write!(f, "Available"),
            Self::InUse => write!(f, "In use"),
            Self::Unavailable => {
                if is_alt {
                    write!(f, "Unavailable (e.g., connected to a processor that is not installed)")
                } else {
                    write!(f, "Unavailable")
                }
            }
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl From<u8> for SlotLength {
    fn from(byte: u8) -> SlotLength {
        match byte {
            0x01 => Self::Other,
            0x02 => Self::Unknown,
            0x03 => Self::ShortLength,
            0x04 => Self::LongLength,
            0x05 => Self::DriveFormFactor2_5,
            0x06 => Self::DriveFormFactor3_5,
            v => Self::Undefined(v),
        }
    }
}
impl fmt::Display for SlotLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::Unknown => write!(f, "Unknown"),
            Self::ShortLength => write!(f, "Short Length"),
            Self::LongLength => write!(f, "Long Length"),
            Self::DriveFormFactor2_5 => write!(f, "2.5\" drive form factor"),
            Self::DriveFormFactor3_5 => write!(f, "3.5\" drive form factor"),
            Self::Undefined(v) => write!(f, "Undefined: {}", v),
        }
    }
}

impl<'a> BitField<'a> for SlotCharacteristics1 {
    type Size = u8;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 8;
        "Characteristics unknown",
        "5.0 V is provided"
            "Provides 5.0 volts",
        "3.3 V is provided"
            "Provides 3.3 volts",
        "Opening is shared"
            "Slot’s opening is shared with another slot (for example, PCI/EISA shared slot)",
        "PC Card-16 is supported"
            "PC Card slot supports PC Card-16",
        "Cardbus is supported"
            "PC Card slot supports CardBus",
        "Zoom Video is supported"
            "PC Card slot supports Zoom Video",
        "Modem ring resume is supported"
            "PC Card slot supports Modem Ring Resume",
    );
}
impl From<u8> for SlotCharacteristics1 {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl<'a> BitField<'a> for SlotCharacteristics2 {
    type Size = u8;
    fn value(&self) -> Self::Size {
        self.0
    }
    layout!(
        length = 8;
        "PME signal is supported"
            "PCI slot supports Power Management Event (PME#) signal",
        "Hot-plug devices are supported"
            "Slot supports hot-plug devices",
        "SMBus signal is supported"
            "PCI slot supports SMBus signal",
        "PCIe slot bifurcation is supported"
            "PCIe slot supports bifurcation. This slot can partition its lanes into two or more \
            PCIe devices plugged into the slot. Note: This field does not indicate complete details \
            on what levels of bifurcation are supported by the slot, but only that the slot \
            supports some level of bifurcation",
        "Async/surprise removal is supported"
            "Slot supports async/surprise removal (i.e., removal without prior notification to the \
            operating system, device driver, or applications)",
        "Flexbus slot, CXL 1.0 capable",
        "Flexbus slot, CXL 2.0 capable",
        "Reserved": 1,
    );
}
impl From<u8> for SlotCharacteristics2 {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl<'a> From<&'a [u8]> for Device {
    fn from(data: &'a [u8]) -> Device {
        let_as_struct!(packed, DevicePacked, data);
        Device {
            segment_group_number: packed.segment_group_number,
            bus_number: packed.bus_number,
            device_and_function_number: packed.dev_and_fun_number.into(),
            data_bus_width: packed.data_bus_width,
        }
    }
}
impl<'a> From<&'a Device> for [u8; 5] {
    fn from(d: &'a Device) -> [u8; 5] {
        let segment = d.segment_group_number.to_ne_bytes();
        [
            segment[0],
            segment[1],
            d.bus_number,
            d.device_and_function_number.into(),
            d.data_bus_width,
        ]
    }
}
impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04x}:{:02x}:{:02x}.{:x} (Width {})",
            self.segment_group_number,
            self.bus_number,
            self.device_and_function_number.0,
            self.device_and_function_number.1,
            self.data_bus_width,
        )
    }
}

impl From<u8> for DeviceAndFunctionNumber {
    fn from(byte: u8) -> Self {
        Self(byte >> 3, byte & 0b0111)
    }
}
impl From<DeviceAndFunctionNumber> for u8 {
    fn from(df: DeviceAndFunctionNumber) -> Self {
        (df.0 << 3) | df.1
    }
}

impl<'a> From<&'a [u8]> for PeerDevices<'a> {
    fn from(data: &'a [u8]) -> PeerDevices {
        Self(data.chunks(5))
    }
}
impl<'a> PartialEq for PeerDevices<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.clone().eq(other.0.clone())
    }
}
impl<'buffer> Eq for PeerDevices<'buffer> {}
impl<'buffer> Hash for PeerDevices<'buffer> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.clone().for_each(|c| c.hash(state));
    }
}
impl<'buffer> Iterator for PeerDevices<'buffer> {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Into::into)
    }
}

impl From<u16> for SlotPitch {
    fn from(word: u16) -> Self {
        Self(word)
    }
}
impl fmt::Display for SlotPitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            write!(f, "Not given or is unknown")
        } else {
            write!(f, "{:.1} mm", (self.0 as f32) / 10.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::prelude::v1::*;
    const PRIMES: &[usize] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61];

    #[test]
    fn slot_type() {
        use super::SlotType;
        let samples = &[
            (0x03, SlotType::Isa, "ISA", "ISA"),
            (
                0x21,
                SlotType::PciExpressMini52pin1,
                "PCI Express Mini 52-pin with bottom-side keep-outs",
                "PCI Express Mini 52-pin (CEM spec. 2.0) with bottom-side keep-outs. Use Slot \
                Length field value 03h (short length) for \"half-Mini card\" -only support, 04h \
                (long length) for \"full-Mini card\" or dual support.",
            ),
            (0xA0, SlotType::Pc98C20, "PC-98/C20", "PC-98/C20"),
            (
                0xC4,
                SlotType::PciExpressGen6,
                "PCI Express Gen 6 and Beyond",
                "PCI Express Gen 6 and Beyond",
            ),
            (0xFE, SlotType::Undefined(254), "Undefined: 254", "Undefined: 254"),
        ];
        let result = samples.iter().map(|v| Into::into(v.0)).collect::<Vec<_>>();
        assert_eq!(
            samples
                .iter()
                .map(|(_, v, s, m)| (v, (*s).into(), (*m).into()))
                .collect::<Vec<_>>(),
            result
                .iter()
                .map(|r| (r, format!("{}", r), format!("{:#}", r)))
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn slot_width() {
        use super::SlotWidth;
        let samples = &[
            (0x01, SlotWidth::Other, "Other"),
            (0x03, SlotWidth::Byte, "8 bit"),
            (0x06, SlotWidth::Qword, "64 bit"),
            (0x07, SlotWidth::Dqword, "128 bit"),
            (0x0E, SlotWidth::X32, "32x or x32"),
            (0xFE, SlotWidth::Undefined(254), "Undefined: 254"),
        ];
        let result = samples.iter().map(|v| Into::into(v.0)).collect::<Vec<_>>();
        assert_eq!(
            samples.iter().map(|(_, v, s)| (v, (*s).into())).collect::<Vec<_>>(),
            result.iter().map(|r| (r, format!("{}", r))).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn current_usage() {
        use super::CurrentUsage;
        let samples = &[
            (0x01, CurrentUsage::Other, "Other"),
            (0x02, CurrentUsage::Unknown, "Unknown"),
            (0x04, CurrentUsage::InUse, "In use"),
            (
                0x05,
                CurrentUsage::Unavailable,
                "Unavailable (e.g., connected to a processor that is not installed)",
            ),
            (0xFE, CurrentUsage::Undefined(254), "Undefined: 254"),
        ];
        let result = samples.iter().map(|v| Into::into(v.0)).collect::<Vec<_>>();
        assert_eq!(
            samples.iter().map(|(_, v, s)| (v, (*s).into())).collect::<Vec<_>>(),
            result.iter().map(|r| (r, format!("{:#}", r))).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn slot_length() {
        use super::SlotLength;
        let samples = &[
            (0x01, SlotLength::Other, "Other"),
            (0x02, SlotLength::Unknown, "Unknown"),
            (0x03, SlotLength::ShortLength, "Short Length"),
            (0x05, SlotLength::DriveFormFactor2_5, "2.5\" drive form factor"),
            (0xFE, SlotLength::Undefined(254), "Undefined: 254"),
        ];
        let result = samples.iter().map(|v| Into::into(v.0)).collect::<Vec<_>>();
        assert_eq!(
            samples.iter().map(|(_, v, s)| (v, (*s).into())).collect::<Vec<_>>(),
            result.iter().map(|r| (r, format!("{:}", r))).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn slot_caharacteristics_1() {
        use super::SlotCharacteristics1;
        use crate::bitfield::{BitField, Position};
        let sample = PRIMES.iter().cloned().take_while(|&x| x < 8).collect::<Vec<_>>();
        let byte = sample.iter().map(|&p| Position(p)).collect();
        let result = SlotCharacteristics1(byte)
            .iter()
            .filter_map(|f| if f.is_set { Some(*f.position) } else { None })
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Positions");

        let dflt_sample = vec!["5.0 V is provided", "Modem ring resume is supported"];
        let alt_sample = vec!["Provides 5.0 volts", "PC Card slot supports Modem Ring Resume"];
        let byte = 0b1000_0010;
        let iter = SlotCharacteristics1(byte).significants();
        let dflt_result = iter.map(|f| format!("{}", f)).collect::<Vec<_>>();
        assert_eq!(
            dflt_sample, dflt_result,
            "Significant values, default formatting ({:08b})",
            byte
        );
        let alt_result = iter.map(|f| format!("{:#}", f)).collect::<Vec<_>>();
        assert_eq!(
            alt_sample, alt_result,
            "Significant values, alternative formatting ({:08b})",
            byte
        );

        let result = SlotCharacteristics1(0).reserved().count();
        assert_eq!(0, result, "Reserved fields");
    }

    #[test]
    fn slot_caharacteristics_2() {
        use super::SlotCharacteristics2;
        use crate::bitfield::{BitField, Position};
        let sample = PRIMES.iter().cloned().take_while(|&x| x < 8).collect::<Vec<_>>();
        let byte = sample.iter().map(|&p| Position(p)).collect();
        let result = SlotCharacteristics2(byte)
            .iter()
            .filter_map(|f| if f.is_set { Some(*f.position) } else { None })
            .collect::<Vec<_>>();
        assert_eq!(sample, result, "Positions");

        let dflt_sample = vec!["PME signal is supported", "Async/surprise removal is supported"];
        let alt_sample = vec!["PCI slot supports Power Management Event (PME#) signal","Slot supports async/surprise removal (i.e., removal without prior notification to the operating system, device driver, or applications)"];
        let byte = 0b0001_0001;
        let iter = SlotCharacteristics2(byte).significants();
        let dflt_result = iter.map(|f| format!("{}", f)).collect::<Vec<_>>();
        assert_eq!(
            dflt_sample, dflt_result,
            "Significant values, default formatting ({:08b})",
            byte
        );
        let alt_result = iter.map(|f| format!("{:#}", f)).collect::<Vec<_>>();
        assert_eq!(
            alt_sample, alt_result,
            "Significant values, alternative formatting ({:08b})",
            byte
        );

        let result = SlotCharacteristics2(0).reserved().count();
        assert_eq!(1, result, "Reserved fields");
    }

    #[test]
    fn device() {
        use super::Device;
        let sample_data = &[
            0x09, 0x11, 0x03, 0x09, 0x01, 0xB1, 0x0D, 0x04, 0x04, 0x04, 0x00, 0x04, 0x01, 0xE9, 0x05, 0xB5, 0xDF, 0x10,
        ];
        let result: Device = sample_data[0x0D..=0x11].into();
        assert_eq!("05e9:b5:1b.7 (Width 16)", format!("{}", result), "Display trait");
        let as_array: [u8; 5] = (&result).into();
        assert_eq!([0xE9, 0x05, 0xB5, 0xDF, 0x10], as_array, "Display into [u8; 5]");
    }

    #[test]
    fn peer_devices() {
        use super::PeerDevices;
        let sample_data = &[
            0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xE3, 0x01, 0x00, 0x00, 0x00, 0xE4, 0x04,
        ];
        let result = PeerDevices(sample_data.chunks(5));
        let display_sample: Vec<String> = [
            "0000:00:01.0 (Width 0)",
            "0000:00:1c.3 (Width 1)",
            "0000:00:1c.4 (Width 4)",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();
        assert_eq!(display_sample, result.map(|v| format!("{}", v)).collect::<Vec<_>>());
    }

    #[test]
    fn system_slots() {
        use super::*;
        use crate::{InfoType, RawStructure, SmbiosVersion};
        for major in u8::MIN..=u8::MAX {
            for minor in u8::MIN..=u8::MAX {
                let structure = RawStructure {
                    version: SmbiosVersion { major, minor },
                    info: InfoType::SystemSlots,
                    length: 0,
                    handle: 666,
                    data: &[],
                    strings: &[],
                };
                let result = SystemSlots::try_from(structure);
                match ((major, minor), result) {
                    (v, Err(e)) if ((2, 0)..(2, 1)).contains(&v) => {
                        assert_eq!(
                            "Formatted section length of structure SystemSlots with handle 666 \
                            should be 12 bytes",
                            format!("{}", e)
                        );
                    }
                    (v, Err(e)) if ((2, 1)..(2, 6)).contains(&v) => {
                        assert_eq!(
                            "Formatted section length of structure SystemSlots with handle 666 \
                            should be 13 bytes",
                            format!("{}", e)
                        );
                    }
                    (v, Err(e)) if ((2, 6)..(3, 2)).contains(&v) => {
                        assert_eq!(
                            "Formatted section length of structure SystemSlots with handle 666 \
                            should be 17 bytes",
                            format!("{}", e)
                        );
                    }
                    (v, Err(e)) if ((3, 2)..).contains(&v) => {
                        assert_eq!(
                            "Formatted section length of structure SystemSlots with handle 666 \
                            should be minimum of 17 bytes",
                            format!("{}", e)
                        );
                    }
                    (_, Err(e)) => {
                        assert_eq!("could not convert slice to array", format!("{}", e));
                    }
                    (_, Ok(ss)) => {
                        assert_eq!(666, ss.handle);
                    }
                }
            }
        }
        let peer_devices = [
            Device {
                segment_group_number: 1,
                bus_number: 1,
                device_and_function_number: DeviceAndFunctionNumber(1, 1),
                data_bus_width: 1,
            },
            Device {
                segment_group_number: 2,
                bus_number: 2,
                device_and_function_number: DeviceAndFunctionNumber(2, 2),
                data_bus_width: 2,
            },
            Device {
                segment_group_number: 3,
                bus_number: 3,
                device_and_function_number: DeviceAndFunctionNumber(3, 3),
                data_bus_width: 3,
            },
        ]
        .iter()
        .fold(Vec::new(), |mut acc, v| {
            let arr: [u8; 5] = v.into();
            acc.extend_from_slice(&arr);
            acc
        });
        let sample = SystemSlots {
            handle: 0x0023,
            slot_designation: "SSD1",
            slot_type: SlotType::PciExpress,
            slot_data_bus_width: SlotWidth::X4,
            current_usage: CurrentUsage::InUse,
            slot_length: SlotLength::ShortLength,
            slot_id: 2,
            slot_characteristics_1: SlotCharacteristics1(0b0000_1100),
            slot_characteristics_2: Some(SlotCharacteristics2(0b0000_0001)),
            segment_group_number: Some(0),
            bus_number: Some(0),
            device_and_function_number: Some(DeviceAndFunctionNumber(0x1C, 4)),
            data_bus_width: Some(0),
            peer_devices: Some(peer_devices.as_slice().into()),
            slot_information: Some(0x06),
            slot_physical_width: Some(SlotWidth::X16),
            slot_pitch: Some(SlotPitch(0x04E2)),
        };
        let structure = RawStructure {
            version: (3, 4).into(),
            info: InfoType::SystemSlots,
            length: 0,
            handle: 0x0023,
            // Remove 4 bytes from `dmidecode -H 8 -u` 'Header and Data'
            data: &[
                0x01, // Slot designation: first string
                0xA5, // Slot type: PCI Express
                0x0A, // Slot Data Bus Width: 4x or x4
                0x04, // Current Usage: In Use
                0x03, // Slot Length: Short
                0x02,
                0x00, // Slot ID: 2
                0x0C, // Slot Characteristics 1: 3.3 V is provided, Opening is shared
                0x01, // Slot Characteristics 2: PME signal is supported
                0x00,
                0x00, // Segment Group Number: 0
                0x00, // Bus Number: 0
                0xE4, // Device Number: 1c, Function number: 4
                0x00, // Data Bus Width: 0
                0x03, // Peer grouping count: 3
                // Peer groups (Segment/Bus/Device/Function/Width)
                0x01,
                0x00,
                0x01,
                0x01 << 3 | 0x01,
                0x01, // Peer group 1: ( 1 / 1 / 1 / 1 / 1)
                0x02,
                0x00,
                0x02,
                0x02 << 3 | 0x02,
                0x02, // Peer group 2: ( 2 / 2 / 2 / 2 / 2)
                0x03,
                0x00,
                0x03,
                0x03 << 3 | 0x03,
                0x03, // Peer group 3: ( 3 / 3 / 3 / 3 / 3)
                0x00, // Blank field, may be mistake in SMBIOS specification
                0x06, // Slot information: Gen6
                0x0D, // Slot physical width: x16
                0xE2,
                0x04, // Slot pitch: 12.5 mm
            ],
            strings: &[
                // SSD1
                0x53, 0x53, 0x44, 0x31, 0x00,
            ],
        };
        let result = SystemSlots::try_from(structure).unwrap();
        assert_eq!(sample, result, "Sample:\n{:X?}\nResult:\n{:X?}", sample, result);
    }
    #[test]
    fn dmi_bin() {
        use super::*;
        use crate::{EntryPoint, Structure};
        const DMIDECODE_BIN: &[u8] = include_bytes!("../../tests/data/dmi.0.bin");
        let entry_point = EntryPoint::search(DMIDECODE_BIN).unwrap();
        let slots = entry_point
            .structures(&DMIDECODE_BIN[(entry_point.smbios_address() as usize)..])
            .filter_map(|s| {
                if let Err(ref s) = s {
                    println!("{}", s);
                }
                s.ok().filter(|s| matches!(s, Structure::SystemSlots(_)))
            })
            .collect::<Vec<_>>();
        assert_eq!(4, slots.len(), "Slots count: {}. Should be 4", slots.len());

        let slot1_sample = SystemSlots {
            handle: 0x0900,
            slot_designation: "PCIe Slot 1",
            slot_type: SlotType::PciExpressGen3x16,
            slot_data_bus_width: SlotWidth::X8,
            current_usage: CurrentUsage::Available,
            slot_length: SlotLength::LongLength,
            slot_id: 1,
            slot_characteristics_1: SlotCharacteristics1(0b0000_0100),
            slot_characteristics_2: Some(SlotCharacteristics2(0b0000_0001)),
            segment_group_number: None,
            bus_number: None,
            device_and_function_number: None,
            data_bus_width: None,
            peer_devices: None,
            slot_information: None,
            slot_physical_width: None,
            slot_pitch: None,
        };
        let slot1_result = slots
            .iter()
            .find_map(|s| match s {
                Structure::SystemSlots(ss) if ss.handle == 0x0900 => Some(ss),
                _ => None,
            })
            .unwrap();
        assert_eq!(&slot1_sample, slot1_result, "Entire SystemSlots struct: Slot 1");
        let slot4_sample = SystemSlots {
            handle: 0x0903,
            slot_designation: "PCIe Slot 4",
            slot_type: SlotType::PciExpressGen3,
            slot_data_bus_width: SlotWidth::X16,
            current_usage: CurrentUsage::InUse,
            slot_length: SlotLength::LongLength,
            slot_id: 4,
            slot_characteristics_1: SlotCharacteristics1(0b0000_0100),
            slot_characteristics_2: Some(SlotCharacteristics2(0b0000_0001)),
            segment_group_number: Some(0),
            bus_number: Some(0xAF),
            device_and_function_number: Some(0x00.into()),
            data_bus_width: None,
            peer_devices: None,
            slot_information: None,
            slot_physical_width: None,
            slot_pitch: None,
        };
        let slot4_result = slots
            .iter()
            .find_map(|s| match s {
                Structure::SystemSlots(ss) if ss.handle == 0x0903 => Some(ss),
                _ => None,
            })
            .unwrap();
        assert_eq!(&slot4_sample, slot4_result, "Entire SystemSlots struct: Slot 4");
    }
}
