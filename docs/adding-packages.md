# Adding packages

## UEFI modules

Create a new package in the `uefi/` directory.

Example:

```
cargo new --bin uefi/example-driver
cargo new --lib uefi/example-lib
```

### Package build script

The `build-cfg` library is used to set link args for UEFI modules based on
their edk2 module type. Add it as a dependency, and call `configure()` from
the package build script.

Example:

```toml
# Cargo.toml
[build-dependencies]
build-cfg.workspace = true
```

```rust
// build.rs
fn main() {
    build_cfg::configure(build_cfg::ModuleType::UefiDriver);
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
cargo new --lib tools/host-lib
```

Add an alias for it in `.cargo/config.toml` if it is to be used as a top-level
tool.
