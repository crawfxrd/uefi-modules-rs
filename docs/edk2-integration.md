# Integrating with edk2

Currently how we integrate with [edk2](https://github.com/tianocore/edk2):

- Append the path to this workspace to edk2 `PACKAGES_PATH`
  - e.g.: `PACKAGES_PATH=../uefi-modules-rs`
- Specify the path to the INF path relative to the workspace
  - e.g.: `FIRMWARE_OPEN_GOP_POLICY=uefi/system76-gop-policy/module.inf`
