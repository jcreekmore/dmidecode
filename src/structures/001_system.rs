//! System Information (Type 1)
//!
//! The information in this structure defines attributes of the overall system and is intended to
//! be associated with the Component ID group of the system’s MIF. An SMBIOS implementation is
//! associated with a single system instance and contains one and only one System Information
//! (Type 1) structure.

use crate::{MalformedStructureError, RawStructure};

/// The wakeup type defined in the SMBIOS specification.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

/// The `System` table defined in the SMBIOS specification.
///
/// Optional fields will only be set if the version of the parsed SMBIOS table
/// is high enough to have defined the field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct System<'buffer> {
    pub handle: u16,
    pub manufacturer: &'buffer str,
    pub product: &'buffer str,
    pub version: &'buffer str,
    pub serial: &'buffer str,
    pub uuid: Option<uuid::Uuid>,
    pub wakeup: Option<WakeupType>,
    pub sku: Option<&'buffer str>,
    pub family: Option<&'buffer str>,
}

impl<'buffer> System<'buffer> {
    pub(crate) fn try_from(structure: RawStructure<'buffer>) -> Result<System<'buffer>, MalformedStructureError> {
        /// Internal helpers for decoding complex representations
        mod wrapper {
            /// Wrapper around the 16-byte UUID field to implement SMBIOS-version-aware decoding.
            #[repr(transparent)]
            pub struct UuidRepr([u8; 16]);

            impl UuidRepr {
                /// Decodes the raw UUID according to the SMBIOS specification.
                ///
                /// - For SMBIOS versions >= 2.6, the first 3 fields are encoded in little-endian format according
                ///   to the SMBIOS specification.
                /// - For older versions, the UUID is returned as-is to be consistent with `dmidecode` utility.
                #[inline(always)]
                pub fn decode_by_smbios_version(self, version: crate::SmbiosVersion) -> uuid::Uuid {
                    if version < (2, 6).into() {
                        uuid::Uuid::from_bytes(self.0)
                    } else {
                        uuid::Uuid::from_bytes_le(self.0)
                    }
                }
            }
        }

        #[repr(C)]
        #[repr(packed)]
        struct SystemPacked2_0 {
            manufacturer: u8,
            product: u8,
            version: u8,
            serial: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct SystemPacked2_1 {
            v2_0: SystemPacked2_0,
            uuid: wrapper::UuidRepr,
            wakeup: u8,
        }

        #[repr(C)]
        #[repr(packed)]
        struct SystemPacked2_4 {
            v2_1: SystemPacked2_1,
            sku: u8,
            family: u8,
        }

        if structure.version < (2, 1).into() {
            let_as_struct!(packed, SystemPacked2_0, structure.data);

            Ok(System {
                handle: structure.handle,
                manufacturer: structure.find_string(packed.manufacturer)?,
                product: structure.find_string(packed.product)?,
                version: structure.find_string(packed.version)?,
                serial: structure.find_string(packed.serial)?,
                uuid: None,
                wakeup: None,
                sku: None,
                family: None,
            })
        } else if structure.version < (2, 4).into() {
            let_as_struct!(packed, SystemPacked2_1, structure.data);

            Ok(System {
                handle: structure.handle,
                manufacturer: structure.find_string(packed.v2_0.manufacturer)?,
                product: structure.find_string(packed.v2_0.product)?,
                version: structure.find_string(packed.v2_0.version)?,
                serial: structure.find_string(packed.v2_0.serial)?,
                uuid: Some(packed.uuid.decode_by_smbios_version(structure.version)),
                wakeup: Some(packed.wakeup.into()),
                sku: None,
                family: None,
            })
        } else {
            let_as_struct!(packed, SystemPacked2_4, structure.data);

            Ok(System {
                handle: structure.handle,
                manufacturer: structure.find_string(packed.v2_1.v2_0.manufacturer)?,
                product: structure.find_string(packed.v2_1.v2_0.product)?,
                version: structure.find_string(packed.v2_1.v2_0.version)?,
                serial: structure.find_string(packed.v2_1.v2_0.serial)?,
                uuid: Some(packed.v2_1.uuid.decode_by_smbios_version(structure.version)),
                wakeup: Some(packed.v2_1.wakeup.into()),
                sku: Some(structure.find_string(packed.sku)?),
                family: Some(structure.find_string(packed.family)?),
            })
        }
    }
}
