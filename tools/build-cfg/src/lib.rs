// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2025 System76, Inc.

use std::env;

/// PE32 subsystem type
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Subsystem {
    Application,
    BootServiceDriver,
    RuntimeDriver,
}

impl Subsystem {
    fn link_name(&self) -> &str {
        match self {
            Self::Application => "efi_application",
            Self::BootServiceDriver => "efi_boot_service_driver",
            Self::RuntimeDriver => "efi_runtime_driver",
        }
    }
}

/// edk2 module type
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ModuleType {
    DxeDriver,
    DxeRuntimeDriver,
    UefiApplication,
    UefiDriver,
}

impl ModuleType {
    /// Get the PE32 subsystem type for the module.
    pub fn subsystem(&self) -> Subsystem {
        match self {
            Self::DxeDriver => Subsystem::BootServiceDriver,
            Self::DxeRuntimeDriver => Subsystem::RuntimeDriver,
            Self::UefiApplication => Subsystem::Application,
            Self::UefiDriver => Subsystem::BootServiceDriver,
        }
    }

    pub fn print_link_args(&self) {
        match self {
            Self::UefiApplication => (),
            _ => {
                println!("cargo::rustc-link-arg=/heap:0,0");
                println!("cargo::rustc-link-arg=/stack:0,0");
                println!("cargo::rustc-link-arg=/dll");
                println!("cargo::rustc-link-arg=/base:0");
                println!("cargo::rustc-link-arg=/align:32");
                println!("cargo::rustc-link-arg=/filealign:32");
            }
        }

        let pe32ss = self.subsystem();
        println!("cargo::rustc-link-arg=/subsystem:{}", pe32ss.link_name());
    }
}

// TODO: Read subsystem type from Cargo.toml (package.metadata).
pub fn configure(module: ModuleType) {
    if env::var("OUT_DIR").is_err() {
        println!("cargo::error=must be called from a build script");
        return;
    }

    let target = env::var("TARGET").unwrap();
    if !target.ends_with("-unknown-uefi") {
        println!("cargo::warning=not a valid UEFI target: {}", target);
        return;
    }

    let manifest_path = env::var("CARGO_MANIFEST_PATH").unwrap();
    println!("cargo::rerun-if-changed={}", manifest_path);

    module.print_link_args();
}
