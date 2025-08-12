# System76 Platform GOP Policy

A driver that installs the Intel Platform GOP Policy protocol. Requires a VBT
file included in the UEFI FFS with a specific GUID.

```
FILE FREEFORM = 56752da9-de6b-4895-8819-1945b6b76c22 {
  SECTION RAW = vbt.rom
  SECTION UI = "IntelGopVbt"
}
```

```
cargo build --release --target x86_64-unknown-uefi
```
