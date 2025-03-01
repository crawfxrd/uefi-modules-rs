# Adding packages

## UEFI modules

Create a new package in the `uefi/` directory.

Example:

```
cargo new --bin uefi/example-driver
cargo new --lib uefi/example-lib
```

### Package build script

`*-unknown-uefi` by default creates UEFI applications. For drivers, the
subsystem type must be set by a linker argument.

Create a `build.rs` file in the package root that sets `/subsystem` for UEFI
targets.

Example:

```rust
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.ends_with("-unknown-uefi") {
        println!("cargo::rustc-link-arg=/subsystem:efi_boot_service_driver");
    }
}
```

### EDK II Module Information (INF) file

Add a `module.inf` if the package is to be integrated into an edk2 build.

Example:

```ini
[Defines]
  INF_VERSION                    = 1.27
  BASE_NAME                      = example-driver
  FILE_GUID                      = fefc99de-af68-4858-ab50-4fad04805400
  MODULE_TYPE                    = DXE_DRIVER
  VERSION_STRING                 = 0.1

[Binaries.X64]
  PE32|target/x86_64-unknown-uefi/release/example-driver.efi|*
```

## Host tools

Create a new package in the `tools/` directory.

Example:

```
cargo new --bin tools/host-tool
cargo new --bin tools/host-lib
```

If it is to be used as a top-level tool, like `xtask`, add an alias for it in
`.cargo/config.toml`.
