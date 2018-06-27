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

impl<'a> System<'a> {
    pub fn new<'entry>(structure: &super::Structure<'a, 'entry>) -> Result<System<'a>, super::MalformedStructureError> {
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

        if structure.entry.major == 2 && structure.entry.minor < 1 {
            let_as_struct!(packed, SystemPacked_2_0, structure.data);

            Ok(System {
                manufacturer: structure.find_string(packed.manufacturer)?,
                product: structure.find_string(packed.product)?,
                version: structure.find_string(packed.version)?,
                serial: structure.find_string(packed.serial)?,
                uuid: None,
                wakeup: None,
                sku: None,
                family: None,
            })

        } else if structure.entry.major == 2 && structure.entry.minor < 4 {
            let_as_struct!(packed, SystemPacked_2_1, structure.data);

            Ok(System {
                manufacturer: structure.find_string(packed.v2_0.manufacturer)?,
                product: structure.find_string(packed.v2_0.product)?,
                version: structure.find_string(packed.v2_0.version)?,
                serial: structure.find_string(packed.v2_0.serial)?,
                uuid: Some(packed.uuid),
                wakeup: Some(packed.wakeup.into()),
                sku: None,
                family: None,
            })

        } else {
            let_as_struct!(packed, SystemPacked_2_4, structure.data);

            Ok(System {
                manufacturer: structure.find_string(packed.v2_1.v2_0.manufacturer)?,
                product: structure.find_string(packed.v2_1.v2_0.product)?,
                version: structure.find_string(packed.v2_1.v2_0.version)?,
                serial: structure.find_string(packed.v2_1.v2_0.serial)?,
                uuid: Some(packed.v2_1.uuid),
                wakeup: Some(packed.v2_1.wakeup.into()),
                sku: Some(structure.find_string(packed.sku)?),
                family: Some(structure.find_string(packed.family)?),
            })
        }
    }
}
