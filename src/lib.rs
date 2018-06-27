#[macro_use]
extern crate bitflags;
extern crate core;
extern crate failure;
#[macro_use]
extern crate failure_derive;

use core::mem;
use core::str;

#[repr(C)]
#[repr(packed)]
pub struct Entry {
    pub signature: u32,
    pub checksum: u8,
    pub len: u8,
    pub major: u8,
    pub minor: u8,
    pub max: u16,
    pub revision: u8,
    pub formatted: [u8; 5],
    pub dmi_signature: [u8; 5],
    pub dmi_checksum: u8,
    pub smbios_len: u16,
    pub smbios_address: u32,
    pub smbios_count: u16,
    pub bcd_revision: u8,
}

#[derive(Debug, Fail)]
pub enum InvalidEntryError {
    #[fail(display = "Input did not contain a valid SMBIOS entry")]
    NotFound,
    #[fail(display = "Input version number was below 2.0: {}", _0)]
    TooOldVersion(u8),
    #[fail(display = "Input contained an invalid-sized SMBIOS entry: {}", _0)]
    BadSize(u8),
    #[fail(display = "SMBIOS entry has an invalid checksum: {}", _0)]
    BadChecksum(u8),
}

fn find_signature(buffer: &[u8]) -> Option<usize> {
    static STRIDE: usize = 16;
    static SIG: &[u8; 4] = &[0x5f, 0x53, 0x4d, 0x5f];
    for (idx, chunk) in buffer.chunks(STRIDE).enumerate() {
        if chunk.starts_with(SIG) {
            return Some(idx * STRIDE);
        }
    }

    None
}

macro_rules! lib_ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e);
        }
    };
}

impl Entry {
    pub fn new(buffer: &[u8]) -> Result<Entry, InvalidEntryError> {
        find_signature(buffer)
            .ok_or_else(|| InvalidEntryError::NotFound)
            .and_then(|start| {
                let sub_buffer = &buffer[start..];
                lib_ensure!(
                    sub_buffer.len() >= mem::size_of::<Entry>(),
                    InvalidEntryError::BadSize(sub_buffer.len() as u8)
                );

                let entry: Entry = unsafe { std::ptr::read(sub_buffer.as_ptr() as *const _) };
                lib_ensure!(
                    entry.len as usize >= mem::size_of::<Entry>(),
                    InvalidEntryError::BadSize(entry.len)
                );

                lib_ensure!(
                    entry.major >= 2,
                    InvalidEntryError::TooOldVersion(entry.major)
                );

                lib_ensure!(
                    sub_buffer.len() as u8 >= entry.len,
                    InvalidEntryError::BadSize(sub_buffer.len() as u8)
                );

                let mut sum = 0u8;
                for val in &sub_buffer[0..(entry.len as usize)] {
                    sum = sum.wrapping_add(*val);
                }
                lib_ensure!(sum == 0, InvalidEntryError::BadChecksum(sum));

                Ok(entry)
            })
    }

    pub fn structures<'a, 'b>(&'a self, buffer: &'b [u8]) -> Structures<'a, 'b> {
        Structures {
            entry: self,
            count: 0,
            idx: 0u16,
            buffer: buffer,
        }
    }
}

pub struct Structures<'a, 'b> {
    entry: &'a Entry,
    count: u16,
    idx: u16,
    buffer: &'b [u8],
}

#[derive(Debug, Fail)]
pub enum InvalidStructureError {
    #[fail(display = "Structure at offset {} with length {} extends beyond SMBIOS", _0, _1)]
    BadSize(u16, u8),
    #[fail(display = "Structure at offset {} with unterminated strings", _0)]
    UnterminatedStrings(u16),
}

/// Finds the final nul nul terminator of a buffer and returns the index of the final nul
fn find_nulnul(buf: &[u8]) -> Option<usize> {
    for i in 0..buf.len() {
        if i + 1 >= buf.len() {
            return None;
        }

        if buf[i] == 0 && buf[i + 1] == 0 {
            return Some(i + 1);
        }
    }

    None
}

impl<'entry, 'b> Iterator for Structures<'entry, 'b> {
    type Item = Result<Structure<'b, 'entry>, InvalidStructureError>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.idx + mem::size_of::<HeaderPacked>() as u16) > self.entry.smbios_len
            || self.count >= self.entry.smbios_count
        {
            return None;
        }

        let working = &self.buffer[(self.idx as usize)..];
        let header: HeaderPacked = unsafe { std::ptr::read(working.as_ptr() as *const _) };

        let strings_idx: u16 = self.idx + header.len as u16;
        if strings_idx >= self.entry.smbios_len {
            return Some(Err(InvalidStructureError::BadSize(self.idx, header.len)));
        }

        let term = find_nulnul(&self.buffer[(strings_idx as usize)..]);
        let strings_len = match term {
            Some(terminator) => (terminator + 1) as u16,
            None => {
                return Some(Err(InvalidStructureError::UnterminatedStrings(self.idx)));
            }
        };

        let structure = Structure {
            info: header._type.into(),
            handle: header.handle,
            entry: self.entry,
            data: &self.buffer
                [(self.idx + mem::size_of::<HeaderPacked>() as u16) as usize..strings_idx as usize],
            strings: &self.buffer[strings_idx as usize..(strings_idx + strings_len) as usize],
        };

        self.idx = strings_idx + strings_len;
        self.count += 1;

        Some(Ok(structure))
    }
}

#[repr(C)]
#[repr(packed)]
pub struct HeaderPacked {
    pub _type: u8,
    pub len: u8,
    pub handle: u16,
}

pub struct Structure<'a, 'entry> {
    pub info: InfoType,
    pub handle: u16,
    entry: &'entry Entry,
    data: &'a [u8],
    strings: &'a [u8],
}

#[derive(Debug, Fail)]
pub enum MalformedStructureError {
    #[fail(display = "Structure {:?} with handle {} cannot be decoded to {}", _0, _1, _2)]
    BadType(InfoType, u16, &'static str),
    #[fail(display = "Structure {:?} with handle {} has invalid string index {}", _0, _1, _2)]
    InvalidStringIndex(InfoType, u16, u8),
}

macro_rules! let_as_struct {
    ($name:ident, $ty:ty, $data:expr) => {
        let $name: $ty = unsafe { std::ptr::read($data.as_ptr() as * const _) };
    };
}

impl<'a, 'entry> Structure<'a, 'entry> {
    fn strings(&self) -> impl Iterator<Item = &'a str> {
        self.strings.split(|elm| *elm == 0).filter_map(|slice| {
            if slice.is_empty() {
                None
            } else {
                unsafe { Some(str::from_utf8_unchecked(slice)) }
            }
        })
    }

    fn find_string(&self, idx: u8) -> Result<&'a str, MalformedStructureError> {
        self.strings()
            .nth((idx - 1) as usize)
            .ok_or_else(|| MalformedStructureError::InvalidStringIndex(self.info, self.handle, idx))
    }

    pub fn system(&self) -> Result<System<'a>, MalformedStructureError> {
        lib_ensure!(
            self.info == InfoType::System,
            MalformedStructureError::BadType(self.info, self.handle, "System")
        );

        #[repr(C)]
        #[repr(packed)]
        struct SystemPacked_2_0 {
            manufacturer: u8,
            product: u8,
            version: u8,
            serial: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct SystemPacked_2_1 {
            v2_0: SystemPacked_2_0,
            uuid: [u8; 16],
            wakeup: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct SystemPacked_2_4 {
            v2_1: SystemPacked_2_1,
            sku: u8,
            family: u8,
        }

        if self.entry.major == 2 && self.entry.minor < 1 {
            let_as_struct!(packed, SystemPacked_2_0, self.data);

            Ok(System {
                manufacturer: self.find_string(packed.manufacturer)?,
                product: self.find_string(packed.product)?,
                version: self.find_string(packed.version)?,
                serial: self.find_string(packed.serial)?,
                uuid: None,
                wakeup: None,
                sku: None,
                family: None,
            })

        } else if self.entry.major == 2 && self.entry.minor < 4 {
            let_as_struct!(packed, SystemPacked_2_1, self.data);

            Ok(System {
                manufacturer: self.find_string(packed.v2_0.manufacturer)?,
                product: self.find_string(packed.v2_0.product)?,
                version: self.find_string(packed.v2_0.version)?,
                serial: self.find_string(packed.v2_0.serial)?,
                uuid: Some(packed.uuid),
                wakeup: Some(packed.wakeup.into()),
                sku: None,
                family: None,
            })

        } else {
            let_as_struct!(packed, SystemPacked_2_4, self.data);

            Ok(System {
                manufacturer: self.find_string(packed.v2_1.v2_0.manufacturer)?,
                product: self.find_string(packed.v2_1.v2_0.product)?,
                version: self.find_string(packed.v2_1.v2_0.version)?,
                serial: self.find_string(packed.v2_1.v2_0.serial)?,
                uuid: Some(packed.v2_1.uuid),
                wakeup: Some(packed.v2_1.wakeup.into()),
                sku: Some(self.find_string(packed.sku)?),
                family: Some(self.find_string(packed.family)?),
            })
        }
    }

    pub fn base_board(&self) -> Result<BaseBoard<'a>, MalformedStructureError> {
        lib_ensure!(
            self.info == InfoType::BaseBoard,
            MalformedStructureError::BadType(self.info, self.handle, "BaseBoard")
        );

        #[repr(C)]
        #[repr(packed)]
        struct BaseBoardPacked {
            manufacturer: u8,
            product: u8,
            version: u8,
            serial: u8,
            asset: u8,
            feature_flags: u8,
            location_in_chassis: u8,
            chassis_handle: u16,
            board_type: u8,
        }

        let_as_struct!(packed, BaseBoardPacked, self.data);

        Ok(BaseBoard {
            manufacturer: self.find_string(packed.manufacturer)?,
            product: self.find_string(packed.product)?,
            version: self.find_string(packed.version)?,
            serial: self.find_string(packed.serial)?,
            asset: self.find_string(packed.asset)?,
            feature_flags: BaseBoardFlags::from_bits_truncate(packed.feature_flags),
            location_in_chassis: self.find_string(packed.location_in_chassis)?,
            chassis_handle: packed.chassis_handle,
            board_type: packed.board_type.into(),
        })
    }

    pub fn processor(&self) -> Result<Processor<'a>, MalformedStructureError> {
        lib_ensure!(
            self.info == InfoType::Processor,
            MalformedStructureError::BadType(self.info, self.handle, "Processor")
        );

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

        if self.entry.major == 2 && self.entry.minor < 1 {
            let_as_struct!(packed, ProcessorPacked_2_0, self.data);

            Ok(Processor {
                socket_designation: self.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family as u16,
                processor_manufacturer: self.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: self.find_string(packed.processor_version)?,
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
        } else if self.entry.major == 2 && self.entry.minor < 3 {
            let_as_struct!(packed, ProcessorPacked_2_1, self.data);

            Ok(Processor {
                socket_designation: self.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family as u16,
                processor_manufacturer: self.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: self.find_string(packed.processor_version)?,
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
        } else if self.entry.major == 2 && self.entry.minor < 5 {
            let_as_struct!(packed, ProcessorPacked_2_3, self.data);

            Ok(Processor {
                socket_designation: self.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family as u16,
                processor_manufacturer: self.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: self.find_string(packed.processor_version)?,
                voltage: packed.voltage,
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(self.find_string(packed.serial_number)?),
                asset_tag: Some(self.find_string(packed.asset_tag)?),
                part_number: Some(self.find_string(packed.part_number)?),
                core_count: None,
                core_enabled: None,
                thread_count: None,
                processor_characteristics: None,
            })
        } else if self.entry.major == 2 && self.entry.minor < 6 {
            let_as_struct!(packed, ProcessorPacked_2_5, self.data);

            Ok(Processor {
                socket_designation: self.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family as u16,
                processor_manufacturer: self.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: self.find_string(packed.processor_version)?,
                voltage: packed.voltage,
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(self.find_string(packed.serial_number)?),
                asset_tag: Some(self.find_string(packed.asset_tag)?),
                part_number: Some(self.find_string(packed.part_number)?),
                core_count: Some(packed.core_count as u16),
                core_enabled: Some(packed.core_enabled as u16),
                thread_count: Some(packed.thread_count as u16),
                processor_characteristics: None,
            })
        } else if self.entry.major < 3 {
            let_as_struct!(packed, ProcessorPacked_2_6, self.data);

            Ok(Processor {
                socket_designation: self.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family_2,
                processor_manufacturer: self.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: self.find_string(packed.processor_version)?,
                voltage: packed.voltage,
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(self.find_string(packed.serial_number)?),
                asset_tag: Some(self.find_string(packed.asset_tag)?),
                part_number: Some(self.find_string(packed.part_number)?),
                core_count: Some(packed.core_count as u16),
                core_enabled: Some(packed.core_enabled as u16),
                thread_count: Some(packed.thread_count as u16),
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(packed.processor_characteristics)),
            })
        } else {
            let_as_struct!(packed, ProcessorPacked_3_0, self.data);

            Ok(Processor {
                socket_designation: self.find_string(packed.socket_designation)?,
                processor_type: packed.processor_type.into(),
                processor_family: packed.processor_family_2,
                processor_manufacturer: self.find_string(packed.processor_manufacturer)?,
                processor_id: packed.processor_id,
                processor_version: self.find_string(packed.processor_version)?,
                voltage: packed.voltage,
                external_clock: packed.external_clock,
                max_speed: packed.max_speed,
                current_speed: packed.current_speed,
                status: ProcessorStatus::from_bits_truncate(packed.status),
                processor_upgrade: packed.processor_upgrade.into(),
                l1_cache_handle: Some(packed.l1_cache_handle),
                l2_cache_handle: Some(packed.l2_cache_handle),
                l3_cache_handle: Some(packed.l3_cache_handle),
                serial_number: Some(self.find_string(packed.serial_number)?),
                asset_tag: Some(self.find_string(packed.asset_tag)?),
                part_number: Some(self.find_string(packed.part_number)?),
                core_count: Some(packed.core_count_2),
                core_enabled: Some(packed.core_enabled_2),
                thread_count: Some(packed.thread_count_2),
                processor_characteristics: Some(ProcessorCharacteristics::from_bits_truncate(packed.processor_characteristics)),
            })
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WakeupType {
    Reserved,
    Other,
    Unknown,
    APM_Timer,
    Modem_Ring,
    LAN_Remote,
    Power_Switch,
    PCI_PME,
    AC_Power_Restored,
    Undefined(u8),
}

impl From<u8> for WakeupType {
    fn from(_type: u8) -> WakeupType {
        match _type {
            0 => WakeupType::Reserved,
            1 => WakeupType::Other,
            2 => WakeupType::Unknown,
            3 => WakeupType::APM_Timer,
            4 => WakeupType::Modem_Ring,
            5 => WakeupType::LAN_Remote,
            6 => WakeupType::Power_Switch,
            7 => WakeupType::PCI_PME,
            8 => WakeupType::AC_Power_Restored,
            t => WakeupType::Undefined(t),
        }
    }
}

#[derive(Debug)]
pub struct System<'a> {
    pub manufacturer: &'a str,
    pub product: &'a str,
    pub version: &'a str,
    pub serial: &'a str,
    pub uuid: Option<[u8; 16]>,
    pub wakeup: Option<WakeupType>,
    pub sku: Option<&'a str>,
    pub family: Option<&'a str>,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BoardType {
    Unknown,
    Other,
    ServerBlade,
    ConnectivitySwitch,
    SystemManagementModule,
    ProcessorModule,
    IoModule,
    MemoryModule,
    DaughterBoard,
    MotherBoard,
    ProcessorMemoryModule,
    ProcessorIoModule,
    InterconnectBoard,
    Undefined(u8),
}

impl From<u8> for BoardType {
    fn from(_type: u8) -> BoardType {
        match _type {
            1 => BoardType::Unknown,
            2 => BoardType::Other,
            3 => BoardType::ServerBlade,
            4 => BoardType::ConnectivitySwitch,
            5 => BoardType::SystemManagementModule,
            6 => BoardType::ProcessorModule,
            7 => BoardType::IoModule,
            8 => BoardType::MemoryModule,
            9 => BoardType::DaughterBoard,
           10 => BoardType::MotherBoard,
           11 => BoardType::ProcessorMemoryModule,
           12 => BoardType::ProcessorIoModule,
           13 => BoardType::InterconnectBoard,
            t => BoardType::Undefined(t),
        }
    }
}

bitflags! {
    pub struct BaseBoardFlags: u8 {
        const HOSTING = 0b0000_0001;
        const REQUIRES_DAUGHTER = 0b0000_0010;
        const IS_REMOVABLE = 0b0000_0100;
        const IS_REPLACEABLE = 0b0000_1000;
        const IS_HOT_SWAPPABLE = 0b0001_0000;
    }
}

#[derive(Debug)]
pub struct BaseBoard<'a> {
    pub manufacturer: &'a str,
    pub product: &'a str,
    pub version: &'a str,
    pub serial: &'a str,
    pub asset: &'a str,
    pub feature_flags: BaseBoardFlags,
    pub location_in_chassis: &'a str,
    pub chassis_handle: u16,
    pub board_type: BoardType,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Debug)]
pub struct Processor<'a> {
    pub socket_designation: &'a str,
    pub processor_type: ProcessorType,
    pub processor_family: u16,
    pub processor_manufacturer: &'a str,
    pub processor_id: u64,
    pub processor_version: &'a str,
    pub voltage: u8,
    pub external_clock: u16,
    pub max_speed: u16,
    pub current_speed: u16,
    pub status: ProcessorStatus,
    pub processor_upgrade: u8,
    pub l1_cache_handle: Option<u16>,
    pub l2_cache_handle: Option<u16>,
    pub l3_cache_handle: Option<u16>,
    pub serial_number: Option<&'a str>,
    pub asset_tag: Option<&'a str>,
    pub part_number: Option<&'a str>,
    pub core_count: Option<u16>,
    pub core_enabled: Option<u16>,
    pub thread_count: Option<u16>,
    pub processor_characteristics: Option<ProcessorCharacteristics>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InfoType {
    Bios,
    System,
    BaseBoard,
    Enclosure,
    Processor,
    Cache,
    SystemSlots,
    PhysicalMemoryArray,
    MemoryDevice,
    MemoryArrayMappedAddress,
    MemoryDeviceMappedAddress,
    SystemBoot,
    Oem(u8),
    End,
}

impl From<u8> for InfoType {
    fn from(_type: u8) -> InfoType {
        match _type {
            0 => InfoType::Bios,
            1 => InfoType::System,
            2 => InfoType::BaseBoard,
            3 => InfoType::Enclosure,
            4 => InfoType::Processor,
            7 => InfoType::Cache,
            9 => InfoType::SystemSlots,
            16 => InfoType::PhysicalMemoryArray,
            17 => InfoType::MemoryDevice,
            19 => InfoType::MemoryArrayMappedAddress,
            20 => InfoType::MemoryDeviceMappedAddress,
            32 => InfoType::SystemBoot,
            127 => InfoType::End,
            t => InfoType::Oem(t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DMIDECODE_BIN: &'static [u8] = include_bytes!("./dmidecode.bin");
    const ENTRY_BIN: &'static [u8] = include_bytes!("./entry.bin");
    const DMI_BIN: &'static [u8] = include_bytes!("./dmi.bin");

    #[test]
    fn found_smbios_entry() {
        Entry::new(ENTRY_BIN).unwrap();
        Entry::new(DMIDECODE_BIN).unwrap();
    }

    #[test]
    #[should_panic]
    fn doesnt_find_smbios_entry() {
        Entry::new(DMI_BIN).unwrap();
    }

    #[test]
    fn found_signature() {
        find_signature(ENTRY_BIN).unwrap();
        find_signature(DMIDECODE_BIN).unwrap();
    }

    #[test]
    #[should_panic]
    fn doesnt_find_signature() {
        find_signature(DMI_BIN).unwrap();
    }

    #[test]
    fn iterator_through_structures() {
        let entry = Entry::new(DMIDECODE_BIN).unwrap();
        for s in entry.structures(&DMIDECODE_BIN[(entry.smbios_address as usize)..]) {
            s.unwrap();
        }
    }

    #[test]
    fn iterator_through_structures_baseboard() {
        let entry = Entry::new(DMIDECODE_BIN).unwrap();
        let structures = entry
            .structures(&DMIDECODE_BIN[(entry.smbios_address as usize)..])
            .filter_map(|s| s.ok());
        for s in structures.filter(|s| s.info == InfoType::BaseBoard) {
            println!("{:?}", s.base_board().unwrap());
        }
    }

    #[test]
    fn iterator_through_structures_system() {
        let entry = Entry::new(DMIDECODE_BIN).unwrap();
        let structures = entry
            .structures(&DMIDECODE_BIN[(entry.smbios_address as usize)..])
            .filter_map(|s| s.ok());
        for s in structures.filter(|s| s.info == InfoType::System) {
            println!("{:?}", s.system().unwrap());
        }
    }

    #[test]
    fn iterator_through_structures_processor() {
        let entry = Entry::new(DMIDECODE_BIN).unwrap();
        let structures = entry
            .structures(&DMIDECODE_BIN[(entry.smbios_address as usize)..])
            .filter_map(|s| s.ok());
        for s in structures.filter(|s| s.info == InfoType::Processor) {
            println!("{:?}", s.processor().unwrap());
        }
    }

    #[test]
    fn find_nulnul_empty() {
        let buf = vec![];
        assert_eq!(find_nulnul(&buf), None);
    }

    #[test]
    fn find_nulnul_single_char() {
        let buf = vec![0];
        assert_eq!(find_nulnul(&buf), None);
    }

    #[test]
    fn find_nulnul_trivial() {
        let buf = vec![0, 0];
        assert_eq!(find_nulnul(&buf), Some(1));
    }

    #[test]
    fn find_nulnul_with_data() {
        let buf = vec![1, 2, 3, 4, 0, 5, 4, 3, 2, 1, 0, 0];
        assert_eq!(find_nulnul(&buf), Some(11));
    }

    #[test]
    fn find_nulnul_with_data_more_at_end() {
        let buf = vec![1, 2, 3, 4, 0, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3];
        assert_eq!(find_nulnul(&buf), Some(11));
    }
}
