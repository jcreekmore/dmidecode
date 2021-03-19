//! # DMIDECODE
//!
//! This library reports information about system's hardware as described in system BIOS according
//! to the SMBIOS/DMI standard. Each SMBIOS type refers to separate struct.
//!
//! SMBIOS specification defines the following data structures:
//! - [BIOS Information](bios::Bios "bios::Bios") (Type 0)
//! - [System Information](system::System "system::System") (Type 1)
//! - [Baseboard (or Module) Information](baseboard::BaseBoard "baseboard::BaseBoard") (Type 2)
//! - [System Enclosure or Chassis](enclosure::Enclosure "enclosure::Enclosure") (Type 3)
//! - [Processor Information](processor::Processor "processor::Processor") (Type 4)
//! - Memory Controller Information (Type 5, Obsolete)
//! - Memory Module Information (Type 6, Obsolete)
//! - [Cache Information](processor::Cache "processor::Cache") (Type 7)
//! - [Port Connector Information](connector::PortConnector "connector::PortConnector") (Type 8)
//! - [System Slots](slot::SystemSlots "slot::SystemSlots") (Type 9)
//! - On Board Devices Information (Type 10, Obsolete)
//! - OEM Strings (Type 11)
//! - System Configuration Options (Type 12)
//! - [BIOS Language Information](bios::BiosLanguage "bios::BiosLanguage") (Type 13)
//! - Group Associations (Type 14)
//! - System Event Log (Type 15)
//! - [Physical Memory Array](memory::PhysicalMemoryArray "memory::PhysicalMemoryArray") (Type 16)
//! - [Memory Device](memory::MemoryDevice "memory::MemoryDevice") (Type 17)
//! - 32-Bit Memory Error Information (Type 18)
//! - Memory Array Mapped Address (Type 19)
//! - Memory Device Mapped Address (Type 20)
//! - Built-in Pointing Device (Type 21)
//! - Portable Battery (Type 22)
//! - System Reset (Type 23)
//! - Hardware Security (Type 24)
//! - System Power Controls (Type 25)
//! - Voltage Probe (Type 26)
//! - Cooling Device (Type 27)
//! - Temperature Probe (Type 28)
//! - Electrical Current Probe (Type 29)
//! - Out-of-Band Remote Access (Type 30)
//! - Boot Integrity Services (BIS) Entry Point (Type 31)
//! - System Boot Information (Type 32)
//! - 64-Bit Memory Error Information (Type 33)
//! - Management Device (Type 34)
//! - Management Device Component (Type 35)
//! - Management Device Threshold Data (Type 36)
//! - Memory Channel (Type 37)
//! - IPMI Device Information (Type 38)
//! - System Power Supply (Type 39)
//! - Additional Information (Type 40)
//! - Onboard Devices Extended Information (Type 41)
//! - Management Controller Host Interface (Type 42)
//! - TPM Device (Type 43)
//! - Processor Additional Information (Type 44)
//! - Inactive (Type 126)
//! - End-of-Table (Type 127)


#![no_std]

extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate bitflags;
#[cfg(test)]
extern crate lazy_static;
#[cfg(test)]
extern crate pretty_assertions;

use core::mem;
use core::str;
use core::fmt;
use core::convert::TryInto;
use core::array::TryFromSliceError;

#[macro_export]
#[doc(hidden)]
macro_rules! let_as_struct {
    ($name:ident, $ty:ty, $data:expr) => {
        use core::ptr;
        let $name: $ty = unsafe { ptr::read($data.as_ptr() as *const _) };
    };
}

#[doc(hidden)]
macro_rules! lib_ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e);
        }
    };
}

#[macro_use]
pub mod bitfield;

pub mod bios;
pub use bios::Bios;
pub use bios::BiosLanguage;

pub mod memory;
pub use memory::MemoryDevice;
pub use memory::PhysicalMemoryArray;

pub mod system;
pub use system::System;

pub mod baseboard;
pub use baseboard::BaseBoard;

pub mod enclosure;
pub use enclosure::Enclosure;

pub mod processor;
pub use processor::Processor;
pub use processor::Cache;

pub mod connector;
pub use connector::PortConnector;

pub mod slot;
pub use slot::SystemSlots;

enum EntryPointFormat {
    V2,
    V3,
}

pub enum EntryPoint {
    V2(EntryPointV2),
    V3(EntryPointV3),
}

impl EntryPoint {
    pub fn len(&self) -> u8 {
        match self {
            EntryPoint::V2(point) => point.len,
            EntryPoint::V3(point) => point.len,
        }
    }
    pub fn major(&self) -> u8 {
        match self {
            EntryPoint::V2(point) => point.major,
            EntryPoint::V3(point) => point.major,
        }
    }
    pub fn minor(&self) -> u8 {
        match self {
            EntryPoint::V2(point) => point.minor,
            EntryPoint::V3(point) => point.minor,
        }
    }
    pub fn revision(&self) -> u8 {
        match self {
            EntryPoint::V2(point) => point.revision,
            EntryPoint::V3(point) => point.revision,
        }
    }
    pub fn smbios_address(&self) -> u64 {
        match self {
            EntryPoint::V2(point) => point.smbios_address as u64,
            EntryPoint::V3(point) => point.smbios_address,
        }
    }
    pub fn smbios_len(&self) -> u32 {
        match self {
            EntryPoint::V2(point) => point.smbios_len as u32,
            EntryPoint::V3(point) => point.smbios_len_max,
        }
    }
    pub fn to_version(&self) -> SmbiosVersion {
        SmbiosVersion {
            major: self.major(),
            minor: self.minor(),
        }
    }

    /// Create an iterator across the SMBIOS structures found in `buffer`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate failure;
    /// # extern crate dmidecode;
    /// # use failure::Error;
    /// use dmidecode::EntryPoint;
    /// # fn try_main() -> Result<(), Error> {
    /// #
    /// const DMIDECODE_BIN: &'static [u8] = include_bytes!("../tests/data/dmidecode.bin");
    ///
    /// let entry_point = EntryPoint::search(DMIDECODE_BIN)?;
    /// for s in entry_point.structures(&DMIDECODE_BIN[entry_point.smbios_address() as usize..]) {
    ///   let table = s?;
    /// }
    /// Ok(())
    /// # }
    /// ```
    pub fn structures<'buffer>(&self, buffer: &'buffer [u8]) -> Structures<'buffer> {
        Structures {
            smbios_version: self.to_version(),
            smbios_len: self.smbios_len(),
            idx: 0u32,
            buffer: buffer,
        }
    }

    /// Search for an instance of an SMBIOS `EntryPoint` in a memory `buffer`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate dmidecode;
    /// use dmidecode::EntryPoint;
    ///
    /// const ENTRY_BIN: &'static [u8] = include_bytes!("../tests/data/entry.bin");
    ///
    /// let entry_point = EntryPoint::search(ENTRY_BIN);
    /// ```
    ///
    /// # Errors
    ///
    /// If this function fails to find a valid SMBIOS `EntryPoint`, it will return
    /// an `InvalidEntryPointError` variant.
    pub fn search(buffer: &[u8]) -> Result<EntryPoint, InvalidEntryPointError> {
        find_signature(buffer)
            .ok_or_else(|| InvalidEntryPointError::NotFound)
            .and_then(|(kind, start)| {
                let sub_buffer = &buffer[start..];

                let entry_point = match kind {
                    EntryPointFormat::V2 => {
                        lib_ensure!(
                            sub_buffer.len() >= mem::size_of::<EntryPointV2>(),
                            InvalidEntryPointError::BadSize(sub_buffer.len() as u8)
                        );
                        let_as_struct!(entry_point, EntryPointV2, sub_buffer);
                        lib_ensure!(
                            entry_point.len as usize >= mem::size_of::<EntryPointV2>(),
                            InvalidEntryPointError::BadSize(entry_point.len)
                        );
                        EntryPoint::V2(entry_point)
                    }
                    EntryPointFormat::V3 => {
                        lib_ensure!(
                            sub_buffer.len() >= mem::size_of::<EntryPointV3>(),
                            InvalidEntryPointError::BadSize(sub_buffer.len() as u8)
                        );
                        let_as_struct!(entry_point, EntryPointV3, sub_buffer);
                        lib_ensure!(
                            entry_point.len as usize >= mem::size_of::<EntryPointV3>(),
                            InvalidEntryPointError::BadSize(entry_point.len)
                        );
                        EntryPoint::V3(entry_point)
                    }
                };

                lib_ensure!(
                    entry_point.major() >= 2,
                    InvalidEntryPointError::TooOldVersion(entry_point.major())
                );

                lib_ensure!(
                    sub_buffer.len() as u8 >= entry_point.len(),
                    InvalidEntryPointError::BadSize(sub_buffer.len() as u8)
                );

                let mut sum = 0u8;
                for val in &sub_buffer[0..(entry_point.len() as usize)] {
                    sum = sum.wrapping_add(*val);
                }
                lib_ensure!(sum == 0, InvalidEntryPointError::BadChecksum(sum));

                Ok(entry_point)
            })
    }
}


///
/// An SMBIOSv2 `EntryPoint` structure.
///
/// The SMBIOS `EntryPoint` structure is used to verify that a set of SMBIOS tables exist
/// in memory and what version of the SMBIOS specification should be used to
/// access the tables.
///
#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EntryPointV2 {
    pub signature: u32,
    pub checksum: u8,
    pub len: u8,
    pub major: u8,
    pub minor: u8,
    pub struct_max: u16,
    pub revision: u8,
    pub formatted: [u8; 5],
    pub dmi_signature: [u8; 5],
    pub dmi_checksum: u8,
    pub smbios_len: u16,
    pub smbios_address: u32,
    pub smbios_count: u16,
    pub bcd_revision: u8,
}

///
/// An SMBIOSv3 `EntryPoint` structure.
///
#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EntryPointV3 {
    pub signature: [u8; 5],
    pub checksum: u8,
    pub len: u8,
    pub major: u8,
    pub minor: u8,
    pub docrev: u8,
    pub revision: u8,
    _reserved: u8,
    pub smbios_len_max: u32,
    pub smbios_address: u64,
}

/// The version number associated with the Smbios `EntryPoint`
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SmbiosVersion {
    pub major: u8,
    pub minor: u8,
}

impl From<(usize, usize)> for SmbiosVersion {
    fn from(other: (usize, usize)) -> SmbiosVersion {
        SmbiosVersion {
            major: other.0 as u8,
            minor: other.1 as u8,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct SmbiosBound {
    len: u16,
    count: u16,
}

/// Failure type for trying to find the SMBIOS `EntryPoint` structure in memory.
#[derive(Debug, Fail)]
pub enum InvalidEntryPointError {
    /// The SMBIOS `EntryPoint` structure was not found in the memory buffer.
    #[fail(display = "Input did not contain a valid SMBIOS entry point")]
    NotFound,
    /// The SMBIOS `EntryPoint` structure was versioned before 2.0.
    #[fail(display = "Input version number was below 2.0: {}", _0)]
    TooOldVersion(u8),
    /// The SMBIOS `EntryPoint` structure was smaller than the size of the SMBIOS 2.1 structure.
    #[fail(display = "Input contained an invalid-sized SMBIOS entry: {}", _0)]
    BadSize(u8),
    /// The SMBIOS `EntryPoint` structure had an invalid checksum.
    #[fail(display = "SMBIOS entry point has an invalid checksum: {}", _0)]
    BadChecksum(u8),
}

fn find_signature(buffer: &[u8]) -> Option<(EntryPointFormat, usize)> {
    static STRIDE: usize = 16;
    static V2_SIG: &[u8; 4] = &[0x5f, 0x53, 0x4d, 0x5f];
    static V3_SIG: &[u8; 5] = &[0x5f, 0x53, 0x4d, 0x33, 0x5f];

    for (idx, chunk) in buffer.chunks(STRIDE).enumerate() {
        if chunk.starts_with(V2_SIG) {
            return Some((EntryPointFormat::V2, idx * STRIDE));
        } else if chunk.starts_with(V3_SIG) {
            return Some((EntryPointFormat::V3, idx * STRIDE));
        }
    }

    None
}

/// An iterator that traverses the SMBIOS structure tables.
/// This struct is produced by the `structures` method on `EntryPoint`. See its documentation for more details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Structures<'buffer> {
    smbios_version: SmbiosVersion,
    smbios_len: u32,
    idx: u32,
    buffer: &'buffer [u8],
}

/// Variant structure for decoding the SMBIOS table types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Structure<'buffer> {
    Bios(Bios<'buffer>),
    System(System<'buffer>),
    BaseBoard(BaseBoard<'buffer>),
    Enclosure(Enclosure<'buffer>),
    Processor(Processor<'buffer>),
    Cache(Cache<'buffer>),
    PortConnector(PortConnector<'buffer>),
    SystemSlots(SystemSlots<'buffer>),
    BiosLanguage(BiosLanguage<'buffer>),
    MemoryDevice(MemoryDevice<'buffer>),
    PhysicalMemoryArray(PhysicalMemoryArray),
    Other(RawStructure<'buffer>),
}

/// Failure type for trying to decode the SMBIOS `Structures` iterator into the `Structure` variant type.
#[derive(Debug, Fail)]
pub enum MalformedStructureError {
    /// The SMBIOS structure exceeds the end of the memory buffer given to the `EntryPoint::structures` method.
    #[fail(
        display = "Structure at offset {} with length {} extends beyond SMBIOS",
        _0, _1
    )]
    BadSize(u32, u8),
    /// The SMBIOS structure contains an unterminated strings section.
    #[fail(display = "Structure at offset {} with unterminated strings", _0)]
    UnterminatedStrings(u32),
    /// The SMBIOS structure contains an invalid string index.
    #[fail(
        display = "Structure {:?} with handle {} has invalid string index {}",
        _0, _1, _2
    )]
    InvalidStringIndex(InfoType, u16, u8),
    /// This error returned when a conversion from a slice to an array fails.
    #[fail(display = "{}", _0)]
    InvalidSlice(#[fail(cause)] core::array::TryFromSliceError),
    /// The SMBIOS structure formatted section length does not correspond to SMBIOS reference
    /// specification
    #[fail(
        display = "Formatted section length of structure {:?} should be {} bytes",
        _0, _1
    )]
    InvalidFormattedSectionLength(InfoType, &'static str),
}

#[doc(hidden)]
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

impl<'buffer> Iterator for Structures<'buffer> {
    type Item = Result<Structure<'buffer>, MalformedStructureError>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.idx + mem::size_of::<HeaderPacked>() as u32) > self.smbios_len {
            return None;
        }

        let working = &self.buffer[(self.idx as usize)..];
        let_as_struct!(header, HeaderPacked, working);

        let strings_idx: u32 = self.idx + header.len as u32;
        if strings_idx >= self.smbios_len {
            return Some(Err(MalformedStructureError::BadSize(self.idx, header.len)));
        }

        let term = find_nulnul(&self.buffer[(strings_idx as usize)..]);
        let strings_len = match term {
            Some(terminator) => (terminator + 1) as u32,
            None => {
                return Some(Err(MalformedStructureError::UnterminatedStrings(self.idx)));
            }
        };

        let structure = RawStructure {
            version: self.smbios_version,
            info: header.kind.into(),
            handle: header.handle,
            data: &self.buffer
                [(self.idx + mem::size_of::<HeaderPacked>() as u32) as usize..strings_idx as usize],
            strings: &self.buffer[strings_idx as usize..(strings_idx + strings_len) as usize],
        };

        self.idx = strings_idx + strings_len;

        /*
         * For SMBIOS v3 we have no exact table length and no item count,
         * so stop at the end-of-table marker.
         */
        if self.smbios_version.major >= 3 && structure.info == InfoType::End {
            self.smbios_len = self.idx;
        }

        Some(match structure.info {
            InfoType::Bios => Bios::try_from(structure).map(Structure::Bios),
            InfoType::System => System::try_from(structure).map(Structure::System),
            InfoType::BaseBoard => BaseBoard::try_from(structure).map(Structure::BaseBoard),
            InfoType::Enclosure => Enclosure::try_from(structure).map(Structure::Enclosure),
            InfoType::Processor => Processor::try_from(structure).map(Structure::Processor),
            InfoType::Cache => Cache::try_from(structure).map(Structure::Cache),
            InfoType::PortConnector => PortConnector::try_from(structure).map(Structure::PortConnector),
            InfoType::SystemSlots => SystemSlots::try_from(structure).map(Structure::SystemSlots),
            InfoType::BiosLanguage => BiosLanguage::try_from(structure).map(Structure::BiosLanguage),
            InfoType::PhysicalMemoryArray => {
                PhysicalMemoryArray::try_from(structure).map(Structure::PhysicalMemoryArray)
            }
            InfoType::MemoryDevice => {
                MemoryDevice::try_from(structure).map(Structure::MemoryDevice)
            }
            _ => Ok(Structure::Other(structure)),
        })
    }
}

#[doc(hidden)]
#[repr(C)]
#[repr(packed)]
struct HeaderPacked {
    kind: u8,
    len: u8,
    handle: u16,
}

/// The raw SMBIOS structure information for structures that are not handled by this crate, such as Oem structures.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RawStructure<'buffer> {
    pub version: SmbiosVersion,
    pub info: InfoType,
    pub handle: u16,
    pub data: &'buffer [u8],
    strings: &'buffer [u8],
}

/// General trait for slice -> unsigned conversion
pub trait TryFromBytes<'a, T>: Sized {
    fn try_from_bytes(&'a [u8]) -> Result<Self, TryFromSliceError>;
}

impl<'a> TryFromBytes<'a, u8> for u8 {
    fn try_from_bytes(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        bytes.try_into()
            .map(|arr| u8::from_le_bytes(arr))
    }
}
impl<'a> TryFromBytes<'a, u16> for u16 {
    fn try_from_bytes(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        bytes.try_into()
            .map(|arr| u16::from_le_bytes(arr))
    }
}
impl<'a> TryFromBytes<'a, u32> for u32 {
    fn try_from_bytes(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        bytes.try_into()
            .map(|arr| u32::from_le_bytes(arr))
    }
}
impl<'a> TryFromBytes<'a, u64> for u64 {
    fn try_from_bytes(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        bytes.try_into()
            .map(|arr| u64::from_le_bytes(arr))
    }
}
impl<'a> TryFromBytes<'a, u128> for u128 {
    fn try_from_bytes(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        bytes.try_into()
            .map(|arr| u128::from_le_bytes(arr))
    }
}

impl<'buffer> RawStructure<'buffer> {
    /// Return an iterator over the strings in the strings table.
    fn strings(&self) -> impl Iterator<Item = &'buffer str> {
        self.strings.split(|elm| *elm == 0).filter_map(|slice| {
            if slice.is_empty() {
                None
            } else {
                unsafe { Some(str::from_utf8_unchecked(slice)) }
            }
        })
    }

    /// Find a string in the strings table by the string index.
    /// If the string index is 0, the empty string is returned. Otherwise, the string corresponding
    /// to that string index in the strings table is returned.
    ///
    /// # Errors
    /// Returns a `MalformedStructureError::InvalidStringIndex` if the index is outside of the strings table.
    pub fn find_string(&self, idx: u8) -> Result<&'buffer str, MalformedStructureError> {
        if idx == 0 {
            Ok("")
        } else {
            self.strings().nth((idx - 1) as usize).ok_or_else(|| {
                MalformedStructureError::InvalidStringIndex(self.info, self.handle, idx)
            })
        }
    }
    /// Get value by offset declared in SMBIOS Reference Specification.\
    /// Type meaning data length is mandatory:
    /// - *BYTE*: u8
    /// - *WORD*: u16
    /// - *DWORD*: u32
    /// - *QWORD*: u64
    ///
    /// The only error this method returned: [MalformedStructureError::InvalidSlice] (actually is
    /// [core::array::TryFromSliceError]). If getting value index exceedes length of *Formatted
    /// section* it may be ignored to return [None] value of structure field. In this case *Formatted
    /// section* length automatically hide non-existing values
    pub fn get<T: TryFromBytes<'buffer, T>>(&self, offset: usize) -> Result<T, MalformedStructureError> {
        // Ignore header
        let start = offset - 4;
        let size = core::mem::size_of::<T>();
        let slice = self.data.get(start..(start + size))
            .unwrap_or(&[]);
        TryFromBytes::try_from_bytes(slice)
            .map_err(|e| MalformedStructureError::InvalidSlice(e))
    }
    /// Wrapper to self.data.get(..) with header offset correction
    pub fn get_slice(&self, offset: usize, size: usize) -> Option<&'buffer [u8]> {
        self.data.get(offset - 4 .. offset - 4 + size)
    }
    /// Get *STRING* by offset declared in SMBIOS Reference Specification
    pub fn get_string(&self, offset: usize) -> Result<&'buffer str, MalformedStructureError> {
        self.get::<u8>(offset)
            .and_then(|idx| self.find_string(idx))
    }
}

/// SMBIOS Table information variant
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum InfoType {
    Bios,
    System,
    BaseBoard,
    Enclosure,
    Processor,
    Cache,
    PortConnector,
    SystemSlots,
    BiosLanguage,
    PhysicalMemoryArray,
    MemoryDevice,
    MemoryArrayMappedAddress,
    MemoryDeviceMappedAddress,
    SystemBoot,
    Oem(u8),
    End,
}

impl From<u8> for InfoType {
    fn from(kind: u8) -> InfoType {
        match kind {
            0 => InfoType::Bios,
            1 => InfoType::System,
            2 => InfoType::BaseBoard,
            3 => InfoType::Enclosure,
            4 => InfoType::Processor,
            7 => InfoType::Cache,
            8 => InfoType::PortConnector,
            9 => InfoType::SystemSlots,
            13 => InfoType::BiosLanguage,
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
impl fmt::Display for InfoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfoType::Bios                      => write!(f, "BIOS Information"),
            InfoType::System                    => write!(f, "System Information"),
            InfoType::BaseBoard                 => write!(f, "Baseboard (or Module) Information"),
            InfoType::Enclosure                 => write!(f, "System Enclosure or Chassis"),
            InfoType::Processor                 => write!(f, "Processor Information"),
            //InfoType::                          => write!(f, "Memory Controller Information"),
            //InfoType::                          => write!(f, "Memory Module Information"),
            InfoType::Cache                     => write!(f, "Cache Information"),
            InfoType::PortConnector             => write!(f, "Port Connector Information"),
            InfoType::SystemSlots               => write!(f, "System Slots"),
            //InfoType::                          => write!(f, "On Board Devices Information"),
            //InfoType::                          => write!(f, "OEM Strings"),
            //InfoType::                          => write!(f, "System Configuration Options"),
            InfoType::BiosLanguage              => write!(f, "BIOS Language Information"),
            //InfoType::                          => write!(f, "Group Associations"),
            //InfoType::                          => write!(f, "System Event Log"),
            InfoType::PhysicalMemoryArray       => write!(f, "Physical Memory Array"),
            InfoType::MemoryDevice              => write!(f, "Memory Device"),
            //InfoType::                          => write!(f, "32-Bit Memory Error Information"),
            InfoType::MemoryArrayMappedAddress  => write!(f, "Memory Array Mapped Address"),
            InfoType::MemoryDeviceMappedAddress => write!(f, "Memory Device Mapped Address"),
            //InfoType::                          => write!(f, "Built-in Pointing Device"),
            //InfoType::                          => write!(f, "Portable Battery"),
            //InfoType::                          => write!(f, "System Reset"),
            //InfoType::                          => write!(f, "Hardware Security"),
            //InfoType::                          => write!(f, "System Power Controls"),
            //InfoType::                          => write!(f, "Voltage Probe"),
            //InfoType::                          => write!(f, "Cooling Device"),
            //InfoType::                          => write!(f, "Temperature Probe"),
            //InfoType::                          => write!(f, "Electrical Current Probe"),
            //InfoType::                          => write!(f, "Out-of-Band Remote Access"),
            //InfoType::                          => write!(f, "Boot Integrity Services (BIS) Entry Point"),
            InfoType::SystemBoot                => write!(f, "System Boot Information"),
            //InfoType::                          => write!(f, "64-Bit Memory Error Information"),
            //InfoType::                          => write!(f, "Management Device"),
            //InfoType::                          => write!(f, "Management Device Component"),
            //InfoType::                          => write!(f, "Management Device Threshold Data"),
            //InfoType::                          => write!(f, "Memory Channel"),
            //InfoType::                          => write!(f, "IPMI Device Information"),
            //InfoType::                          => write!(f, "System Power Supply"),
            //InfoType::                          => write!(f, "Additional Information"),
            //InfoType::                          => write!(f, "Onboard Devices Extended Information"),
            //InfoType::                          => write!(f, "Management Controller Host Interface"),
            //InfoType::                          => write!(f, "TPM Device"),
            //InfoType::                          => write!(f, "Processor Additional Information"),
            //InfoType::                          => write!(f, "Inactive"),
            InfoType::End                       => write!(f, "End-of-Table"),
            InfoType::Oem(t)                    => write!(f, "OEM: {}", t),
        }
    }
}


#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;

    const DMIDECODE_BIN: &'static [u8] = include_bytes!("../tests/data/dmidecode.bin");
    const ENTRY_V2_BIN: &'static [u8] = include_bytes!("../tests/data/entry.bin");
    const DMI_V2_BIN: &'static [u8] = include_bytes!("../tests/data/dmi.bin");
    const ENTRY_V3_BIN: &'static [u8] = include_bytes!("../tests/data/entry_v3.bin");
    const DMI_V3_BIN: &'static [u8] = include_bytes!("../tests/data/dmi_v3.bin");
    const DMI_V3_SHORT: &'static [u8] = include_bytes!("../tests/data/dmi_v3_short.bin");
    const ENTRY_V3_SHORT: &'static [u8] = include_bytes!("../tests/data/entry_v3_short.bin");

    #[test]
    fn found_smbios_entry() {
        EntryPoint::search(ENTRY_V2_BIN).unwrap();
        EntryPoint::search(DMIDECODE_BIN).unwrap();
    }

    #[test]
    fn found_smbios_entry_v3() {
        EntryPoint::search(ENTRY_V3_BIN).unwrap();
    }

    #[test]
    #[should_panic]
    fn doesnt_find_smbios_entry() {
        EntryPoint::search(DMI_V2_BIN).unwrap();
    }

    #[test]
    fn found_signature() {
        find_signature(ENTRY_V2_BIN).unwrap();
        find_signature(ENTRY_V3_BIN).unwrap();
        find_signature(DMIDECODE_BIN).unwrap();
    }

    #[test]
    #[should_panic]
    fn doesnt_find_signature() {
        find_signature(DMI_V2_BIN).unwrap();
        find_signature(DMI_V3_BIN).unwrap();
    }

    #[test]
    fn iterator_through_structures() {
        let entry_point = EntryPoint::search(DMIDECODE_BIN).unwrap();
        for s in entry_point
            .structures(&DMIDECODE_BIN[(entry_point.smbios_address() as usize)..])
            .filter_map(|s| s.ok())
        {
            println!("{:?}", s);
        }
    }

    #[test]
    fn iterator_through_structures_v3_short() {
        let entry_point = EntryPoint::search(ENTRY_V3_SHORT).unwrap();
        for s in entry_point.structures(DMI_V3_SHORT).filter_map(|s| s.ok()) {
            println!("{:?}", s);
        }
    }

    #[test]
    fn iterator_through_structures_v3() {
        let entry_point = EntryPoint::search(ENTRY_V3_BIN).unwrap();
        for s in entry_point.structures(DMI_V3_BIN).filter_map(|s| s.ok()) {
            println!("{:?}", s);
        }
    }

    #[test]
    fn find_nulnul_empty() {
        let buf = [];
        assert_eq!(find_nulnul(&buf), None);
    }

    #[test]
    fn find_nulnul_single_char() {
        let buf = [0];
        assert_eq!(find_nulnul(&buf), None);
    }

    #[test]
    fn find_nulnul_trivial() {
        let buf = [0, 0];
        assert_eq!(find_nulnul(&buf), Some(1));
    }

    #[test]
    fn find_nulnul_with_data() {
        let buf = [1, 2, 3, 4, 0, 5, 4, 3, 2, 1, 0, 0];
        assert_eq!(find_nulnul(&buf), Some(11));
    }

    #[test]
    fn find_nulnul_with_data_more_at_end() {
        let buf = [1, 2, 3, 4, 0, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3];
        assert_eq!(find_nulnul(&buf), Some(11));
    }
}
