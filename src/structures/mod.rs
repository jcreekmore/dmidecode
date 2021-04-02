//! SMBIOS structures
//!
//! The System Information is presented to an application as a set of structures that are obtained
//! by traversing the SMBIOS structure table referenced by the SMBIOS Entry Point Structure.


#[path = "000_bios.rs"]
pub mod bios;
pub use self::bios::Bios;

#[path = "001_system.rs"]
pub mod system;
pub use self::system::System;

#[path = "002_baseboard.rs"]
pub mod baseboard;
pub use self::baseboard::BaseBoard;

#[path = "003_enclosure.rs"]
pub mod enclosure;
pub use self::enclosure::Enclosure;

#[path = "004_processor.rs"]
pub mod processor;
pub use self::processor::Processor;

#[path = "007_cache.rs"]
pub mod cache;
pub use self::cache::Cache;

#[path = "008_port_connector.rs"]
pub mod port_connector;
pub use self::port_connector::PortConnector;

#[path = "009_system_slots.rs"]
pub mod system_slots;
pub use self::system_slots::SystemSlots;

#[path = "011_oem_strings.rs"]
pub mod oem_strings;
pub use self::oem_strings::OemStrings;

#[path = "012_system_configuration_options.rs"]
pub mod system_configuration_options;
pub use self::system_configuration_options::SystemConfigurationOptions;

#[path = "013_bios_language.rs"]
pub mod bios_language;
pub use self::bios_language::BiosLanguage;

#[path = "014_group_associations.rs"]
pub mod group_associations;
pub use self::group_associations::GroupAssociations;

#[path = "015_system_event_log/mod.rs"]
pub mod system_event_log;
pub use self::system_event_log::SystemEventLog;

#[path = "016_physical_memory_array.rs"]
pub mod physical_memory_array;
pub use self::physical_memory_array::PhysicalMemoryArray;

#[path = "017_memory_device.rs"]
pub mod memory_device;
pub use self::memory_device::MemoryDevice;

#[path = "018_memory_error_32.rs"]
pub mod memory_error_32;
pub use self::memory_error_32::MemoryError32;

#[path = "019_memory_array_mapped_address.rs"]
pub mod memory_array_mapped_address;
pub use self::memory_array_mapped_address::MemoryArrayMappedAddress;

#[path = "020_memory_device_mapped_address.rs"]
pub mod memory_device_mapped_address;
pub use self::memory_device_mapped_address::MemoryDeviceMappedAddress;

#[path = "021_built_in_pointing_device.rs"]
pub mod built_in_pointing_device;
pub use self::built_in_pointing_device::BuiltInPointingDevice;

#[path = "022_portable_battery.rs"]
pub mod portable_battery;
pub use self::portable_battery::PortableBattery;
