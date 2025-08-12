# Building

Specify the UEFI target and, optionally, the profile to use and a specific
package to build.

Examples:

```
# Build all packages
cargo build --release --target x86_64-unknown-uefi
# Build only firmware-update
cargo build --release --target x86_64-unknown-uefi -p system76-firmware-update
```
