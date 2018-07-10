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

        if structure.version.major == 2 && structure.version.minor < 1 {
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
        } else if structure.version.major == 2 && structure.version.minor < 3 {
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
        } else if structure.version.major == 2 && structure.version.minor < 5 {
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
        } else if structure.version.major == 2 && structure.version.minor < 6 {
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
        } else if structure.version.major < 3 {
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
