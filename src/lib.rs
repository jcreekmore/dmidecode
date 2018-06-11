extern crate core;
#[macro_use]
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

impl Entry {
    pub fn new(buffer: &[u8]) -> Result<Entry, failure::Error> {
        find_signature(buffer)
            .ok_or_else(|| InvalidEntryError::NotFound.into())
            .and_then(|start| {
                let sub_buffer = &buffer[start..];
                ensure!(
                    sub_buffer.len() >= mem::size_of::<Entry>(),
                    InvalidEntryError::BadSize(sub_buffer.len() as u8)
                );

                let entry: Entry = unsafe { std::ptr::read(sub_buffer.as_ptr() as *const _) };
                ensure!(
                    entry.len as usize >= mem::size_of::<Entry>(),
                    InvalidEntryError::BadSize(entry.len)
                );

                ensure!(
                    sub_buffer.len() as u8 >= entry.len,
                    InvalidEntryError::BadSize(sub_buffer.len() as u8)
                );

                let mut sum = 0u8;
                for val in &sub_buffer[0..(entry.len as usize)] {
                    sum = sum.wrapping_add(*val);
                }
                ensure!(sum == 0, InvalidEntryError::BadChecksum(sum));

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

impl<'a, 'b> Iterator for Structures<'a, 'b> {
    type Item = Result<Structure<'b>, failure::Error>;

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
            return Some(Err(InvalidStructureError::BadSize(self.idx, header.len).into()));
        }

        let term = find_nulnul(&self.buffer[(strings_idx as usize)..]);
        let strings_len = match term {
            Some(terminator) => (terminator + 1) as u16,
            None => {
                return Some(Err(InvalidStructureError::UnterminatedStrings(self.idx).into()));
            }
        };

        let structure = Structure {
            info: header._type.into(),
            handle: header.handle,
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

#[derive(Debug)]
pub struct Structure<'a> {
    pub info: InfoType,
    pub handle: u16,
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

impl<'a> Structure<'a> {
    fn strings(&self) -> impl Iterator<Item = &'a str> {
        self.strings.split(|elm| *elm == 0).filter_map(|slice| {
            if slice.is_empty() {
                None
            } else {
                unsafe { Some(str::from_utf8_unchecked(slice)) }
            }
        })
    }

    fn find_string(&self, idx: u8) -> Result<&'a str, failure::Error> {
        self.strings().nth((idx - 1) as usize).ok_or_else(|| {
            MalformedStructureError::InvalidStringIndex(self.info, self.handle, idx).into()
        })
    }

    pub fn system(&self) -> Result<System<'a>, failure::Error> {
        ensure!(
            self.info == InfoType::System,
            MalformedStructureError::BadType(self.info, self.handle, "System")
        );

        #[repr(C)]
        #[repr(packed)]
        struct SystemPacked {
            manufacturer: u8,
            product: u8,
            version: u8,
            serial: u8,
            uuid: [u8; 16],
            wakeup: u8,
        }

        let packed: SystemPacked = unsafe { std::ptr::read(self.data.as_ptr() as *const _) };

        Ok(System {
            manufacturer: self.find_string(packed.manufacturer)?,
            product: self.find_string(packed.product)?,
            version: self.find_string(packed.version)?,
            serial: self.find_string(packed.serial)?,
            uuid: packed.uuid,
            wakeup: packed.wakeup.into(),
        })
    }

    pub fn base_board(&self) -> Result<BaseBoard<'a>, failure::Error> {
        ensure!(
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
        }

        let packed: BaseBoardPacked = unsafe { std::ptr::read(self.data.as_ptr() as *const _) };

        Ok(BaseBoard {
            manufacturer: self.find_string(packed.manufacturer)?,
            product: self.find_string(packed.product)?,
            version: self.find_string(packed.version)?,
            serial: self.find_string(packed.serial)?,
        })
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
    pub uuid: [u8; 16],
    pub wakeup: WakeupType,
}

#[derive(Debug)]
pub struct BaseBoard<'a> {
    pub manufacturer: &'a str,
    pub product: &'a str,
    pub version: &'a str,
    pub serial: &'a str,
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
