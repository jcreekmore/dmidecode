#![no_std]

#[macro_use]
extern crate bitflags;
extern crate failure;
#[macro_use]
extern crate failure_derive;

use core::mem;
use core::str;

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

pub mod memory;
pub use memory::MemoryDevice;

pub mod system;
pub use system::System;

pub mod baseboard;
pub use baseboard::BaseBoard;

pub mod processor;
pub use processor::Processor;

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
    /// const DMIDECODE_BIN: &'static [u8] = include_bytes!("./test-data/dmidecode.bin");
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
    /// const ENTRY_BIN: &'static [u8] = include_bytes!("./test-data/entry.bin");
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
    System(System<'buffer>),
    BaseBoard(BaseBoard<'buffer>),
    Processor(Processor<'buffer>),
    MemoryDevice(MemoryDevice<'buffer>),
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
    #[fail(display = "{}", _0)]
    InvalidSlice(#[fail(cause)] core::array::TryFromSliceError),
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
            InfoType::System => System::try_from(structure).map(Structure::System),
            InfoType::BaseBoard => BaseBoard::try_from(structure).map(Structure::BaseBoard),
            InfoType::Processor => Processor::try_from(structure).map(Structure::Processor),
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
    fn from(kind: u8) -> InfoType {
        match kind {
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
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;

    const DMIDECODE_BIN: &'static [u8] = include_bytes!("./test-data/dmidecode.bin");
    const ENTRY_V2_BIN: &'static [u8] = include_bytes!("./test-data/entry.bin");
    const DMI_V2_BIN: &'static [u8] = include_bytes!("./test-data/dmi.bin");
    const ENTRY_V3_BIN: &'static [u8] = include_bytes!("./test-data/entry_v3.bin");
    const DMI_V3_BIN: &'static [u8] = include_bytes!("./test-data/dmi_v3.bin");
    const DMI_V3_SHORT: &'static [u8] = include_bytes!("./test-data/dmi_v3_short.bin");
    const ENTRY_V3_SHORT: &'static [u8] = include_bytes!("./test-data/entry_v3_short.bin");

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
