use core::fmt;


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
    }
}

/// The `Processor` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Processor<'buffer> {
    pub handle: u16,
    pub socket_designation: &'buffer str,
    pub processor_type: ProcessorType,
    pub processor_family: u16,
    pub processor_manufacturer: &'buffer str,
    pub processor_id: u64,
    pub processor_version: &'buffer str,
    pub voltage: u8,
    pub external_clock: u16,
    pub max_speed: u16,
    pub current_speed: u16,
    pub status: ProcessorStatus,
    pub processor_upgrade: u8,
    pub l1_cache_handle: Option<u16>,
    pub l2_cache_handle: Option<u16>,
    pub l3_cache_handle: Option<u16>,
    pub serial_number: Option<&'buffer str>,
    pub asset_tag: Option<&'buffer str>,
    pub part_number: Option<&'buffer str>,
    pub core_count: Option<u16>,
    pub core_enabled: Option<u16>,
    pub thread_count: Option<u16>,
    pub processor_characteristics: Option<ProcessorCharacteristics>,
}

/// The `Cache Information` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Cache<'buffer> {
    pub handle: u16,
    /// String number for reference designation EXAMPLE: “CACHE1”, 0
    pub socket_designation: &'buffer str,
    /// Cache Configuration
    pub cache_configuration: CacheConfiguration,
    /// Maximum size that can be installed
    pub maximum_cache_size: CacheSize,
    /// Same format as Max Cache Size field; set to 0 if no cache is installed
    pub installed_size: CacheSize,
    /// Supported SRAM Type
    pub supported_sram_type: CacheSramType,
    /// Current SRAM Type
    pub current_sram_type: CacheSramType,
    /// Cache module speed, in nanoseconds. The value is 0 if the speed is unknown.
    pub cache_speed: Option<u8>,
    /// Error-correction scheme supported by this cache component
    pub error_correction_type: Option<CacheErrorCorrectionType>,
    /// Logical type of cache
    pub system_cache_type: Option<SystemCacheType>,
    /// Associativity of the cache
    pub associativity: Option<CacheAssociativity>,
    /// If this field is present, for cache sizes of 2047 MB or smaller the value in the Max size
    /// in given granularity portion of the field equals the size given in the corresponding
    /// portion of the Maximum Cache Size field, and the Granularity bit matches the value of the
    /// Granularity bit in the Maximum Cache Size field.  For Cache sizes greater than 2047 MB, the
    /// Maximum Cache Size field is set to 0xFFFF and the Maximum Cache Size 2 field is present,
    /// the Granularity bit is set to 1b, and the size set as required;
    pub maximum_cache_size_2: Option<CacheSize2>,
    /// Same format as Maximum Cache Size 2 field; Absent or set to 0 if no cache is installed.
    pub installed_size_2: Option<CacheSize2>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CacheConfiguration {
    /// Cache Level – 1 through 8
    level: CacheLevel,
    /// Cache Socketed (e.g. Cache on a Stick)
    socketed: bool,
    /// Location, relative to the CPU module
    location: CacheLocation,
    /// Enabled/Disabled (at boot time)
    enabled_at_boot_time: bool,
    /// Operational Mode
    operational_mode: CacheOperationalMode,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CacheLevel {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CacheLocation {
    Internal,
    External,
    Reserved,
    Unknown,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CacheOperationalMode {
    WriteThrough,
    WriteBack,
    ValuesWithMemoryAddress,
    Unknown,
}

/// Cache size is same for Maximum Cache Size and Installed Size
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CacheSize {
    Granularity1K(u16),
    Granularity64K(u16),
}

bitflags! {
    /// Cache SRAM Type is same for Supported SRAM Type and Current SRAM Type
    pub struct CacheSramType: u16 {
        const OTHER             = 0b0000_0001;
        const UNKNOWN           = 0b0000_0010;
        const NONBURST          = 0b0000_0100;
        const BURST             = 0b0000_1000;
        const PIPELINE_BURST    = 0b0001_0000;
        const SYNCHRONOUS       = 0b0010_0000;
        const ASYNCHRONOUS      = 0b0100_0000;
    }
}

/// Error Correction Type field
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CacheErrorCorrectionType {
    Other,
    Unknown,
    None,
    Parity,
    SingleBitEcc,
    MultiBitEcc,
    Undefined(u8),
}

/// The cache type for a cache level (L1, L2, L3, ...) is type 03h (Instruction) when all the
/// caches at that level are Instruction caches. The cache type for a specific cache level (L1, L2,
/// L3, ...) is type 04h (Data) when all the caches at that level are Data caches. The cache type
/// for a cache level (L1, L2, L3, ...) is type 05h (Unified) when the caches at that level are a
/// mix of Instruction and Data caches.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SystemCacheType {
    Other,
    Unknown,
    Instruction,
    Data,
    Unified,
    Undefined(u8),
}

/// Associativity field
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CacheAssociativity {
    Other,
    Unknown,
    DirectMapped,
    TwowaySetAssociative,
    FourWaySetAssociative,
    FullyAssociative,
    EightWaySetAssociative,
    SixteenWaySetAssociative,
    TwelveWaySetAssociative,
    TwentyFourWaySetAssociative,
    ThirtyTwoWaySetAssociative,
    FourtyEightWaySetAssociative,
    SixtyFourWaySetAssociative,
    TwentyWaySetAssociative,
    Undefined(u8),
}

/// Cache size is same for Maximum Cache Size and Installed Size
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CacheSize2 {
    Granularity1K(u32),
    Granularity64K(u32),
}


impl<'buffer> Processor<'buffer> {
    pub(crate) fn try_from(structure: super::RawStructure<'buffer>) -> Result<Processor<'buffer>, super::MalformedStructureError> {
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
                processor_family: packed.processor_family as u16,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage,
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
                processor_family: packed.processor_family as u16,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage,
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
                processor_family: packed.processor_family as u16,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage,
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
                processor_family: packed.processor_family as u16,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage,
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

            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family_2,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage,
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
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(packed.processor_characteristics)),
            })
        } else {
            let_as_struct!(packed, ProcessorPacked_3_0, structure.data);

            Ok(Processor {
                handle: structure.handle,
                socket_designation: structure.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family_2,
                processor_manufacturer: structure.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: structure.find_string(packed.processor_version)?,
                voltage: packed.voltage,
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
                core_count: Some(packed.core_count_2),
                core_enabled: Some(packed.core_enabled_2),
                thread_count: Some(packed.thread_count_2),
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(packed.processor_characteristics)),
            })
        }
    }
}


impl<'buffer> Cache<'buffer> {
    pub(crate) fn try_from(structure: super::RawStructure<'buffer>) -> Result<Cache<'buffer>, super::MalformedStructureError> {
        #[repr(C)]
        #[repr(packed)]
        struct CachePacked_3_1 {
            socket_designation: u8,
            cache_configuration: u16,
            maximum_cache_size: u16,
            installed_size: u16,
            supported_sram_type: u16,
            current_sram_type: u16,
            cache_speed: u8,
            error_correction_type: u8,
            system_cache_type: u8,
            associativity: u8,
            maximum_cache_size_2: u32,
            installed_size_2: u32,
        }

        #[repr(C)]
        #[repr(packed)]
        struct CachePacked_2_1 {
            socket_designation: u8,
            cache_configuration: u16,
            maximum_cache_size: u16,
            installed_size: u16,
            supported_sram_type: u16,
            current_sram_type: u16,
            cache_speed: u8,
            error_correction_type: u8,
            system_cache_type: u8,
            associativity: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct CachePacked_2_0 {
            socket_designation: u8,
            cache_configuration: u16,
            maximum_cache_size: u16,
            installed_size: u16,
            supported_sram_type: u16,
            current_sram_type: u16,
        }

        match structure.version {
            v if v > (3, 1).into() => {
                let_as_struct!(packed, CachePacked_3_1, structure.data);
                Ok(Cache {
                    handle: structure.handle,
                    socket_designation: structure.find_string(packed.socket_designation)?,
                    cache_configuration: packed.cache_configuration.into(),
                    maximum_cache_size: packed.maximum_cache_size.into(),
                    installed_size: packed.installed_size.into(),
                    supported_sram_type: CacheSramType::from_bits_truncate(packed.supported_sram_type),
                    current_sram_type: CacheSramType::from_bits_truncate(packed.current_sram_type),
                    cache_speed: Some(packed.cache_speed),
                    error_correction_type: Some(packed.error_correction_type.into()),
                    system_cache_type: Some(packed.system_cache_type.into()),
                    associativity: Some(packed.associativity.into()),
                    maximum_cache_size_2: Some(packed.maximum_cache_size_2.into()),
                    installed_size_2: Some(packed.installed_size_2.into()),
                })
            },
            v if v > (2, 1).into() => {
                let_as_struct!(packed, CachePacked_2_1, structure.data);
                Ok(Cache {
                    handle: structure.handle,
                    socket_designation: structure.find_string(packed.socket_designation)?,
                    cache_configuration: packed.cache_configuration.into(),
                    maximum_cache_size: packed.maximum_cache_size.into(),
                    installed_size: packed.installed_size.into(),
                    supported_sram_type: CacheSramType::from_bits_truncate(packed.supported_sram_type),
                    current_sram_type: CacheSramType::from_bits_truncate(packed.current_sram_type),
                    cache_speed: Some(packed.cache_speed),
                    error_correction_type: Some(packed.error_correction_type.into()),
                    system_cache_type: Some(packed.system_cache_type.into()),
                    associativity: Some(packed.associativity.into()),
                    maximum_cache_size_2: None,
                    installed_size_2: None,
                })
            },
            v if v > (2, 0).into() => {
                let_as_struct!(packed, CachePacked_2_0, structure.data);
                Ok(Cache {
                    handle: structure.handle,
                    socket_designation: structure.find_string(packed.socket_designation)?,
                    cache_configuration: packed.cache_configuration.into(),
                    maximum_cache_size: packed.maximum_cache_size.into(),
                    installed_size: packed.installed_size.into(),
                    supported_sram_type: CacheSramType::from_bits_truncate(packed.supported_sram_type),
                    current_sram_type: CacheSramType::from_bits_truncate(packed.current_sram_type),
                    cache_speed: None,
                    error_correction_type: None,
                    system_cache_type: None,
                    associativity: None,
                    maximum_cache_size_2: None,
                    installed_size_2: None,
                })
            },
            _ => unreachable!(),
        }
    }
}

impl From<u16> for CacheConfiguration {
    fn from(word: u16) -> CacheConfiguration {
        CacheConfiguration {
            level: CacheLevel::from(word & 0b0000_0111),
            socketed: (word & 0b0000_1000) >> 3 == 1,
            location: CacheLocation::from((word & 0b0110_0000) >> 5),
            enabled_at_boot_time: (word & 0b1000_0000) >> 7 == 1,
            operational_mode: CacheOperationalMode::from((word & 0b0000_0011_0000_0000) >> 8),
        }
    }
}

impl From<u16> for CacheSize {
    fn from(dword: u16) -> CacheSize {
        let val = dword & (!(1 << 15));
        if dword & (1 << 15) == 0 {
            CacheSize::Granularity1K(val)
        } else {
            CacheSize::Granularity64K(val)
        }
    }
}
impl CacheSize {
    pub fn bytes(&self) -> u64 {
        match &self {
            Self::Granularity1K(val) => (*val as u64)  * (1 << 10),
            Self::Granularity64K(val) => (*val as u64) * (1 << 16),
        }
    }
}

impl From<u16> for CacheLevel {
    fn from(byte: u16) -> CacheLevel {
        match byte {
            0 => CacheLevel::L1,
            1 => CacheLevel::L2,
            2 => CacheLevel::L3,
            3 => CacheLevel::L4,
            4 => CacheLevel::L5,
            5 => CacheLevel::L6,
            6 => CacheLevel::L7,
            7 => CacheLevel::L8,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for CacheLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::L1 => write!(f, "L1"),
            Self::L2 => write!(f, "L2"),
            Self::L3 => write!(f, "L3"),
            Self::L4 => write!(f, "L4"),
            Self::L5 => write!(f, "L5"),
            Self::L6 => write!(f, "L6"),
            Self::L7 => write!(f, "L7"),
            Self::L8 => write!(f, "L8"),
        }
    }
}

impl From<u16> for CacheLocation {
    fn from(byte: u16) -> CacheLocation {
        match byte {
            0 => CacheLocation::Internal,
            1 => CacheLocation::External,
            2 => CacheLocation::Reserved,
            3 => CacheLocation::Unknown,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for CacheLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal  => write!(f, "Internal"),
            Self::External  => write!(f, "External"),
            Self::Reserved  => write!(f, "Reserved"),
            Self::Unknown   => write!(f, "Unknown"),
        }
    }
}

impl From<u16> for CacheOperationalMode  {
    fn from(byte: u16) -> CacheOperationalMode {
        match byte {
            0 => CacheOperationalMode::WriteThrough,
            1 => CacheOperationalMode::WriteBack,
            2 => CacheOperationalMode::ValuesWithMemoryAddress,
            3 => CacheOperationalMode::Unknown,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for CacheOperationalMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WriteThrough              => write!(f, "Write Through"),
            Self::WriteBack                 => write!(f, "Write Back"),
            Self::ValuesWithMemoryAddress   => write!(f, "Values with Memory Address"),
            Self::Unknown                   => write!(f, "Unknown"),
        }
    }
}



impl From<u8> for CacheErrorCorrectionType  {
    fn from(byte: u8) -> CacheErrorCorrectionType {
        match byte {
            0x01 => CacheErrorCorrectionType::Other,
            0x02 => CacheErrorCorrectionType::Unknown,
            0x03 => CacheErrorCorrectionType::None,
            0x04 => CacheErrorCorrectionType::Parity,
            0x05 => CacheErrorCorrectionType::SingleBitEcc,
            0x06 => CacheErrorCorrectionType::MultiBitEcc,
            t => CacheErrorCorrectionType::Undefined(t),
        }
    }
}
impl fmt::Display for CacheErrorCorrectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other         => write!(f, "Other"),
            Self::Unknown       => write!(f, "Unknown"),
            Self::None          => write!(f, "None"),
            Self::Parity        => write!(f, "Parity"),
            Self::SingleBitEcc  => write!(f, "Single-bit ECC"),
            Self::MultiBitEcc   => write!(f, "Multi-bit ECC"),
            Self::Undefined(t)  => write!(f, "Undefined: {}", t),
        }
    }
}

impl From<u8> for SystemCacheType  {
    fn from(byte: u8) -> SystemCacheType {
        match byte {
            0x01 => SystemCacheType::Other,
            0x02 => SystemCacheType::Unknown,
            0x03 => SystemCacheType::Instruction,
            0x04 => SystemCacheType::Data,
            0x05 => SystemCacheType::Unified,
            t => SystemCacheType::Undefined(t),
        }
    }
}
impl fmt::Display for SystemCacheType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other         => write!(f, "Other"),
            Self::Unknown       => write!(f, "Unknown"),
            Self::Instruction   => write!(f, "Instruction"),
            Self::Data          => write!(f, "Data"),
            Self::Unified       => write!(f, "Unified"),
            Self::Undefined(t)  => write!(f, "Undefined: {}", t),
        }
    }
}

impl From<u8> for CacheAssociativity  {
    fn from(byte: u8) -> CacheAssociativity {
        match byte {
            0x01 => CacheAssociativity::Other,
            0x02 => CacheAssociativity::Unknown,
            0x03 => CacheAssociativity::DirectMapped,
            0x04 => CacheAssociativity::TwowaySetAssociative,
            0x05 => CacheAssociativity::FourWaySetAssociative,
            0x06 => CacheAssociativity::FullyAssociative,
            0x07 => CacheAssociativity::EightWaySetAssociative,
            0x08 => CacheAssociativity::SixteenWaySetAssociative,
            0x09 => CacheAssociativity::TwelveWaySetAssociative,
            0x0A => CacheAssociativity::TwentyFourWaySetAssociative,
            0x0B => CacheAssociativity::ThirtyTwoWaySetAssociative,
            0x0C => CacheAssociativity::FourtyEightWaySetAssociative,
            0x0D => CacheAssociativity::SixtyFourWaySetAssociative,
            0x0E => CacheAssociativity::TwentyWaySetAssociative,
            t  => CacheAssociativity::Undefined(t),
        }}
}
impl fmt::Display for CacheAssociativity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other         => write!(f, "Other"),
            Self::Unknown                       => write!(f, "Unknown"),
            Self::DirectMapped                  => write!(f, "Direct Mapped"),
            Self::TwowaySetAssociative          => write!(f, "2-way Set-Associative"),
            Self::FourWaySetAssociative         => write!(f, "4-way Set-Associative"),
            Self::FullyAssociative              => write!(f, "Fully Associative"),
            Self::EightWaySetAssociative        => write!(f, "8-way Set-Associative"),
            Self::SixteenWaySetAssociative      => write!(f, "16-way Set-Associative"),
            Self::TwelveWaySetAssociative       => write!(f, "12-way Set-Associative"),
            Self::TwentyFourWaySetAssociative   => write!(f, "24-way Set-Associative"),
            Self::ThirtyTwoWaySetAssociative    => write!(f, "32-way Set-Associative"),
            Self::FourtyEightWaySetAssociative  => write!(f, "48-way Set-Associative"),
            Self::SixtyFourWaySetAssociative    => write!(f, "64-way Set-Associative"),
            Self::TwentyWaySetAssociative       => write!(f, "20-way Set-Associative"),
            Self::Undefined(t)                  => write!(f, "Undefined: {}", t),
        }
    }
}

impl From<u32> for CacheSize2 {
    fn from(dword: u32) -> CacheSize2 {
        let val = dword & (!(1 << 31));
        if dword & (1 << 31) == 0 {
            CacheSize2::Granularity1K(val)
        } else {
            CacheSize2::Granularity64K(val)
        }
    }
}
impl CacheSize2 {
    pub fn bytes(&self) -> u64 {
        match &self {
            Self::Granularity1K(val) => (*val as u64)  * (1 << 10),
            Self::Granularity64K(val) => (*val as u64) * (1 << 16),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_configuration() {
        let data = 0b0000_0010_1010_1010;
        let sample = CacheConfiguration {
            level: CacheLevel::L3,
            socketed: true,
            location: CacheLocation::External,
            enabled_at_boot_time: true,
            operational_mode: CacheOperationalMode::ValuesWithMemoryAddress,
        };
        let result: CacheConfiguration = data.into();
        assert_eq!(sample, result);
    }
    #[test]
    fn cache_size() {
        let data = [ 0b0000_0010_1010_1010, 0b1000_0010_1010_1010 ];
        let cs_1k = CacheSize::from(data[0]);
        let cs_64k = CacheSize::from(data[1]);
        let cs2_1k = CacheSize2::from((data[0] as u32) << 16);
        let cs2_64k = CacheSize2::from((data[1] as u32) << 16);
        assert_eq!(CacheSize::Granularity1K(682), cs_1k);
        assert_eq!(682 * 1024, cs_1k.bytes());
        assert_eq!(CacheSize::Granularity64K(682), cs_64k);
        assert_eq!(682 * 65536, cs_64k.bytes());
        assert_eq!(CacheSize2::Granularity1K(44695552), cs2_1k);
        assert_eq!(44695552 * 1024, cs2_1k.bytes());
        assert_eq!(CacheSize2::Granularity64K(44695552), cs2_64k);
        assert_eq!(44695552 * 65536, cs2_64k.bytes());
    }
    #[test]
    fn cache_enums() {
        let data = 0b0101_0101;
        let sram = CacheSramType::from_bits_truncate(data);
        assert!(sram.contains(CacheSramType::OTHER));
        assert!(sram.contains(CacheSramType::NONBURST));
        assert!(sram.contains(CacheSramType::PIPELINE_BURST));
        assert!(sram.contains(CacheSramType::ASYNCHRONOUS));
        assert_eq!(CacheErrorCorrectionType::Undefined(85), (data as u8).into());
        assert_eq!(CacheErrorCorrectionType::SingleBitEcc, ((data & 0b111) as u8).into());
        assert_eq!(SystemCacheType::Undefined(85), (data as u8).into());
        assert_eq!(SystemCacheType::Unified, ((data & 0b111) as u8).into());
        assert_eq!(CacheAssociativity::Undefined(85), (data as u8).into());
        assert_eq!(CacheAssociativity::FourWaySetAssociative, ((data & 0b1111) as u8).into());
    }
}
