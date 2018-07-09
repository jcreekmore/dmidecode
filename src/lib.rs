#[macro_use]
extern crate bitflags;
extern crate core;
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
        let $name: $ty = unsafe { ptr::read($data.as_ptr() as * const _) };
    };
}

pub mod system;
pub use system::System;

pub mod baseboard;
pub use baseboard::BaseBoard;

pub mod processor;
pub use processor::Processor;

///
/// An SMBIOS `Entry` structure.
///
/// The SMBIOS `Entry` structure is used to verify that a set of SMBIOS tables exist
/// in memory and what version of the SMBIOS specification should be used to
/// access the tables.
///
#[repr(C)]
#[repr(packed)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

/// Failure type for trying to find the SMBIOS `Entry` structure in memory.
#[derive(Debug, Fail)]
pub enum InvalidEntryError {
    /// The SMBIOS `Entry` structure was not found in the memory buffer.
    #[fail(display = "Input did not contain a valid SMBIOS entry")]
    NotFound,
    /// The SMBIOS `Entry` structure was versioned before 2.0.
    #[fail(display = "Input version number was below 2.0: {}", _0)]
    TooOldVersion(u8),
    /// The SMBIOS `Entry` structure was smaller than the size of the SMBIOS 2.1 structure.
    #[fail(display = "Input contained an invalid-sized SMBIOS entry: {}", _0)]
    BadSize(u8),
    /// The SMBIOS `Entry` structure had an invalid checksum.
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

#[doc(hidden)]
macro_rules! lib_ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e);
        }
    };
}

impl Entry {
    /// Search for an instance of an SMBIOS `Entry` structure in a memory `buffer`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate dmidecode;
    /// use dmidecode::Entry;
    ///
    /// const ENTRY_BIN: &'static [u8] = include_bytes!("./entry.bin");
    ///
    /// let entry = Entry::search(ENTRY_BIN);
    /// ```
    ///
    /// # Errors
    ///
    /// If this function fails to find a valid SMBIOS `Entry` structure, it will return
    /// an `InvalidEntryError` variant.
    pub fn search(buffer: &[u8]) -> Result<Entry, InvalidEntryError> {
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

    /// Create an iterator across the SMBIOS structures found in `buffer`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate failure;
    /// # extern crate dmidecode;
    /// # use failure::Error;
    /// use dmidecode::Entry;
    /// # fn try_main() -> Result<(), Error> {
    /// #
    /// const DMIDECODE_BIN: &'static [u8] = include_bytes!("./dmidecode.bin");
    ///
    /// let entry = Entry::search(DMIDECODE_BIN)?;
    /// for s in entry.structures(&DMIDECODE_BIN[entry.smbios_address as usize..]) {
    ///   let table = s?;
    /// }
    /// Ok(())
    /// # }
    /// ```
    pub fn structures<'entry, 'buffer>(&'entry self, buffer: &'buffer [u8]) -> Structures<'entry, 'buffer> {
        Structures {
            entry: self,
            count: 0,
            idx: 0u16,
            buffer: buffer,
        }
    }
}

/// An iterator that traverses the SMBIOS structure tables.
/// This struct is produced by the `structures` method on `Entry`. See its documentation for more details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Structures<'entry, 'buffer> {
    entry: &'entry Entry,
    count: u16,
    idx: u16,
    buffer: &'buffer [u8],
}

/// Variant structure for decoding the SMBIOS table types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Structure<'entry, 'buffer> {
    System(System<'buffer>),
    BaseBoard(BaseBoard<'buffer>),
    Processor(Processor<'buffer>),
    Other(RawStructure<'entry, 'buffer>)
}

/// Failure type for trying to decode the SMBIOS `Structures` iterator into the `Structure` variant type.
#[derive(Debug, Fail)]
pub enum MalformedStructureError {
    /// The SMBIOS structure exceeds the end of the memory buffer given to the `Entry::structure` method.
    #[fail(display = "Structure at offset {} with length {} extends beyond SMBIOS", _0, _1)]
    BadSize(u16, u8),
    /// The SMBIOS structure contains an unterminated strings section.
    #[fail(display = "Structure at offset {} with unterminated strings", _0)]
    UnterminatedStrings(u16),
    /// The SMBIOS structure contains an invalid string index.
    #[fail(display = "Structure {:?} with handle {} has invalid string index {}", _0, _1, _2)]
    InvalidStringIndex(InfoType, u16, u8),
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

impl<'entry, 'buffer> Iterator for Structures<'entry, 'buffer> {
    type Item = Result<Structure<'entry, 'buffer>, MalformedStructureError>;

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
            return Some(Err(MalformedStructureError::BadSize(self.idx, header.len)));
        }

        let term = find_nulnul(&self.buffer[(strings_idx as usize)..]);
        let strings_len = match term {
            Some(terminator) => (terminator + 1) as u16,
            None => {
                return Some(Err(MalformedStructureError::UnterminatedStrings(self.idx)));
            }
        };

        let structure = RawStructure {
            info: header._type.into(),
            handle: header.handle,
            entry: self.entry,
            data: &self.buffer
                [(self.idx + mem::size_of::<HeaderPacked>() as u16) as usize..strings_idx as usize],
            strings: &self.buffer[strings_idx as usize..(strings_idx + strings_len) as usize],
        };

        self.idx = strings_idx + strings_len;
        self.count += 1;

        Some(match structure.info {
            InfoType::System => System::new(structure).map(Structure::System),
            InfoType::BaseBoard => BaseBoard::new(structure).map(Structure::BaseBoard),
            InfoType::Processor => Processor::new(structure).map(Structure::Processor),
            _ => Ok(Structure::Other(structure))
        })
    }
}

#[doc(hidden)]
#[repr(C)]
#[repr(packed)]
struct HeaderPacked {
    pub _type: u8,
    pub len: u8,
    pub handle: u16,
}

/// The raw SMBIOS structure information for structures that are not handled by this crate, such as Oem structures.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RawStructure<'entry, 'buffer> {
    pub info: InfoType,
    pub handle: u16,
    pub entry: &'entry Entry,
    pub data: &'buffer [u8],
    strings: &'buffer [u8],
}

impl<'entry, 'buffer> RawStructure<'entry, 'buffer> {
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
            self.strings()
                .nth((idx - 1) as usize)
                .ok_or_else(|| MalformedStructureError::InvalidStringIndex(self.info, self.handle, idx))
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
        Entry::search(ENTRY_BIN).unwrap();
        Entry::search(DMIDECODE_BIN).unwrap();
    }

    #[test]
    #[should_panic]
    fn doesnt_find_smbios_entry() {
        Entry::search(DMI_BIN).unwrap();
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
        let entry = Entry::search(DMIDECODE_BIN).unwrap();
        for s in entry.structures(&DMIDECODE_BIN[(entry.smbios_address as usize)..]).filter_map(|s| s.ok()) {
            println!("{:?}", s);
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
