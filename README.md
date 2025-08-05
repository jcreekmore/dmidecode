# dmidecode

[<img alt="github" src="https://img.shields.io/badge/github-jcreekmore/dmidecode-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/jcreekmore/dmidecode)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dmidecode.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dmidecode)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dmidecode-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dmidecode)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/jcreekmore/dmidecode/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/jcreekmore/dmidecode/actions?query=branch%3Amaster)

A Rust library for parsing raw **SMBIOS/DMI tables**. This crate lets you read and decode system firmware information provided via `/sys/firmware/dmi/tables/` on Linux, or directly from memory dumps.

```toml
# Cargo.toml
[dependencies]
dmidecode = "1"
```

## Example

```rust
use dmidecode::Structure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Collecting DMI Information...");

    // Get the SMBIOS header and DMI table from sysfs.
    let buf = std::fs::read("/sys/firmware/dmi/tables/smbios_entry_point")?;
    let dmi = std::fs::read("/sys/firmware/dmi/tables/DMI")?;
    let entry = dmidecode::EntryPoint::search(&buf)?;

    for table in entry.structures(&dmi) {
        let Ok(table) = table else {
            eprintln!("DMI tables contain malformed structure: {table:?}");
            continue;
        };

        match table {
            Structure::System(system) => {
                // do stuff
            }
            Structure::BaseBoard(base_board) => {
                // do stuff
            }
            // ...
            _ => continue,
        }
    }

    Ok(())
}
```


## No-std support

In no_std mode, almost all of the same API is available and works the same
way. To depend on `dmidecode` in `no_std` mode, disable our default enabled
`std` feature in `Cargo.toml`.

The `std` feature just implements the `Error` trait on error types used by
`dmidecode`.

```toml
[dependencies]
dmidecode = { version = "1", default-features = false }
```

## Rust Version Support The minimum supported Rust version is documented in
the Cargo.toml file. This may be bumped in minor releases as necessary.

## License

`dmidecode` is released under the terms of the [MIT](./LICENSE) license.
