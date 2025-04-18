//! Processor Information (Type 4)
//!
//! The information in this structure defines the attributes of a single processor; a separate
//! structure instance is provided for each system processor socket/slot. For example, a system
//! with an IntelDX2™ processor would have a single structure instance while a system with an
//! IntelSX2™ processor would have a structure to describe the main CPU and a second structure to
//! describe the 80487 co1021 processor.

#[cfg(feature = "std")]
extern crate std;

use core::fmt;

use crate::{MalformedStructureError, RawStructure};

/// The processor types defined in the SMBIOS specification.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ProcessorType {
    Other,
    Unknown,
    CentralProcessor,
    MathProcessor,
    DspProcessor,
    VideoProcessor,
    Undefined(u8),
}

impl From<u8> for ProcessorType {
    fn from(_type: u8) -> ProcessorType {
        match _type {
            1 => ProcessorType::Other,
            2 => ProcessorType::Unknown,
            3 => ProcessorType::CentralProcessor,
            4 => ProcessorType::MathProcessor,
            5 => ProcessorType::DspProcessor,
            6 => ProcessorType::VideoProcessor,
            t => ProcessorType::Undefined(t),
        }
    }
}

bitflags! {
    /// The processor status flags defined in the SMBIOS specification.
    pub struct ProcessorStatus: u8 {
        const CPU_SOCKET_POPULATED = 0b0100_0000;
        const CPU_ENABLED = 0b0000_0001;
        const CPU_DISABLED_BY_USER = 0b0000_0010;
        const CPU_DISABLED_BY_BIOS = 0b0000_0011;
        const CPU_IDLE = 0b0000_0100;
        const CPU_OTHER = 0b000_0111;
    }
}

bitflags! {
    /// The processor characteristic flags defined in the SMBIOS specification.
    pub struct ProcessorCharacteristics: u16 {
        const RESERVED = 0b0000_0001;
        const UNKNOWN = 0b0000_0010;
        const CAPABLE_64BIT = 0b0000_0100;
        const MULTICORE = 0b0000_1000;
        const HARDWARE_THREAD = 0b0001_0000;
        const EXECUTE_PROTECTION = 0b0010_0000;
        const ENHANCED_VIRTUALIZATION = 0b0100_0000;
        const POWER_PERFORMANCE_CONTROL = 0b1000_0000;
        const ARM64_SOC_ID = 0b0000_0010_0000_0000;
    }
}

/// The `Processor` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Processor<'buffer> {
    pub handle: u16,
    /// String number for Reference Designation EXAMPLE: ‘J202’,0
    pub socket_designation: &'buffer str,
    /// Processor Type field
    pub processor_type: ProcessorType,
    /// Processor Family field
    pub processor_family: ProcessorFamily,
    /// String number of Processor Manufacturer
    pub processor_manufacturer: &'buffer str,
    /// Raw processor identification data
    pub processor_id: u64,
    /// String number describing the Processor
    pub processor_version: &'buffer str,
    /// Voltage
    pub voltage: Voltage,
    /// External Clock Frequency, in MHz. If the value is unknown, the field is set to 0.
    pub external_clock: u16,
    /// Maximum processor speed (in MHz) supported by the system for this processor socket
    pub max_speed: u16,
    /// This field identifies the processor's speed at system boot; the processor may support more
    /// than one speed.
    pub current_speed: u16,
    /// Status
    pub status: ProcessorStatus,
    /// Processor Upgrade field
    pub processor_upgrade: ProcessorUpgrade,
    /// Handle of a Cache Information structure that defines the attributes of the primary
    /// (Level 1) cache for this processor
    pub l1_cache_handle: Option<u16>,
    /// Handle of a Cache Information structure that defines the attributes of the secondary
    /// (Level 2) cache for this processor
    pub l2_cache_handle: Option<u16>,
    /// Handle of a Cache Information structure that defines the attributes of the tertiary
    /// (Level 3) cache for this processor
    pub l3_cache_handle: Option<u16>,
    /// String number for the serial number of this processor
    pub serial_number: Option<&'buffer str>,
    /// String number for the asset tag of this processor
    pub asset_tag: Option<&'buffer str>,
    /// String number for the part number of this processor
    pub part_number: Option<&'buffer str>,
    /// Number of cores per processor socket
    pub core_count: Option<u16>,
    /// Number of enabled cores per processor socket
    pub core_enabled: Option<u16>,
    /// Number of threads per processor socket
    pub thread_count: Option<u16>,
    /// Defines which functions the processor supports
    pub processor_characteristics: Option<ProcessorCharacteristics>,
}

/// For processor family enumerations from 0 to FDh, *Processor Family* is identical to *Processor Family 2*.
/// For processor family enumerations from 100h to FFFDh, *Processor Family* has a value of FEh
/// and *Processor Family 2* has the enumerated value.
/// The following values are reserved:
/// • FFh Not used. FFh is the un-initialized value of Flash memory.
/// • FFFFh Not used. FFFFh is the un-initialized value of Flash memory.
/// • FFFEh For special use in the future, such as FEh as the extension indicator.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ProcessorFamily {
    Other,
    Unknown,
    Intel8086,
    Intel80286,
    Intel386Processor,
    Intel486Processor,
    Intel8087,
    Intel80287,
    Intel80387,
    Intel80487,
    IntelPentiumProcessor,
    PentiumProProcessor,
    PentiumIIProcessor,
    PentiumProcessorWithMMXTechnology,
    IntelCeleronProcessor,
    PentiumIIXeonProcessor,
    PentiumIIIProcessor,
    M1Family,
    M2Family,
    IntelCeleronMProcessor,
    IntelPentium4HTProcessor,
    /// Note that the meaning associated with this value is different from the meaning defined in
    /// CIM_Processor.Family for the same value.
    AMDDuronProcessorFamily,
    /// Note that the meaning associated with this value is different from the meaning defined in
    /// CIM_Processor.Family for the same value.
    K5Family,
    /// Note that the meaning associated with this value is different from the meaning defined in
    /// CIM_Processor.Family for the same value.
    K6Family,
    /// Note that the meaning associated with this value is different from the meaning defined in
    /// CIM_Processor.Family for the same value.
    K62,
    /// Note that the meaning associated with this value is different from the meaning defined in
    /// CIM_Processor.Family for the same value.
    K63,
    /// Note that the meaning associated with this value is different from the meaning defined in
    /// CIM_Processor.Family for the same value.
    AMDAthlonProcessorFamily,
    AMD29000Family,
    K62Plus,
    PowerPCFamily,
    PowerPC601,
    PowerPC603,
    PowerPC603Plus,
    PowerPC604,
    PowerPC620,
    PowerPCX704,
    PowerPC750,
    IntelCoreDuoProcessor,
    IntelCoreDuoMobileProcessor,
    IntelCoreSoloMobileProcessor,
    IntelAtomProcessor,
    IntelCoreMProcessor,
    IntelCoreM3Processor,
    IntelCoreM5Processor,
    IntelCoreM7Processor,
    /// Some version 2.0 specification implementations used Processor Family type value 30h to
    /// represent a Pentium® Pro processor.
    AlphaFamily,
    Alpha21064,
    Alpha21066,
    Alpha21164,
    Alpha21164PC,
    Alpha21164a,
    Alpha21264,
    Alpha21364,
    AMDTurionIIUltraDualCoreMobileMProcessorFamily,
    AMDTurionIIDualCoreMobileMProcessorFamily,
    AMDAthlonIIDualCoreMProcessorFamily,
    AMDOpteron6100SeriesProcessor,
    AMDOpteron4100SeriesProcessor,
    AMDOpteron6200SeriesProcessor,
    AMDOpteron4200SeriesProcessor,
    AMDFXSeriesProcessor,
    MIPSFamily,
    MIPSR4000,
    MIPSR4200,
    MIPSR4400,
    MIPSR4600,
    MIPSR10000,
    AMDCSeriesProcessor,
    AMDESeriesProcessor,
    AMDASeriesProcessor,
    AMDGSeriesProcessor,
    AMDZSeriesProcessor,
    AMDRSeriesProcessor,
    AMDOpteron4300SeriesProcessor,
    AMDOpteron6300SeriesProcessor,
    AMDOpteron3300SeriesProcessor,
    AMDFireProSeriesProcessor,
    SPARCFamily,
    SuperSPARC,
    MicroSPARCII,
    MicroSPARCIIep,
    UltraSPARC,
    UltraSPARCII,
    UltraSPARCIii,
    UltraSPARCIII,
    UltraSPARCIIIi,
    Motorola68040Family,
    Motorola68xxx,
    Motorola68000,
    Motorola68010,
    Motorola68020,
    Motorola68030,
    AMDAthlonX4QuadCoreProcessorFamily,
    AMDOpteronX1000SeriesProcessor,
    AMDOpteronX2000SeriesAPU,
    AMDOpteronASeriesProcessor,
    AMDOpteronX3000SeriesAPU,
    AMDZenProcessorFamily,
    HobbitFamily,
    CrusoeTM5000Family,
    CrusoeTM3000Family,
    EfficeonTM8000Family,
    Weitek,
    AvailableForAssignment,
    ItaniumProcessor,
    AMDAthlon64ProcessorFamily,
    AMDOpteronProcessorFamily,
    AMDSempronProcessorFamily,
    AMDTurion64MobileTechnology,
    DualCoreAMDOpteronProcessorFamily,
    AMDAthlon64X2DualCoreProcessorFamily,
    AMDTurion64X2MobileTechnology,
    QuadCoreAMDOpteronProcessorFamily,
    ThirdGenerationAMDOpteronProcessorFamily,
    AMDPhenomFXQuadCoreProcessorFamily,
    AMDPhenomX4QuadCoreProcessorFamily,
    AMDPhenomX2DualCoreProcessorFamily,
    AMDAthlonX2DualCoreProcessorFamily,
    PARISCFamily,
    PARISC8500,
    PARISC8000,
    PARISC7300LC,
    PARISC7200,
    PARISC7100LC,
    PARISC7100,
    V30Family,
    QuadCoreIntelXeonProcessor3200Series,
    DualCoreIntelXeonProcessor3000Series,
    QuadCoreIntelXeonProcessor5300Series,
    DualCoreIntelXeonProcessor5100Series,
    DualCoreIntelXeonProcessor5000Series,
    DualCoreIntelXeonProcessorLV,
    DualCoreIntelXeonProcessorULV,
    DualCoreIntelXeonProcessor7100Series,
    QuadCoreIntelXeonProcessor5400Series,
    QuadCoreIntelXeonProcessor,
    DualCoreIntelXeonProcessor5200Series,
    DualCoreIntelXeonProcessor7200Series,
    QuadCoreIntelXeonProcessor7300Series,
    QuadCoreIntelXeonProcessor7400Series,
    MultiCoreIntelXeonProcessor7400Series,
    PentiumIIIXeonProcessor,
    PentiumIIIProcessorWithIntelSpeedStepTechnology,
    Pentium4Processor,
    IntelXeonProcessor,
    AS400Family,
    IntelXeonProcessorMP,
    AMDAthlonXPProcessorFamily,
    AMDAthlonMPProcessorFamily,
    IntelItanium2Processor,
    IntelPentiumMProcessor,
    IntelCeleronDProcessor,
    IntelPentiumDProcessor,
    IntelPentiumProcessorExtremeEdition,
    IntelCoreSoloProcessor,
    /// Version 2.5 of this specification listed this value as “available for assignment”.
    /// CIM_Processor.mof files assigned this value to AMD K7 processors in the
    /// CIM_Processor.Family property, and an SMBIOS change request assigned it to Intel Core 2
    /// processors. Some implementations of the SMBIOS version 2.5 specification are known to use
    /// BEh to indicate Intel Core 2 processors. Some implementations of SMBIOS and some
    /// implementations of CIM-based software may also have used BEh to indicate AMD K7 processors.
    Ambiguous,
    IntelCore2DuoProcessor,
    IntelCore2SoloProcessor,
    IntelCore2ExtremeProcessor,
    IntelCore2QuadProcessor,
    IntelCore2ExtremeMobileProcessor,
    IntelCore2DuoMobileProcessor,
    IntelCore2SoloMobileProcessor,
    IntelCoreI7Processor,
    DualCoreIntelCeleronProcessor,
    IBM390Family,
    G4,
    G5,
    ESA390G6,
    ZArchitectureBase,
    IntelCoreI5Processor,
    IntelCoreI3Processor,
    IntelCoreI9Processor,
    VIAC7MProcessorFamily,
    VIAC7DProcessorFamily,
    VIAC7ProcessorFamily,
    VIAEdenProcessorFamily,
    MultiCoreIntelXeonProcessor,
    DualCoreIntelXeonProcessor3xxxSeries,
    QuadCoreIntelXeonProcessor3xxxSeries,
    VIANanoProcessorFamily,
    DualCoreIntelXeonProcessor5xxxSeries,
    QuadCoreIntelXeonProcessor5xxxSeries,
    DualCoreIntelXeonProcessor7xxxSeries,
    QuadCoreIntelXeonProcessor7xxxSeries,
    MultiCoreIntelXeonProcessor7xxxSeries,
    MultiCoreIntelXeonProcessor3400Series,
    AMDOpteron3000SeriesProcessor,
    AMDSempronIIProcessor,
    EmbeddedAMDOpteronQuadCoreProcessorFamily,
    AMDPhenomTripleCoreProcessorFamily,
    AMDTurionUltraDualCoreMobileProcessorFamily,
    AMDTurionDualCoreMobileProcessorFamily,
    AMDAthlonDualCoreProcessorFamily,
    AMDSempronSIProcessorFamily,
    AMDPhenomIIProcessorFamily,
    AMDAthlonIIProcessorFamily,
    SixCoreAMDOpteronProcessorFamily,
    AMDSempronMProcessorFamily,
    I860,
    I960,
    ARMv7,
    ARMv8,
    ARMv9,
    SH3,
    SH4,
    ARM,
    StrongARM,
    Cyrix6x86,
    MediaGX,
    MII,
    WinChip,
    DSP,
    VideoProcessor,
    RISCVRV32,
    RISCVRV64,
    RISCVRV128,
    Available(u16),
    NotUsed(u16),
    ForFutureUse,
    ProcessorFamily2,
    OutOfSpec,
}

/// Two forms of information can be specified by the SMBIOS in this field, dependent on the value
/// present in bit 7 (the most-significant bit). If bit 7 is 0 (legacy mode), the remaining bits of
/// the field represent the specific voltages that the processor socket can accept. If bit 7 is set
/// to 1, the remaining seven bits of the field are set to contain the processor’s current voltage
/// times 10.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Voltage {
    Legacy(VoltageLegacy),
    Current(u8),
    Undefined(u8),
}

bitflags! {
    /// Voltage Capability. A set bit indicates that the voltage is supported
    pub struct VoltageLegacy: u8 {
        const VOLTAGE_CAPABILITY_5V0  = 0b0000_0001;
        const VOLTAGE_CAPABILITY_3V3  = 0b0000_0010;
        const VOLTAGE_CAPABILITY_2V9  = 0b0000_0100;
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ProcessorUpgrade {
    Other,
    Unknown,
    DaughterBoard,
    ZIFSocket,
    ReplaceablePiggyBack,
    None,
    LIFSocket,
    Slot1,
    Slot2,
    Socket370,
    SlotA,
    SlotM,
    Socket423,
    SocketA,
    Socket478,
    Socket754,
    Socket940,
    Socket939,
    SocketmPGA604,
    SocketLGA771,
    SocketLGA775,
    SocketS1,
    SocketAM2,
    SocketF,
    SocketLGA1366,
    SocketG34,
    SocketAM3,
    SocketC32,
    SocketLGA1156,
    SocketLGA1567,
    SocketPGA988A,
    SocketBGA1288,
    SocketrPGA988B,
    SocketBGA1023,
    SocketBGA1224,
    SocketLGA1155,
    SocketLGA1356,
    SocketLGA2011,
    SocketFS1,
    SocketFS2,
    SocketFM1,
    SocketFM2,
    SocketLGA2011Three,
    SocketLGA1356Three,
    SocketLGA1150,
    SocketBGA1168,
    SocketBGA1234,
    SocketBGA1364,
    SocketAM4,
    SocketLGA1151,
    SocketBGA1356,
    SocketBGA1440,
    SocketBGA1515,
    SocketLGA3647,
    SocketSP3,
    SocketSP3r2,
    SocketLGA2066,
    SocketBGA1392,
    SocketBGA1510,
    SocketBGA1528,
    SocketLGA4189,
    SocketLGA1200,
    Undefined(u8),
}

impl<'buffer> Processor<'buffer> {
    pub(crate) fn try_from(structure: RawStructure<'buffer>) -> Result<Processor<'buffer>, MalformedStructureError> {
        #[repr(C)]
        #[repr(packed)]
        struct ProcessorPacked_2_0 {
            socket_designation: u8,
            processor_type: u8,
            processor_family: u8,
            processor_manufacturer: u8,
            processor_id: u64,
            processor_version: u8,
            voltage: u8,
            external_clock: u16,
            max_speed: u16,
            current_speed: u16,
            status: u8,
            processor_upgrade: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct ProcessorPacked_2_1 {
            socket_designation: u8,
            processor_type: u8,
            processor_family: u8,
            processor_manufacturer: u8,
            processor_id: u64,
            processor_version: u8,
            voltage: u8,
            external_clock: u16,
            max_speed: u16,
            current_speed: u16,
            status: u8,
            processor_upgrade: u8,
            l1_cache_handle: u16,
            l2_cache_handle: u16,
            l3_cache_handle: u16,
        }

        #[repr(C)]
        #[repr(packed)]
        struct ProcessorPacked_2_3 {
            socket_designation: u8,
            processor_type: u8,
            processor_family: u8,
            processor_manufacturer: u8,
            processor_id: u64,
            processor_version: u8,
            voltage: u8,
            external_clock: u16,
            max_speed: u16,
            current_speed: u16,
            status: u8,
            processor_upgrade: u8,
            l1_cache_handle: u16,
            l2_cache_handle: u16,
            l3_cache_handle: u16,
            serial_number: u8,
            asset_tag: u8,
            part_number: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct ProcessorPacked_2_5 {
            socket_designation: u8,
            processor_type: u8,
            processor_family: u8,
            processor_manufacturer: u8,
            processor_id: u64,
            processor_version: u8,
            voltage: u8,
            external_clock: u16,
            max_speed: u16,
            current_speed: u16,
            status: u8,
            processor_upgrade: u8,
            l1_cache_handle: u16,
            l2_cache_handle: u16,
            l3_cache_handle: u16,
            serial_number: u8,
            asset_tag: u8,
            part_number: u8,
            core_count: u8,
            core_enabled: u8,
            thread_count: u8,
            processor_characteristics: u16,
        }

        #[repr(C)]
        #[repr(packed)]
        struct ProcessorPacked_2_6 {
            socket_designation: u8,
            processor_type: u8,
            processor_family: u8,
            processor_manufacturer: u8,
            processor_id: u64,
            processor_version: u8,
            voltage: u8,
            external_clock: u16,
            max_speed: u16,
            current_speed: u16,
            status: u8,
            processor_upgrade: u8,
            l1_cache_handle: u16,
            l2_cache_handle: u16,
            l3_cache_handle: u16,
            serial_number: u8,
            asset_tag: u8,
            part_number: u8,
            core_count: u8,
            core_enabled: u8,
            thread_count: u8,
            processor_characteristics: u16,
            processor_family_2: u16,
        }

        #[repr(C)]
        #[repr(packed)]
        struct ProcessorPacked_3_0 {
            socket_designation: u8,
            processor_type: u8,
            processor_family: u8,
            processor_manufacturer: u8,
            processor_id: u64,
            processor_version: u8,
            voltage: u8,
            external_clock: u16,
            max_speed: u16,
            current_speed: u16,
            status: u8,
            processor_upgrade: u8,
            l1_cache_handle: u16,
            l2_cache_handle: u16,
            l3_cache_handle: u16,
            serial_number: u8,
            asset_tag: u8,
            part_number: u8,
            core_count: u8,
            core_enabled: u8,
            thread_count: u8,
            processor_characteristics: u16,
            processor_family_2: u16,
            core_count_2: u16,
            core_enabled_2: u16,
            thread_count_2: u16,
        }

        if structure.version < (2, 1).into() {
            let_as_struct!(packed, ProcessorPacked_2_0, structure.data);

            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family.into(),
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage.into(),
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: None,
                l2_cache_handle: None,
                l3_cache_handle: None,
                serial_number: None,
                asset_tag: None,
                part_number: None,
                core_count: None,
                core_enabled: None,
                thread_count: None,
                processor_characteristics: None,
            })
        } else if structure.version < (2, 3).into() {
            let_as_struct!(packed, ProcessorPacked_2_1, structure.data);

            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family.into(),
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage.into(),
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: None,
                asset_tag: None,
                part_number: None,
                core_count: None,
                core_enabled: None,
                thread_count: None,
                processor_characteristics: None,
            })
        } else if structure.version < (2, 5).into() {
            let_as_struct!(packed, ProcessorPacked_2_3, structure.data);

            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family.into(),
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage.into(),
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(structure.find_string(packed.serial_number)?),
                asset_tag: Some(structure.find_string(packed.asset_tag)?),
                part_number: Some(structure.find_string(packed.part_number)?),
                core_count: None,
                core_enabled: None,
                thread_count: None,
                processor_characteristics: None,
            })
        } else if structure.version < (2, 6).into() {
            let_as_struct!(packed, ProcessorPacked_2_5, structure.data);

            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family.into(),
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage.into(),
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(structure.find_string(packed.serial_number)?),
                asset_tag: Some(structure.find_string(packed.asset_tag)?),
                part_number: Some(structure.find_string(packed.part_number)?),
                core_count: Some(packed.core_count as u16),
                core_enabled: Some(packed.core_enabled as u16),
                thread_count: Some(packed.thread_count as u16),
                processor_characteristics: None,
            })
        } else if structure.version < (3, 0).into() {
            let_as_struct!(packed, ProcessorPacked_2_6, structure.data);
            // smbios spec specifies 0xFE as an indicator to obtain processor
            // family from the Processor Family 2 field.
            let processor_family = match packed.processor_family.into() {
                ProcessorFamily::ProcessorFamily2 => packed.processor_family_2.into(),
                family => family,
            };
            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage.into(),
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(structure.find_string(packed.serial_number)?),
                asset_tag: Some(structure.find_string(packed.asset_tag)?),
                part_number: Some(structure.find_string(packed.part_number)?),
                core_count: Some(packed.core_count as u16),
                core_enabled: Some(packed.core_enabled as u16),
                thread_count: Some(packed.thread_count as u16),
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(
                    packed.processor_characteristics,
                )),
            })
        } else {
            let_as_struct!(packed, ProcessorPacked_3_0, structure.data);
            // smbios spec specifies 0xFE as an indicator to obtain processor
            // family from the Processor Family 2 field.
            let processor_family = match packed.processor_family.into() {
                ProcessorFamily::ProcessorFamily2 => packed.processor_family_2.into(),
                family => family,
            };

            // The Core Count 2 field supports core counts > 255. For core counts of 256 or greater, the Core Count
            // field is set to FFh and the Core Count 2 field is set to the number of cores. For core counts of 255 or
            // fewer, if Core Count 2 is present it shall be set the same value as Core Count
            //
            // The rule is same for Core Enabled and Thread Count as well.
            let core_count = if packed.core_count == 0xFF {
                Some(packed.core_count_2)
            } else {
                Some(packed.core_count as u16)
            };
            let core_enabled = if packed.core_enabled == 0xFF {
                Some(packed.core_enabled_2)
            } else {
                Some(packed.core_enabled as u16)
            };
            let thread_count = if packed.thread_count == 0xFF {
                Some(packed.thread_count_2)
            } else {
                Some(packed.thread_count as u16)
            };
            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage.into(),
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(structure.find_string(packed.serial_number)?),
                asset_tag: Some(structure.find_string(packed.asset_tag)?),
                part_number: Some(structure.find_string(packed.part_number)?),
                core_count,
                core_enabled,
                thread_count,
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(
                    packed.processor_characteristics,
                )),
            })
        }
    }
}

impl From<u8> for ProcessorFamily {
    fn from(byte: u8) -> ProcessorFamily {
        ProcessorFamily::from(byte as u16)
    }
}

impl From<u16> for ProcessorFamily {
    fn from(word: u16) -> Self {
        match word {
            0x00 => ProcessorFamily::OutOfSpec,
            0x01 => ProcessorFamily::Other,
            0x02 => ProcessorFamily::Unknown,
            0x03 => ProcessorFamily::Intel8086,
            0x04 => ProcessorFamily::Intel80286,
            0x05 => ProcessorFamily::Intel386Processor,
            0x06 => ProcessorFamily::Intel486Processor,
            0x07 => ProcessorFamily::Intel8087,
            0x08 => ProcessorFamily::Intel80287,
            0x09 => ProcessorFamily::Intel80387,
            0x0A => ProcessorFamily::Intel80487,
            0x0B => ProcessorFamily::IntelPentiumProcessor,
            0x0C => ProcessorFamily::PentiumProProcessor,
            0x0D => ProcessorFamily::PentiumIIProcessor,
            0x0E => ProcessorFamily::PentiumProcessorWithMMXTechnology,
            0x0F => ProcessorFamily::IntelCeleronProcessor,
            0x10 => ProcessorFamily::PentiumIIXeonProcessor,
            0x11 => ProcessorFamily::PentiumIIIProcessor,
            0x12 => ProcessorFamily::M1Family,
            0x13 => ProcessorFamily::M2Family,
            0x14 => ProcessorFamily::IntelCeleronMProcessor,
            0x15 => ProcessorFamily::IntelPentium4HTProcessor,
            n @ 0x16..=0x17 => ProcessorFamily::Available(n),
            0x18 => ProcessorFamily::AMDDuronProcessorFamily,
            0x19 => ProcessorFamily::K5Family,
            0x1A => ProcessorFamily::K6Family,
            0x1B => ProcessorFamily::K62,
            0x1C => ProcessorFamily::K63,
            0x1D => ProcessorFamily::AMDAthlonProcessorFamily,
            0x1E => ProcessorFamily::AMD29000Family,
            0x1F => ProcessorFamily::K62Plus,
            0x20 => ProcessorFamily::PowerPCFamily,
            0x21 => ProcessorFamily::PowerPC601,
            0x22 => ProcessorFamily::PowerPC603,
            0x23 => ProcessorFamily::PowerPC603Plus,
            0x24 => ProcessorFamily::PowerPC604,
            0x25 => ProcessorFamily::PowerPC620,
            0x26 => ProcessorFamily::PowerPCX704,
            0x27 => ProcessorFamily::PowerPC750,
            0x28 => ProcessorFamily::IntelCoreDuoProcessor,
            0x29 => ProcessorFamily::IntelCoreDuoMobileProcessor,
            0x2A => ProcessorFamily::IntelCoreSoloMobileProcessor,
            0x2B => ProcessorFamily::IntelAtomProcessor,
            0x2C => ProcessorFamily::IntelCoreMProcessor,
            0x2D => ProcessorFamily::IntelCoreM3Processor,
            0x2E => ProcessorFamily::IntelCoreM5Processor,
            0x2F => ProcessorFamily::IntelCoreM7Processor,
            0x30 => ProcessorFamily::AlphaFamily,
            0x31 => ProcessorFamily::Alpha21064,
            0x32 => ProcessorFamily::Alpha21066,
            0x33 => ProcessorFamily::Alpha21164,
            0x34 => ProcessorFamily::Alpha21164PC,
            0x35 => ProcessorFamily::Alpha21164a,
            0x36 => ProcessorFamily::Alpha21264,
            0x37 => ProcessorFamily::Alpha21364,
            0x38 => ProcessorFamily::AMDTurionIIUltraDualCoreMobileMProcessorFamily,
            0x39 => ProcessorFamily::AMDTurionIIDualCoreMobileMProcessorFamily,
            0x3A => ProcessorFamily::AMDAthlonIIDualCoreMProcessorFamily,
            0x3B => ProcessorFamily::AMDOpteron6100SeriesProcessor,
            0x3C => ProcessorFamily::AMDOpteron4100SeriesProcessor,
            0x3D => ProcessorFamily::AMDOpteron6200SeriesProcessor,
            0x3E => ProcessorFamily::AMDOpteron4200SeriesProcessor,
            0x3F => ProcessorFamily::AMDFXSeriesProcessor,
            0x40 => ProcessorFamily::MIPSFamily,
            0x41 => ProcessorFamily::MIPSR4000,
            0x42 => ProcessorFamily::MIPSR4200,
            0x43 => ProcessorFamily::MIPSR4400,
            0x44 => ProcessorFamily::MIPSR4600,
            0x45 => ProcessorFamily::MIPSR10000,
            0x46 => ProcessorFamily::AMDCSeriesProcessor,
            0x47 => ProcessorFamily::AMDESeriesProcessor,
            0x48 => ProcessorFamily::AMDASeriesProcessor,
            0x49 => ProcessorFamily::AMDGSeriesProcessor,
            0x4A => ProcessorFamily::AMDZSeriesProcessor,
            0x4B => ProcessorFamily::AMDRSeriesProcessor,
            0x4C => ProcessorFamily::AMDOpteron4300SeriesProcessor,
            0x4D => ProcessorFamily::AMDOpteron6300SeriesProcessor,
            0x4E => ProcessorFamily::AMDOpteron3300SeriesProcessor,
            0x4F => ProcessorFamily::AMDFireProSeriesProcessor,
            0x50 => ProcessorFamily::SPARCFamily,
            0x51 => ProcessorFamily::SuperSPARC,
            0x52 => ProcessorFamily::MicroSPARCII,
            0x53 => ProcessorFamily::MicroSPARCIIep,
            0x54 => ProcessorFamily::UltraSPARC,
            0x55 => ProcessorFamily::UltraSPARCII,
            0x56 => ProcessorFamily::UltraSPARCIii,
            0x57 => ProcessorFamily::UltraSPARCIII,
            0x58 => ProcessorFamily::UltraSPARCIIIi,
            n @ 0x59..=0x5F => ProcessorFamily::Available(n),
            0x60 => ProcessorFamily::Motorola68040Family,
            0x61 => ProcessorFamily::Motorola68xxx,
            0x62 => ProcessorFamily::Motorola68000,
            0x63 => ProcessorFamily::Motorola68010,
            0x64 => ProcessorFamily::Motorola68020,
            0x65 => ProcessorFamily::Motorola68030,
            0x66 => ProcessorFamily::AMDAthlonX4QuadCoreProcessorFamily,
            0x67 => ProcessorFamily::AMDOpteronX1000SeriesProcessor,
            0x68 => ProcessorFamily::AMDOpteronX2000SeriesAPU,
            0x69 => ProcessorFamily::AMDOpteronASeriesProcessor,
            0x6A => ProcessorFamily::AMDOpteronX3000SeriesAPU,
            0x6B => ProcessorFamily::AMDZenProcessorFamily,
            n @ 0x6C..=0x6F => ProcessorFamily::Available(n),
            0x70 => ProcessorFamily::HobbitFamily,
            n @ 0x71..=0x77 => ProcessorFamily::Available(n),
            0x78 => ProcessorFamily::CrusoeTM5000Family,
            0x79 => ProcessorFamily::CrusoeTM3000Family,
            0x7A => ProcessorFamily::EfficeonTM8000Family,
            n @ 0x7B..=0x7F => ProcessorFamily::Available(n),
            0x80 => ProcessorFamily::Weitek,
            n @ 0x81 => ProcessorFamily::Available(n),
            0x82 => ProcessorFamily::ItaniumProcessor,
            0x83 => ProcessorFamily::AMDAthlon64ProcessorFamily,
            0x84 => ProcessorFamily::AMDOpteronProcessorFamily,
            0x85 => ProcessorFamily::AMDSempronProcessorFamily,
            0x86 => ProcessorFamily::AMDTurion64MobileTechnology,
            0x87 => ProcessorFamily::DualCoreAMDOpteronProcessorFamily,
            0x88 => ProcessorFamily::AMDAthlon64X2DualCoreProcessorFamily,
            0x89 => ProcessorFamily::AMDTurion64X2MobileTechnology,
            0x8A => ProcessorFamily::QuadCoreAMDOpteronProcessorFamily,
            0x8B => ProcessorFamily::ThirdGenerationAMDOpteronProcessorFamily,
            0x8C => ProcessorFamily::AMDPhenomFXQuadCoreProcessorFamily,
            0x8D => ProcessorFamily::AMDPhenomX4QuadCoreProcessorFamily,
            0x8E => ProcessorFamily::AMDPhenomX2DualCoreProcessorFamily,
            0x8F => ProcessorFamily::AMDAthlonX2DualCoreProcessorFamily,
            0x90 => ProcessorFamily::PARISCFamily,
            0x91 => ProcessorFamily::PARISC8500,
            0x92 => ProcessorFamily::PARISC8000,
            0x93 => ProcessorFamily::PARISC7300LC,
            0x94 => ProcessorFamily::PARISC7200,
            0x95 => ProcessorFamily::PARISC7100LC,
            0x96 => ProcessorFamily::PARISC7100,
            n @ 0x97..=0x9F => ProcessorFamily::Available(n),
            0xA0 => ProcessorFamily::V30Family,
            0xA1 => ProcessorFamily::QuadCoreIntelXeonProcessor3200Series,
            0xA2 => ProcessorFamily::DualCoreIntelXeonProcessor3000Series,
            0xA3 => ProcessorFamily::QuadCoreIntelXeonProcessor5300Series,
            0xA4 => ProcessorFamily::DualCoreIntelXeonProcessor5100Series,
            0xA5 => ProcessorFamily::DualCoreIntelXeonProcessor5000Series,
            0xA6 => ProcessorFamily::DualCoreIntelXeonProcessorLV,
            0xA7 => ProcessorFamily::DualCoreIntelXeonProcessorULV,
            0xA8 => ProcessorFamily::DualCoreIntelXeonProcessor7100Series,
            0xA9 => ProcessorFamily::QuadCoreIntelXeonProcessor5400Series,
            0xAA => ProcessorFamily::QuadCoreIntelXeonProcessor,
            0xAB => ProcessorFamily::DualCoreIntelXeonProcessor5200Series,
            0xAC => ProcessorFamily::DualCoreIntelXeonProcessor7200Series,
            0xAD => ProcessorFamily::QuadCoreIntelXeonProcessor7300Series,
            0xAE => ProcessorFamily::QuadCoreIntelXeonProcessor7400Series,
            0xAF => ProcessorFamily::MultiCoreIntelXeonProcessor7400Series,
            0xB0 => ProcessorFamily::PentiumIIIXeonProcessor,
            0xB1 => ProcessorFamily::PentiumIIIProcessorWithIntelSpeedStepTechnology,
            0xB2 => ProcessorFamily::Pentium4Processor,
            0xB3 => ProcessorFamily::IntelXeonProcessor,
            0xB4 => ProcessorFamily::AS400Family,
            0xB5 => ProcessorFamily::IntelXeonProcessorMP,
            0xB6 => ProcessorFamily::AMDAthlonXPProcessorFamily,
            0xB7 => ProcessorFamily::AMDAthlonMPProcessorFamily,
            0xB8 => ProcessorFamily::IntelItanium2Processor,
            0xB9 => ProcessorFamily::IntelPentiumMProcessor,
            0xBA => ProcessorFamily::IntelCeleronDProcessor,
            0xBB => ProcessorFamily::IntelPentiumDProcessor,
            0xBC => ProcessorFamily::IntelPentiumProcessorExtremeEdition,
            0xBD => ProcessorFamily::IntelCoreSoloProcessor,
            0xBE => ProcessorFamily::Ambiguous,
            0xBF => ProcessorFamily::IntelCore2DuoProcessor,
            0xC0 => ProcessorFamily::IntelCore2SoloProcessor,
            0xC1 => ProcessorFamily::IntelCore2ExtremeProcessor,
            0xC2 => ProcessorFamily::IntelCore2QuadProcessor,
            0xC3 => ProcessorFamily::IntelCore2ExtremeMobileProcessor,
            0xC4 => ProcessorFamily::IntelCore2DuoMobileProcessor,
            0xC5 => ProcessorFamily::IntelCore2SoloMobileProcessor,
            0xC6 => ProcessorFamily::IntelCoreI7Processor,
            0xC7 => ProcessorFamily::DualCoreIntelCeleronProcessor,
            0xC8 => ProcessorFamily::IBM390Family,
            0xC9 => ProcessorFamily::G4,
            0xCA => ProcessorFamily::G5,
            0xCB => ProcessorFamily::ESA390G6,
            0xCC => ProcessorFamily::ZArchitectureBase,
            0xCD => ProcessorFamily::IntelCoreI5Processor,
            0xCE => ProcessorFamily::IntelCoreI3Processor,
            0xCF => ProcessorFamily::IntelCoreI9Processor,
            n @ 0xD0..=0xD1 => ProcessorFamily::Available(n),
            0xD2 => ProcessorFamily::VIAC7MProcessorFamily,
            0xD3 => ProcessorFamily::VIAC7DProcessorFamily,
            0xD4 => ProcessorFamily::VIAC7ProcessorFamily,
            0xD5 => ProcessorFamily::VIAEdenProcessorFamily,
            0xD6 => ProcessorFamily::MultiCoreIntelXeonProcessor,
            0xD7 => ProcessorFamily::DualCoreIntelXeonProcessor3xxxSeries,
            0xD8 => ProcessorFamily::QuadCoreIntelXeonProcessor3xxxSeries,
            0xD9 => ProcessorFamily::VIANanoProcessorFamily,
            0xDA => ProcessorFamily::DualCoreIntelXeonProcessor5xxxSeries,
            0xDB => ProcessorFamily::QuadCoreIntelXeonProcessor5xxxSeries,
            n @ 0xDC => ProcessorFamily::Available(n),
            0xDD => ProcessorFamily::DualCoreIntelXeonProcessor7xxxSeries,
            0xDE => ProcessorFamily::QuadCoreIntelXeonProcessor7xxxSeries,
            0xDF => ProcessorFamily::MultiCoreIntelXeonProcessor7xxxSeries,
            0xE0 => ProcessorFamily::MultiCoreIntelXeonProcessor3400Series,
            n @ 0xE1..=0xE3 => ProcessorFamily::Available(n),
            0xE4 => ProcessorFamily::AMDOpteron3000SeriesProcessor,
            0xE5 => ProcessorFamily::AMDSempronIIProcessor,
            0xE6 => ProcessorFamily::EmbeddedAMDOpteronQuadCoreProcessorFamily,
            0xE7 => ProcessorFamily::AMDPhenomTripleCoreProcessorFamily,
            0xE8 => ProcessorFamily::AMDTurionUltraDualCoreMobileProcessorFamily,
            0xE9 => ProcessorFamily::AMDTurionDualCoreMobileProcessorFamily,
            0xEA => ProcessorFamily::AMDAthlonDualCoreProcessorFamily,
            0xEB => ProcessorFamily::AMDSempronSIProcessorFamily,
            0xEC => ProcessorFamily::AMDPhenomIIProcessorFamily,
            0xED => ProcessorFamily::AMDAthlonIIProcessorFamily,
            0xEE => ProcessorFamily::SixCoreAMDOpteronProcessorFamily,
            0xEF => ProcessorFamily::AMDSempronMProcessorFamily,
            n @ 0xF0..=0xF9 => ProcessorFamily::Available(n),
            0xFA => ProcessorFamily::I860,
            0xFB => ProcessorFamily::I960,
            n @ 0xFC..=0xFD => ProcessorFamily::Available(n),
            0xFE => ProcessorFamily::ProcessorFamily2,
            n @ 0xFF => ProcessorFamily::NotUsed(n),
            0x100 => ProcessorFamily::ARMv7,
            0x101 => ProcessorFamily::ARMv8,
            0x102 => ProcessorFamily::ARMv9,
            0x103 => ProcessorFamily::Available(0x103),
            0x104 => ProcessorFamily::SH3,
            0x105 => ProcessorFamily::SH4,
            n @ 0x106..=0x117 => ProcessorFamily::Available(n),
            0x118 => ProcessorFamily::ARM,
            0x119 => ProcessorFamily::StrongARM,
            n @ 0x11A..=0x12B => ProcessorFamily::Available(n),
            0x12C => ProcessorFamily::Cyrix6x86,
            0x12D => ProcessorFamily::MediaGX,
            0x12E => ProcessorFamily::MII,
            n @ 0x12F..=0x13F => ProcessorFamily::Available(n),
            0x140 => ProcessorFamily::WinChip,
            n @ 0x141..=0x15D => ProcessorFamily::Available(n),
            0x15E => ProcessorFamily::DSP,
            n @ 0x15F..=0x1F3 => ProcessorFamily::Available(n),
            0x1F4 => ProcessorFamily::VideoProcessor,
            n @ 0x1F5..=0x1FF => ProcessorFamily::Available(n),
            0x200 => ProcessorFamily::RISCVRV32,
            0x201 => ProcessorFamily::RISCVRV64,
            0x202 => ProcessorFamily::RISCVRV128,
            n @ 0x203..=0xFFFD => ProcessorFamily::Available(n),
            0xFFFE => ProcessorFamily::ForFutureUse,
            n @ 0xFFFF => ProcessorFamily::NotUsed(n),
        }
    }
}
impl fmt::Display for ProcessorFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorFamily::Other => write!(f, "Other"),
            ProcessorFamily::Unknown => write!(f, "Unknown"),
            ProcessorFamily::Intel8086 => write!(f, "8086"),
            ProcessorFamily::Intel80286 => write!(f, "80286"),
            ProcessorFamily::Intel386Processor => write!(f, "Intel386™ processor"),
            ProcessorFamily::Intel486Processor => write!(f, "Intel486™ processor"),
            ProcessorFamily::Intel8087 => write!(f, "8087"),
            ProcessorFamily::Intel80287 => write!(f, "80287"),
            ProcessorFamily::Intel80387 => write!(f, "80387"),
            ProcessorFamily::Intel80487 => write!(f, "80487"),
            ProcessorFamily::IntelPentiumProcessor => write!(f, "Intel® Pentium® processor"),
            ProcessorFamily::PentiumProProcessor => write!(f, "Pentium® Pro processor"),
            ProcessorFamily::PentiumIIProcessor => write!(f, "Pentium® II processor"),
            ProcessorFamily::PentiumProcessorWithMMXTechnology => {
                write!(f, "Pentium® processor with MMX™ technology")
            }
            ProcessorFamily::IntelCeleronProcessor => write!(f, "Intel® Celeron® processor"),
            ProcessorFamily::PentiumIIXeonProcessor => write!(f, "Pentium® II Xeon™ processor"),
            ProcessorFamily::PentiumIIIProcessor => write!(f, "Pentium® III processor"),
            ProcessorFamily::M1Family => write!(f, "M1 Family"),
            ProcessorFamily::M2Family => write!(f, "M2 Family"),
            ProcessorFamily::IntelCeleronMProcessor => write!(f, "Intel® Celeron® M processor"),
            ProcessorFamily::IntelPentium4HTProcessor => {
                write!(f, "Intel® Pentium® 4 HT processor")
            }
            ProcessorFamily::AMDDuronProcessorFamily => {
                write!(f, "AMD Duron™ Processor Family [1]")
            }
            ProcessorFamily::K5Family => write!(f, "K5 Family [1]"),
            ProcessorFamily::K6Family => write!(f, "K6 Family [1]"),
            ProcessorFamily::K62 => write!(f, "K6-2"),
            ProcessorFamily::K63 => write!(f, "K6-3"),
            ProcessorFamily::AMDAthlonProcessorFamily => {
                write!(f, "AMD Athlon™ Processor Family [1]")
            }
            ProcessorFamily::AMD29000Family => write!(f, "AMD29000 Family"),
            ProcessorFamily::K62Plus => write!(f, "K6-2+"),
            ProcessorFamily::PowerPCFamily => write!(f, "Power PC Family"),
            ProcessorFamily::PowerPC601 => write!(f, "Power PC 601"),
            ProcessorFamily::PowerPC603 => write!(f, "Power PC 603"),
            ProcessorFamily::PowerPC603Plus => write!(f, "Power PC 603+"),
            ProcessorFamily::PowerPC604 => write!(f, "Power PC 604"),
            ProcessorFamily::PowerPC620 => write!(f, "Power PC 620"),
            ProcessorFamily::PowerPCX704 => write!(f, "Power PC x704"),
            ProcessorFamily::PowerPC750 => write!(f, "Power PC 750"),
            ProcessorFamily::IntelCoreDuoProcessor => write!(f, "Intel® Core™ Duo processor"),
            ProcessorFamily::IntelCoreDuoMobileProcessor => {
                write!(f, "Intel® Core™ Duo mobile processor")
            }
            ProcessorFamily::IntelCoreSoloMobileProcessor => {
                write!(f, "Intel® Core™ Solo mobile processor")
            }
            ProcessorFamily::IntelAtomProcessor => write!(f, "Intel® Atom™ processor"),
            ProcessorFamily::IntelCoreMProcessor => write!(f, "Intel® Core™ M processor"),
            ProcessorFamily::IntelCoreM3Processor => write!(f, "Intel(R) Core(TM) m3 processor"),
            ProcessorFamily::IntelCoreM5Processor => write!(f, "Intel(R) Core(TM) m5 processor"),
            ProcessorFamily::IntelCoreM7Processor => write!(f, "Intel(R) Core(TM) m7 processor"),
            ProcessorFamily::AlphaFamily => write!(f, "Alpha Family [2]"),
            ProcessorFamily::Alpha21064 => write!(f, "Alpha 21064"),
            ProcessorFamily::Alpha21066 => write!(f, "Alpha 21066"),
            ProcessorFamily::Alpha21164 => write!(f, "Alpha 21164"),
            ProcessorFamily::Alpha21164PC => write!(f, "Alpha 21164PC"),
            ProcessorFamily::Alpha21164a => write!(f, "Alpha 21164a"),
            ProcessorFamily::Alpha21264 => write!(f, "Alpha 21264"),
            ProcessorFamily::Alpha21364 => write!(f, "Alpha 21364"),
            ProcessorFamily::AMDTurionIIUltraDualCoreMobileMProcessorFamily => {
                write!(f, "AMD Turion™ II Ultra Dual-Core Mobile M Processor Family")
            }
            ProcessorFamily::AMDTurionIIDualCoreMobileMProcessorFamily => {
                write!(f, "AMD Turion™ II Dual-Core Mobile M Processor Family")
            }
            ProcessorFamily::AMDAthlonIIDualCoreMProcessorFamily => {
                write!(f, "AMD Athlon™ II Dual-Core M Processor Family")
            }
            ProcessorFamily::AMDOpteron6100SeriesProcessor => {
                write!(f, "AMD Opteron™ 6100 Series Processor")
            }
            ProcessorFamily::AMDOpteron4100SeriesProcessor => {
                write!(f, "AMD Opteron™ 4100 Series Processor")
            }
            ProcessorFamily::AMDOpteron6200SeriesProcessor => {
                write!(f, "AMD Opteron™ 6200 Series Processor")
            }
            ProcessorFamily::AMDOpteron4200SeriesProcessor => {
                write!(f, "AMD Opteron™ 4200 Series Processor")
            }
            ProcessorFamily::AMDFXSeriesProcessor => write!(f, "AMD FX™ Series Processor"),
            ProcessorFamily::MIPSFamily => write!(f, "MIPS Family"),
            ProcessorFamily::MIPSR4000 => write!(f, "MIPS R4000"),
            ProcessorFamily::MIPSR4200 => write!(f, "MIPS R4200"),
            ProcessorFamily::MIPSR4400 => write!(f, "MIPS R4400"),
            ProcessorFamily::MIPSR4600 => write!(f, "MIPS R4600"),
            ProcessorFamily::MIPSR10000 => write!(f, "MIPS R10000"),
            ProcessorFamily::AMDCSeriesProcessor => write!(f, "AMD C-Series Processor"),
            ProcessorFamily::AMDESeriesProcessor => write!(f, "AMD E-Series Processor"),
            ProcessorFamily::AMDASeriesProcessor => write!(f, "AMD A-Series Processor"),
            ProcessorFamily::AMDGSeriesProcessor => write!(f, "AMD G-Series Processor"),
            ProcessorFamily::AMDZSeriesProcessor => write!(f, "AMD Z-Series Processor"),
            ProcessorFamily::AMDRSeriesProcessor => write!(f, "AMD R-Series Processor"),
            ProcessorFamily::AMDOpteron4300SeriesProcessor => {
                write!(f, "AMD Opteron™ 4300 Series Processor")
            }
            ProcessorFamily::AMDOpteron6300SeriesProcessor => {
                write!(f, "AMD Opteron™ 6300 Series Processor")
            }
            ProcessorFamily::AMDOpteron3300SeriesProcessor => {
                write!(f, "AMD Opteron™ 3300 Series Processor")
            }
            ProcessorFamily::AMDFireProSeriesProcessor => {
                write!(f, "AMD FirePro™ Series Processor")
            }
            ProcessorFamily::SPARCFamily => write!(f, "SPARC Family"),
            ProcessorFamily::SuperSPARC => write!(f, "SuperSPARC"),
            ProcessorFamily::MicroSPARCII => write!(f, "microSPARC II"),
            ProcessorFamily::MicroSPARCIIep => write!(f, "microSPARC IIep"),
            ProcessorFamily::UltraSPARC => write!(f, "UltraSPARC"),
            ProcessorFamily::UltraSPARCII => write!(f, "UltraSPARC II"),
            ProcessorFamily::UltraSPARCIii => write!(f, "UltraSPARC Iii"),
            ProcessorFamily::UltraSPARCIII => write!(f, "UltraSPARC III"),
            ProcessorFamily::UltraSPARCIIIi => write!(f, "UltraSPARC IIIi"),
            ProcessorFamily::Motorola68040Family => write!(f, "68040 Family"),
            ProcessorFamily::Motorola68xxx => write!(f, "68xxx"),
            ProcessorFamily::Motorola68000 => write!(f, "68000"),
            ProcessorFamily::Motorola68010 => write!(f, "68010"),
            ProcessorFamily::Motorola68020 => write!(f, "68020"),
            ProcessorFamily::Motorola68030 => write!(f, "68030"),
            ProcessorFamily::AMDAthlonX4QuadCoreProcessorFamily => {
                write!(f, "AMD Athlon(TM) X4 Quad-Core Processor Family")
            }
            ProcessorFamily::AMDOpteronX1000SeriesProcessor => {
                write!(f, "AMD Opteron(TM) X1000 Series Processor")
            }
            ProcessorFamily::AMDOpteronX2000SeriesAPU => {
                write!(f, "AMD Opteron(TM) X2000 Series APU")
            }
            ProcessorFamily::AMDOpteronASeriesProcessor => {
                write!(f, "AMD Opteron(TM) A-Series Processor")
            }
            ProcessorFamily::AMDOpteronX3000SeriesAPU => {
                write!(f, "AMD Opteron(TM) X3000 Series APU")
            }
            ProcessorFamily::AMDZenProcessorFamily => write!(f, "AMD Zen Processor Family"),
            ProcessorFamily::HobbitFamily => write!(f, "Hobbit Family"),
            ProcessorFamily::CrusoeTM5000Family => write!(f, "Crusoe™ TM5000 Family"),
            ProcessorFamily::CrusoeTM3000Family => write!(f, "Crusoe™ TM3000 Family"),
            ProcessorFamily::EfficeonTM8000Family => write!(f, "Efficeon™ TM8000 Family"),
            ProcessorFamily::Weitek => write!(f, "Weitek"),
            ProcessorFamily::AvailableForAssignment => write!(f, "Available for assignment"),
            ProcessorFamily::ItaniumProcessor => write!(f, "Itanium™ processor"),
            ProcessorFamily::AMDAthlon64ProcessorFamily => {
                write!(f, "AMD Athlon™ 64 Processor Family")
            }
            ProcessorFamily::AMDOpteronProcessorFamily => {
                write!(f, "AMD Opteron™ Processor Family")
            }
            ProcessorFamily::AMDSempronProcessorFamily => {
                write!(f, "AMD Sempron™ Processor Family")
            }
            ProcessorFamily::AMDTurion64MobileTechnology => {
                write!(f, "AMD Turion™ 64 Mobile Technology")
            }
            ProcessorFamily::DualCoreAMDOpteronProcessorFamily => {
                write!(f, "Dual-Core AMD Opteron™ Processor Family")
            }
            ProcessorFamily::AMDAthlon64X2DualCoreProcessorFamily => {
                write!(f, "AMD Athlon™ 64 X2 Dual-Core Processor Family")
            }
            ProcessorFamily::AMDTurion64X2MobileTechnology => {
                write!(f, "AMD Turion™ 64 X2 Mobile Technology")
            }
            ProcessorFamily::QuadCoreAMDOpteronProcessorFamily => {
                write!(f, "Quad-Core AMD Opteron™ Processor Family")
            }
            ProcessorFamily::ThirdGenerationAMDOpteronProcessorFamily => {
                write!(f, "Third-Generation AMD Opteron™ Processor Family")
            }
            ProcessorFamily::AMDPhenomFXQuadCoreProcessorFamily => {
                write!(f, "AMD Phenom™ FX Quad-Core Processor Family")
            }
            ProcessorFamily::AMDPhenomX4QuadCoreProcessorFamily => {
                write!(f, "AMD Phenom™ X4 Quad-Core Processor Family")
            }
            ProcessorFamily::AMDPhenomX2DualCoreProcessorFamily => {
                write!(f, "AMD Phenom™ X2 Dual-Core Processor Family")
            }
            ProcessorFamily::AMDAthlonX2DualCoreProcessorFamily => {
                write!(f, "AMD Athlon™ X2 Dual-Core Processor Family")
            }
            ProcessorFamily::PARISCFamily => write!(f, "PA-RISC Family"),
            ProcessorFamily::PARISC8500 => write!(f, "PA-RISC 8500"),
            ProcessorFamily::PARISC8000 => write!(f, "PA-RISC 8000"),
            ProcessorFamily::PARISC7300LC => write!(f, "PA-RISC 7300LC"),
            ProcessorFamily::PARISC7200 => write!(f, "PA-RISC 7200"),
            ProcessorFamily::PARISC7100LC => write!(f, "PA-RISC 7100LC"),
            ProcessorFamily::PARISC7100 => write!(f, "PA-RISC 7100"),
            ProcessorFamily::V30Family => write!(f, "V30 Family"),
            ProcessorFamily::QuadCoreIntelXeonProcessor3200Series => {
                write!(f, "Quad-Core Intel® Xeon® processor 3200 Series")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor3000Series => {
                write!(f, "Dual-Core Intel® Xeon® processor 3000 Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor5300Series => {
                write!(f, "Quad-Core Intel® Xeon® processor 5300 Series")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor5100Series => {
                write!(f, "Dual-Core Intel® Xeon® processor 5100 Series")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor5000Series => {
                write!(f, "Dual-Core Intel® Xeon® processor 5000 Series")
            }
            ProcessorFamily::DualCoreIntelXeonProcessorLV => {
                write!(f, "Dual-Core Intel® Xeon® processor LV")
            }
            ProcessorFamily::DualCoreIntelXeonProcessorULV => {
                write!(f, "Dual-Core Intel® Xeon® processor ULV")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor7100Series => {
                write!(f, "Dual-Core Intel® Xeon® processor 7100 Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor5400Series => {
                write!(f, "Quad-Core Intel® Xeon® processor 5400 Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor => {
                write!(f, "Quad-Core Intel® Xeon® processor")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor5200Series => {
                write!(f, "Dual-Core Intel® Xeon® processor 5200 Series")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor7200Series => {
                write!(f, "Dual-Core Intel® Xeon® processor 7200 Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor7300Series => {
                write!(f, "Quad-Core Intel® Xeon® processor 7300 Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor7400Series => {
                write!(f, "Quad-Core Intel® Xeon® processor 7400 Series")
            }
            ProcessorFamily::MultiCoreIntelXeonProcessor7400Series => {
                write!(f, "Multi-Core Intel® Xeon® processor 7400 Series")
            }
            ProcessorFamily::PentiumIIIXeonProcessor => write!(f, "Pentium® III Xeon™ processor"),
            ProcessorFamily::PentiumIIIProcessorWithIntelSpeedStepTechnology => {
                write!(f, "Pentium® III Processor with Intel® SpeedStep™ Technology")
            }
            ProcessorFamily::Pentium4Processor => write!(f, "Pentium® 4 Processor"),
            ProcessorFamily::IntelXeonProcessor => write!(f, "Intel® Xeon® processor"),
            ProcessorFamily::AS400Family => write!(f, "AS400 Family"),
            ProcessorFamily::IntelXeonProcessorMP => write!(f, "Intel® Xeon™ processor MP"),
            ProcessorFamily::AMDAthlonXPProcessorFamily => {
                write!(f, "AMD Athlon™ XP Processor Family")
            }
            ProcessorFamily::AMDAthlonMPProcessorFamily => {
                write!(f, "AMD Athlon™ MP Processor Family")
            }
            ProcessorFamily::IntelItanium2Processor => write!(f, "Intel® Itanium® 2 processor"),
            ProcessorFamily::IntelPentiumMProcessor => write!(f, "Intel® Pentium® M processor"),
            ProcessorFamily::IntelCeleronDProcessor => write!(f, "Intel® Celeron® D processor"),
            ProcessorFamily::IntelPentiumDProcessor => write!(f, "Intel® Pentium® D processor"),
            ProcessorFamily::IntelPentiumProcessorExtremeEdition => {
                write!(f, "Intel® Pentium® Processor Extreme Edition")
            }
            ProcessorFamily::IntelCoreSoloProcessor => write!(f, "Intel® Core™ Solo Processor"),
            ProcessorFamily::Ambiguous => write!(f, "Ambiguous"),
            ProcessorFamily::IntelCore2DuoProcessor => write!(f, "Intel® Core™ 2 Duo Processor"),
            ProcessorFamily::IntelCore2SoloProcessor => write!(f, "Intel® Core™ 2 Solo processor"),
            ProcessorFamily::IntelCore2ExtremeProcessor => {
                write!(f, "Intel® Core™ 2 Extreme processor")
            }
            ProcessorFamily::IntelCore2QuadProcessor => write!(f, "Intel® Core™ 2 Quad processor"),
            ProcessorFamily::IntelCore2ExtremeMobileProcessor => {
                write!(f, "Intel® Core™ 2 Extreme mobile processor")
            }
            ProcessorFamily::IntelCore2DuoMobileProcessor => {
                write!(f, "Intel® Core™ 2 Duo mobile processor")
            }
            ProcessorFamily::IntelCore2SoloMobileProcessor => {
                write!(f, "Intel® Core™ 2 Solo mobile processor")
            }
            ProcessorFamily::IntelCoreI7Processor => write!(f, "Intel® Core™ i7 processor"),
            ProcessorFamily::DualCoreIntelCeleronProcessor => {
                write!(f, "Dual-Core Intel® Celeron® processor")
            }
            ProcessorFamily::IBM390Family => write!(f, "IBM390 Family"),
            ProcessorFamily::G4 => write!(f, "G4"),
            ProcessorFamily::G5 => write!(f, "G5"),
            ProcessorFamily::ESA390G6 => write!(f, "ESA/390 G6"),
            ProcessorFamily::ZArchitectureBase => write!(f, "z/Architecture base"),
            ProcessorFamily::IntelCoreI5Processor => write!(f, "Intel® Core™ i5 processor"),
            ProcessorFamily::IntelCoreI3Processor => write!(f, "Intel® Core™ i3 processor"),
            ProcessorFamily::IntelCoreI9Processor => write!(f, "Intel® Core™ i9 processor"),
            ProcessorFamily::VIAC7MProcessorFamily => write!(f, "VIA C7™-M Processor Family"),
            ProcessorFamily::VIAC7DProcessorFamily => write!(f, "VIA C7™-D Processor Family"),
            ProcessorFamily::VIAC7ProcessorFamily => write!(f, "VIA C7™ Processor Family"),
            ProcessorFamily::VIAEdenProcessorFamily => write!(f, "VIA Eden™ Processor Family"),
            ProcessorFamily::MultiCoreIntelXeonProcessor => {
                write!(f, "Multi-Core Intel® Xeon® processor")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor3xxxSeries => {
                write!(f, "Dual-Core Intel® Xeon® processor 3xxx Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor3xxxSeries => {
                write!(f, "Quad-Core Intel® Xeon® processor 3xxx Series")
            }
            ProcessorFamily::VIANanoProcessorFamily => write!(f, "VIA Nano™ Processor Family"),
            ProcessorFamily::DualCoreIntelXeonProcessor5xxxSeries => {
                write!(f, "Dual-Core Intel® Xeon® processor 5xxx Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor5xxxSeries => {
                write!(f, "Quad-Core Intel® Xeon® processor 5xxx Series")
            }
            ProcessorFamily::DualCoreIntelXeonProcessor7xxxSeries => {
                write!(f, "Dual-Core Intel® Xeon® processor 7xxx Series")
            }
            ProcessorFamily::QuadCoreIntelXeonProcessor7xxxSeries => {
                write!(f, "Quad-Core Intel® Xeon® processor 7xxx Series")
            }
            ProcessorFamily::MultiCoreIntelXeonProcessor7xxxSeries => {
                write!(f, "Multi-Core Intel® Xeon® processor 7xxx Series")
            }
            ProcessorFamily::MultiCoreIntelXeonProcessor3400Series => {
                write!(f, "Multi-Core Intel® Xeon® processor 3400 Series")
            }
            ProcessorFamily::AMDOpteron3000SeriesProcessor => {
                write!(f, "AMD Opteron™ 3000 Series Processor")
            }
            ProcessorFamily::AMDSempronIIProcessor => write!(f, "AMD Sempron™ II Processor"),
            ProcessorFamily::EmbeddedAMDOpteronQuadCoreProcessorFamily => {
                write!(f, "Embedded AMD Opteron™ Quad-Core Processor Family")
            }
            ProcessorFamily::AMDPhenomTripleCoreProcessorFamily => {
                write!(f, "AMD Phenom™ Triple-Core Processor Family")
            }
            ProcessorFamily::AMDTurionUltraDualCoreMobileProcessorFamily => {
                write!(f, "AMD Turion™ Ultra Dual-Core Mobile Processor Family")
            }
            ProcessorFamily::AMDTurionDualCoreMobileProcessorFamily => {
                write!(f, "AMD Turion™ Dual-Core Mobile Processor Family")
            }
            ProcessorFamily::AMDAthlonDualCoreProcessorFamily => {
                write!(f, "AMD Athlon™ Dual-Core Processor Family")
            }
            ProcessorFamily::AMDSempronSIProcessorFamily => {
                write!(f, "AMD Sempron™ SI Processor Family")
            }
            ProcessorFamily::AMDPhenomIIProcessorFamily => {
                write!(f, "AMD Phenom™ II Processor Family")
            }
            ProcessorFamily::AMDAthlonIIProcessorFamily => {
                write!(f, "AMD Athlon™ II Processor Family")
            }
            ProcessorFamily::SixCoreAMDOpteronProcessorFamily => {
                write!(f, "Six-Core AMD Opteron™ Processor Family")
            }
            ProcessorFamily::AMDSempronMProcessorFamily => {
                write!(f, "AMD Sempron™ M Processor Family")
            }
            ProcessorFamily::I860 => write!(f, "i860"),
            ProcessorFamily::I960 => write!(f, "i960"),
            ProcessorFamily::ARMv7 => write!(f, "ARMv7"),
            ProcessorFamily::ARMv8 => write!(f, "ARMv8"),
            ProcessorFamily::ARMv9 => write!(f, "ARMv9"),
            ProcessorFamily::SH3 => write!(f, "SH-3"),
            ProcessorFamily::SH4 => write!(f, "SH-4"),
            ProcessorFamily::ARM => write!(f, "ARM"),
            ProcessorFamily::StrongARM => write!(f, "StrongARM"),
            ProcessorFamily::Cyrix6x86 => write!(f, "6x86"),
            ProcessorFamily::MediaGX => write!(f, "MediaGX"),
            ProcessorFamily::MII => write!(f, "MII"),
            ProcessorFamily::WinChip => write!(f, "WinChip"),
            ProcessorFamily::DSP => write!(f, "DSP"),
            ProcessorFamily::VideoProcessor => write!(f, "Video Processor"),
            ProcessorFamily::RISCVRV32 => write!(f, "RISC-V RV32"),
            ProcessorFamily::RISCVRV64 => write!(f, "RISC-V RV64"),
            ProcessorFamily::RISCVRV128 => write!(f, "RISC-V RV128"),
            ProcessorFamily::ForFutureUse => write!(f, "For special use in the future"),
            ProcessorFamily::ProcessorFamily2 => {
                write!(f, "Processor Family 2 has the enumerated value")
            }
            ProcessorFamily::Available(n) => write!(f, "Available {:#X}", n),
            ProcessorFamily::NotUsed(n) => write!(f, "Not used. {:X}h is the un-initialized value of Flash memory.", n),
            ProcessorFamily::OutOfSpec => write!(f, "OUT OF SPEC"),
        }
    }
}

impl From<u8> for Voltage {
    fn from(byte: u8) -> Self {
        if (byte & 0b1000_0000) == 0 {
            if (byte & 0b0111_1000) != 0 {
                Self::Undefined(byte)
            } else {
                Self::Legacy(VoltageLegacy::from_bits_truncate(byte))
            }
        } else {
            Self::Current(byte & 0b0111_1111)
        }
    }
}
impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Current(v) => write!(f, "Current voltage: {:.1} V", *v as f32 / 10.0),
            Self::Undefined(n) => write!(f, "Undefined {:#b}", n),
            Self::Legacy(legacy) => {
                let s55 = if legacy.contains(VoltageLegacy::VOLTAGE_CAPABILITY_5V0) {
                    "5.5V "
                } else {
                    ""
                };
                let s33 = if legacy.contains(VoltageLegacy::VOLTAGE_CAPABILITY_3V3) {
                    "3.3V "
                } else {
                    ""
                };
                let s29 = if legacy.contains(VoltageLegacy::VOLTAGE_CAPABILITY_2V9) {
                    "2.9V "
                } else {
                    ""
                };
                if s55.is_empty() && s33.is_empty() && s29.is_empty() {
                    write!(f, "Voltage capability unknown")
                } else {
                    write!(f, "Processor socket accept: {}{}{}", s55, s33, s29)
                }
            }
        }
    }
}

impl From<u8> for ProcessorUpgrade {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => ProcessorUpgrade::Other,
            0x02 => ProcessorUpgrade::Unknown,
            0x03 => ProcessorUpgrade::DaughterBoard,
            0x04 => ProcessorUpgrade::ZIFSocket,
            0x05 => ProcessorUpgrade::ReplaceablePiggyBack,
            0x06 => ProcessorUpgrade::None,
            0x07 => ProcessorUpgrade::LIFSocket,
            0x08 => ProcessorUpgrade::Slot1,
            0x09 => ProcessorUpgrade::Slot2,
            0x0a => ProcessorUpgrade::Socket370,
            0x0b => ProcessorUpgrade::SlotA,
            0x0c => ProcessorUpgrade::SlotM,
            0x0d => ProcessorUpgrade::Socket423,
            0x0e => ProcessorUpgrade::SocketA,
            0x0f => ProcessorUpgrade::Socket478,
            0x10 => ProcessorUpgrade::Socket754,
            0x11 => ProcessorUpgrade::Socket940,
            0x12 => ProcessorUpgrade::Socket939,
            0x13 => ProcessorUpgrade::SocketmPGA604,
            0x14 => ProcessorUpgrade::SocketLGA771,
            0x15 => ProcessorUpgrade::SocketLGA775,
            0x16 => ProcessorUpgrade::SocketS1,
            0x17 => ProcessorUpgrade::SocketAM2,
            0x18 => ProcessorUpgrade::SocketF,
            0x19 => ProcessorUpgrade::SocketLGA1366,
            0x1a => ProcessorUpgrade::SocketG34,
            0x1b => ProcessorUpgrade::SocketAM3,
            0x1c => ProcessorUpgrade::SocketC32,
            0x1d => ProcessorUpgrade::SocketLGA1156,
            0x1e => ProcessorUpgrade::SocketLGA1567,
            0x1f => ProcessorUpgrade::SocketPGA988A,
            0x20 => ProcessorUpgrade::SocketBGA1288,
            0x21 => ProcessorUpgrade::SocketrPGA988B,
            0x22 => ProcessorUpgrade::SocketBGA1023,
            0x23 => ProcessorUpgrade::SocketBGA1224,
            0x24 => ProcessorUpgrade::SocketLGA1155,
            0x25 => ProcessorUpgrade::SocketLGA1356,
            0x26 => ProcessorUpgrade::SocketLGA2011,
            0x27 => ProcessorUpgrade::SocketFS1,
            0x28 => ProcessorUpgrade::SocketFS2,
            0x29 => ProcessorUpgrade::SocketFM1,
            0x2a => ProcessorUpgrade::SocketFM2,
            0x2b => ProcessorUpgrade::SocketLGA2011Three,
            0x2c => ProcessorUpgrade::SocketLGA1356Three,
            0x2d => ProcessorUpgrade::SocketLGA1150,
            0x2e => ProcessorUpgrade::SocketBGA1168,
            0x2f => ProcessorUpgrade::SocketBGA1234,
            0x30 => ProcessorUpgrade::SocketBGA1364,
            0x31 => ProcessorUpgrade::SocketAM4,
            0x32 => ProcessorUpgrade::SocketLGA1151,
            0x33 => ProcessorUpgrade::SocketBGA1356,
            0x34 => ProcessorUpgrade::SocketBGA1440,
            0x35 => ProcessorUpgrade::SocketBGA1515,
            0x36 => ProcessorUpgrade::SocketLGA3647,
            0x37 => ProcessorUpgrade::SocketSP3,
            0x38 => ProcessorUpgrade::SocketSP3r2,
            0x39 => ProcessorUpgrade::SocketLGA2066,
            0x3a => ProcessorUpgrade::SocketBGA1392,
            0x3b => ProcessorUpgrade::SocketBGA1510,
            0x3c => ProcessorUpgrade::SocketBGA1528,
            0x3d => ProcessorUpgrade::SocketLGA4189,
            0x3e => ProcessorUpgrade::SocketLGA1200,
            n => ProcessorUpgrade::Undefined(n),
        }
    }
}
impl fmt::Display for ProcessorUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorUpgrade::Other => write!(f, "Other"),
            ProcessorUpgrade::Unknown => write!(f, "Unknown"),
            ProcessorUpgrade::DaughterBoard => write!(f, "Daughter Board"),
            ProcessorUpgrade::ZIFSocket => write!(f, "ZIF Socket"),
            ProcessorUpgrade::ReplaceablePiggyBack => write!(f, "Replaceable Piggy Back"),
            ProcessorUpgrade::None => write!(f, "None"),
            ProcessorUpgrade::LIFSocket => write!(f, "LIF Socket"),
            ProcessorUpgrade::Slot1 => write!(f, "Slot 1"),
            ProcessorUpgrade::Slot2 => write!(f, "Slot 2"),
            ProcessorUpgrade::Socket370 => write!(f, "370-pin socket"),
            ProcessorUpgrade::SlotA => write!(f, "Slot A"),
            ProcessorUpgrade::SlotM => write!(f, "Slot M"),
            ProcessorUpgrade::Socket423 => write!(f, "Socket 423"),
            ProcessorUpgrade::SocketA => write!(f, "Socket A (Socket 462)"),
            ProcessorUpgrade::Socket478 => write!(f, "Socket 478"),
            ProcessorUpgrade::Socket754 => write!(f, "Socket 754"),
            ProcessorUpgrade::Socket940 => write!(f, "Socket 940"),
            ProcessorUpgrade::Socket939 => write!(f, "Socket 939"),
            ProcessorUpgrade::SocketmPGA604 => write!(f, "Socket mPGA604"),
            ProcessorUpgrade::SocketLGA771 => write!(f, "Socket LGA771"),
            ProcessorUpgrade::SocketLGA775 => write!(f, "Socket LGA775"),
            ProcessorUpgrade::SocketS1 => write!(f, "Socket S1"),
            ProcessorUpgrade::SocketAM2 => write!(f, "Socket AM2"),
            ProcessorUpgrade::SocketF => write!(f, "Socket F (1207)"),
            ProcessorUpgrade::SocketLGA1366 => write!(f, "Socket LGA1366"),
            ProcessorUpgrade::SocketG34 => write!(f, "Socket G34"),
            ProcessorUpgrade::SocketAM3 => write!(f, "Socket AM3"),
            ProcessorUpgrade::SocketC32 => write!(f, "Socket C32"),
            ProcessorUpgrade::SocketLGA1156 => write!(f, "Socket LGA1156"),
            ProcessorUpgrade::SocketLGA1567 => write!(f, "Socket LGA1567"),
            ProcessorUpgrade::SocketPGA988A => write!(f, "Socket PGA988A"),
            ProcessorUpgrade::SocketBGA1288 => write!(f, "Socket BGA1288"),
            ProcessorUpgrade::SocketrPGA988B => write!(f, "Socket rPGA988B"),
            ProcessorUpgrade::SocketBGA1023 => write!(f, "Socket BGA1023"),
            ProcessorUpgrade::SocketBGA1224 => write!(f, "Socket BGA1224"),
            ProcessorUpgrade::SocketLGA1155 => write!(f, "Socket LGA1155"),
            ProcessorUpgrade::SocketLGA1356 => write!(f, "Socket LGA1356"),
            ProcessorUpgrade::SocketLGA2011 => write!(f, "Socket LGA2011"),
            ProcessorUpgrade::SocketFS1 => write!(f, "Socket FS1"),
            ProcessorUpgrade::SocketFS2 => write!(f, "Socket FS2"),
            ProcessorUpgrade::SocketFM1 => write!(f, "Socket FM1"),
            ProcessorUpgrade::SocketFM2 => write!(f, "Socket FM2"),
            ProcessorUpgrade::SocketLGA2011Three => write!(f, "Socket LGA2011-3"),
            ProcessorUpgrade::SocketLGA1356Three => write!(f, "Socket LGA1356-3"),
            ProcessorUpgrade::SocketLGA1150 => write!(f, "Socket LGA1150"),
            ProcessorUpgrade::SocketBGA1168 => write!(f, "Socket BGA1168"),
            ProcessorUpgrade::SocketBGA1234 => write!(f, "Socket BGA1234"),
            ProcessorUpgrade::SocketBGA1364 => write!(f, "Socket BGA1364"),
            ProcessorUpgrade::SocketAM4 => write!(f, "Socket AM4"),
            ProcessorUpgrade::SocketLGA1151 => write!(f, "Socket LGA1151"),
            ProcessorUpgrade::SocketBGA1356 => write!(f, "Socket BGA1356"),
            ProcessorUpgrade::SocketBGA1440 => write!(f, "Socket BGA1440"),
            ProcessorUpgrade::SocketBGA1515 => write!(f, "Socket BGA1515"),
            ProcessorUpgrade::SocketLGA3647 => write!(f, "Socket LGA3647-1"),
            ProcessorUpgrade::SocketSP3 => write!(f, "Socket SP3"),
            ProcessorUpgrade::SocketSP3r2 => write!(f, "Socket SP3r2"),
            ProcessorUpgrade::SocketLGA2066 => write!(f, "Socket LGA2066"),
            ProcessorUpgrade::SocketBGA1392 => write!(f, "Socket BGA1392"),
            ProcessorUpgrade::SocketBGA1510 => write!(f, "Socket BGA1510"),
            ProcessorUpgrade::SocketBGA1528 => write!(f, "Socket BGA1528"),
            ProcessorUpgrade::SocketLGA4189 => write!(f, "Socket LGA4189"),
            ProcessorUpgrade::SocketLGA1200 => write!(f, "Socket LGA1200"),
            ProcessorUpgrade::Undefined(n) => write!(f, "Undefined {}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InfoType;

    #[test]
    fn processor_family() {
        use super::ProcessorFamily::*;
        for i in 0..=0xFFFF {
            let (e, s) = match i {
                0x01 => (Other, "Other".into()),
                0x15 => (IntelPentium4HTProcessor, "Intel® Pentium® 4 HT processor".into()),
                0x0D => (PentiumIIProcessor, "Pentium® II processor".into()),
                0x28 => (IntelCoreDuoProcessor, "Intel® Core™ Duo processor".into()),
                0x3D => (
                    AMDOpteron6200SeriesProcessor,
                    "AMD Opteron™ 6200 Series Processor".into(),
                ),
                0x50 => (SPARCFamily, "SPARC Family".into()),
                0x65 => (Motorola68030, "68030".into()),
                0x8F => (
                    AMDAthlonX2DualCoreProcessorFamily,
                    "AMD Athlon™ X2 Dual-Core Processor Family".into(),
                ),
                0xBE => (Ambiguous, "Ambiguous".into()),
                0xC9 => (G4, "G4".into()),
                0xFB => (I960, "i960".into()),
                0x100 => (ARMv7, "ARMv7".into()),
                0x104 => (SH3, "SH-3".into()),
                0x118 => (ARM, "ARM".into()),
                0x140 => (WinChip, "WinChip".into()),
                n @ 0x16..=0x17 => (Available(n), format!("Available {:#X}", n)),
                n @ 0x59..=0x5F => (Available(n), format!("Available {:#X}", n)),
                n @ 0x6C..=0x6F => (Available(n), format!("Available {:#X}", n)),
                n @ 0x71..=0x77 => (Available(n), format!("Available {:#X}", n)),
                n @ 0x7B..=0x7F => (Available(n), format!("Available {:#X}", n)),
                n @ 0x81 => (Available(n), format!("Available {:#X}", n)),
                n @ 0x97..=0x9F => (Available(n), format!("Available {:#X}", n)),
                n @ 0xD0..=0xD1 => (Available(n), format!("Available {:#X}", n)),
                n @ 0xDC => (Available(n), format!("Available {:#X}", n)),
                n @ 0xE1..=0xE3 => (Available(n), format!("Available {:#X}", n)),
                n @ 0xF0..=0xF9 => (Available(n), format!("Available {:#X}", n)),
                n @ 0xFC..=0xFD => (Available(n), format!("Available {:#X}", n)),
                n @ 0x1F5..=0x1FF => (Available(n), format!("Available {:#X}", n)),
                n @ 0x203..=0xFFFD => (Available(n), format!("Available {:#X}", n)),
                n @ 0xFF => (
                    NotUsed(n),
                    format!("Not used. {:X}h is the un-initialized value of Flash memory.", n),
                ),
                0xFFFE => (ForFutureUse, "For special use in the future".into()),
                n @ 0xFFFF => (
                    NotUsed(n),
                    format!("Not used. {:X}h is the un-initialized value of Flash memory.", n),
                ),
                _ => continue,
            };
            assert_eq!(e, i.into(), "{:#x}", i);
            assert_eq!(s, format!("{}", e));
        }
    }

    #[test]
    fn processor_voltage() {
        let test_data = [
            (
                0b0000_0000,
                Voltage::Legacy(VoltageLegacy::empty()),
                "Voltage capability unknown",
            ),
            (
                0b0000_0001,
                Voltage::Legacy(VoltageLegacy::VOLTAGE_CAPABILITY_5V0),
                "Processor socket accept: 5.5V ",
            ),
            (
                0b0000_0010,
                Voltage::Legacy(VoltageLegacy::VOLTAGE_CAPABILITY_3V3),
                "Processor socket accept: 3.3V ",
            ),
            (
                0b0000_0111,
                Voltage::Legacy(
                    VoltageLegacy::VOLTAGE_CAPABILITY_5V0
                        | VoltageLegacy::VOLTAGE_CAPABILITY_3V3
                        | VoltageLegacy::VOLTAGE_CAPABILITY_2V9,
                ),
                "Processor socket accept: 5.5V 3.3V 2.9V ",
            ),
            (0b0000_1000, Voltage::Undefined(8), "Undefined 0b1000"),
            (0b1001_0010, Voltage::Current(18), "Current voltage: 1.8 V"),
            (0b1111_1111, Voltage::Current(127), "Current voltage: 12.7 V"),
        ];
        for (byte, sample, display) in test_data.iter() {
            let result = Voltage::from(*byte);
            assert_eq!(*sample, result, "Byte: {:#b}", byte);
            assert_eq!(format!("{}", result), format!("{}", display), "Byte: {:#b}", byte);
        }
    }

    #[test]
    fn processor_upgrade() {
        use super::ProcessorUpgrade::*;
        for i in 0..=0xFF {
            let (e, s) = match i {
                0x01 => (Other, "Other".into()),
                0x13 => (SocketmPGA604, "Socket mPGA604".into()),
                0x18 => (SocketF, "Socket F (1207)".into()),
                0x2B => (SocketLGA2011Three, "Socket LGA2011-3".into()),
                0x3E => (SocketLGA1200, "Socket LGA1200".into()),
                n @ 0x3F..=0xFF => (Undefined(n), format!("Undefined {}", n)),
                _ => continue,
            };
            assert_eq!(e, i.into(), "{:#x}", i);
            assert_eq!(s, format!("{}", e));
        }
    }

    #[test]
    fn smbios_2_8_processor_intel_atom_parses() {
        let structure = RawStructure {
            version: (2, 8).into(),
            info: InfoType::Processor,
            length: 0x2a,
            handle: 0x48,
            // data and strings from processor handler, eg: dmidecode -H 0x48 -u
            data: &[
                // omit first 4 header bytes
                // 04 // type
                // 2a // length
                // 48 00 // handle
                0x01, // socket_designation
                0x03, // processor_type
                0x2b, // processor_family
                0x02, // processor_manufacturer
                0xd8, 0x06, 0x04, 0x00, 0xff, 0xfb, 0xeb, 0xbf, // processor_id
                0x03, // processor_version
                0x90, // voltage
                0x64, 0x00, // external_clock
                0x28, 0x0a, // max_speed
                0x60, 0x09, // current_speed
                0x41, // status
                0x01, // processor_upgrade
                0x46, 0x00, // l1_cache
                0x47, 0x00, // l2_cache
                0xff, 0xff, // l3_cache
                0x00, // serial_number
                0x04, // asset_tag
                0x00, // part_number
                0x08, // core_count
                0x08, // core_enabled
                0x08, // thread_count
                0x04, 0x00, // processor_characteristics
                0x00, 0x00, // processor_family2
            ],
            strings: &[
                // CPU0
                0x43, 0x50, 0x55, 0x30, 0x00, // Intel(R) Corporation
                0x49, 0x6E, 0x74, 0x65, 0x6C, 0x28, 0x52, 0x29, 0x20, 0x43, 0x6F, 0x72, 0x70, 0x6F, 0x72, 0x61, 0x74,
                0x69, 0x6F, 0x6E, 0x00, // Intel(R) Atom(TM) CPU  C2750  @ 2.40GHz
                0x49, 0x6E, 0x74, 0x65, 0x6C, 0x28, 0x52, 0x29, 0x20, 0x41, 0x74, 0x6F, 0x6D, 0x28, 0x54, 0x4D, 0x29,
                0x20, 0x43, 0x50, 0x55, 0x20, 0x20, 0x43, 0x32, 0x37, 0x35, 0x30, 0x20, 0x20, 0x40, 0x20, 0x32, 0x2E,
                0x34, 0x30, 0x47, 0x48, 0x7A, 0x00, // ProcessorInfo_ASSET_TAG
                0x50, 0x72, 0x6F, 0x63, 0x65, 0x73, 0x73, 0x6F, 0x72, 0x49, 0x6E, 0x66, 0x6F, 0x5F, 0x41, 0x53, 0x53,
                0x45, 0x54, 0x5F, 0x54, 0x41, 0x47, 0x00,
            ],
        };

        assert_eq!(
            Processor {
                handle: 0x48,
                socket_designation: "CPU0",
                processor_type: ProcessorType::CentralProcessor,
                processor_family: ProcessorFamily::IntelAtomProcessor,
                processor_manufacturer: "Intel(R) Corporation",
                processor_id: 13829424153406736088,
                processor_version: "Intel(R) Atom(TM) CPU  C2750  @ 2.40GHz",
                voltage: Voltage::Current(16),
                external_clock: 100,
                max_speed: 2600,
                current_speed: 2400,
                status: ProcessorStatus::from_bits_truncate(0b0100_0001),
                processor_upgrade: ProcessorUpgrade::Other,
                l1_cache_handle: Some(70),
                l2_cache_handle: Some(71),
                l3_cache_handle: Some(65535),
                serial_number: Some(""),
                asset_tag: Some("ProcessorInfo_ASSET_TAG"),
                part_number: Some(""),
                core_count: Some(8),
                core_enabled: Some(8),
                thread_count: Some(8),
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(0b0000_0100)),
            },
            Processor::try_from(structure).unwrap()
        );
    }

    #[test]
    // Processor info was manipulated to exercise processor_family_2 parsing
    fn smbios_2_8_processor_parses_with_processor_family_2() {
        let structure = RawStructure {
            version: (2, 8).into(),
            info: InfoType::Processor,
            length: 0x2a,
            handle: 0x48,
            // data and strings from processor handler, eg: dmidecode -H 0x48 -u
            // $ hexdump -s 0x4c6 -n 42 -C processor_bin
            data: &[
                // 04 // type
                // 2a // length
                // 48 00 // handle
                0x01, // socket_designation
                0x03, // processor_type
                0xfe, // processor_family
                0x02, // processor_manufacturer
                0xd8, 0x06, 0x04, 0x00, 0xff, 0xfb, 0xeb, 0xbf, // processor_id
                0x03, // processor_version
                0x90, // voltage
                0x64, 0x00, // external_clock
                0x28, 0x0a, // max_speed
                0x60, 0x09, // current_speed
                0x41, // status
                0x01, // processor_upgrade
                0x46, 0x00, // l1_cache
                0x47, 0x00, // l2_cache
                0xff, 0xff, // l3_cache
                0x00, // serial_number
                0x04, // asset_tag
                0x00, // part_number
                0x08, // core_count
                0x08, // core_enabled
                0x08, // thread_count
                0x04, 0x00, // processor_characteristics
                0x18, 0x01, // processor_family2
            ],
            strings: &[
                // CPU0
                0x43, 0x50, 0x55, 0x30, 0x00, // FAKE MANUFACTURER
                0x46, 0x41, 0x4b, 0x45, 0x20, 0x4d, 0x41, 0x4e, 0x55, 0x46, 0x41, 0x43, 0x54, 0x55, 0x52, 0x45, 0x52,
                0x00, // FAKE VERSION
                0x46, 0x41, 0x4b, 0x45, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x00, // FAKE ASSET
                0x46, 0x41, 0x4b, 0x45, 0x20, 0x41, 0x53, 0x53, 0x45, 0x54, 0x20, 0x54, 0x41, 0x47, 0x00,
            ],
        };

        assert_eq!(
            Processor {
                handle: 0x48,
                socket_designation: "CPU0",
                processor_type: ProcessorType::CentralProcessor,
                processor_family: ProcessorFamily::ARM,
                processor_manufacturer: "FAKE MANUFACTURER",
                processor_id: 13829424153406736088,
                processor_version: "FAKE VERSION",
                voltage: Voltage::Current(16),
                external_clock: 100,
                max_speed: 2600,
                current_speed: 2400,
                status: ProcessorStatus::from_bits_truncate(0b0100_0001),
                processor_upgrade: ProcessorUpgrade::Other,
                l1_cache_handle: Some(70),
                l2_cache_handle: Some(71),
                l3_cache_handle: Some(65535),
                serial_number: Some(""),
                asset_tag: Some("FAKE ASSET TAG"),
                part_number: Some(""),
                core_count: Some(8),
                core_enabled: Some(8),
                thread_count: Some(8),
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(0b0000_0100)),
            },
            Processor::try_from(structure).unwrap()
        );
    }

    #[test]
    fn zero_process_family() {
        let structure = RawStructure {
            version: (3, 2).into(),
            info: InfoType::Processor,
            length: 0x32,
            handle: 0x000c,
            // data and strings from processor handler, eg: dmidecode -H 0x48 -u
            // $ hexdump -s 0x4c6 -n 42 -C processor_bin
            data: &[
                // 04 // type
                // 32 // length
                // 0c 00 // handle
                0x01, // socket_designation
                0x03, // processor_type
                0x00, // processor_family
                0x00, // processor_manufacturer
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // processor_id
                0x00, // processor_version
                0x00, // voltage
                0x00, 0x00, // external_clock
                0xa0, 0x0f, // max_speed
                0x00, 0x00, // current_speed
                0x00, // status
                0x3f, // processor_upgrade
                0xff, 0xff, // l1_cache
                0xff, 0xff, // l2_cache
                0xff, 0xff, // l3_cache
                0x00, // serial_number
                0x00, // asset_tag
                0x00, // part_number
                0x00, // core_count
                0x00, // core_enabled
                0x00, // thread_count
                0x00, 0x00, // processor_characteristics
                0x00, 0x00, // processor_family2
                0x00, 0x00, // core count 2
                0x00, 0x00, // core enabled 2
                0x00, 0x00, // thread count 2
                0x00, 0x00,
            ],
            strings: &[
                // CPU0
                0x43, 0x50, 0x55, 0x30, 0x00,
            ],
        };

        assert_eq!(
            Processor {
                handle: 0x000c,
                socket_designation: "CPU0",
                processor_type: ProcessorType::CentralProcessor,
                processor_family: ProcessorFamily::OutOfSpec,
                processor_manufacturer: "",
                processor_id: 0,
                processor_version: "",
                voltage: Voltage::Legacy(VoltageLegacy::empty()),
                external_clock: 0,
                max_speed: 4000,
                current_speed: 0,
                status: ProcessorStatus::from_bits_truncate(0b0),
                processor_upgrade: ProcessorUpgrade::Undefined(0x3f),
                l1_cache_handle: Some(0xffff),
                l2_cache_handle: Some(0xffff),
                l3_cache_handle: Some(0xffff),
                serial_number: Some(""),
                asset_tag: Some(""),
                part_number: Some(""),
                core_count: Some(0),
                core_enabled: Some(0),
                thread_count: Some(0),
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(0b0)),
            },
            Processor::try_from(structure).unwrap()
        );
    }
}
