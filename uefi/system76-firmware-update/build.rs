// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2025 System76, Inc.

use std::env;

fn main() {
    println!("cargo::rerun-if-env-changed=BASEDIR");

    if env::var("BASEDIR").is_err() {
        println!("cargo::rustc-env=BASEDIR=system76-firmware-update");
    }

    build_cfg::configure(build_cfg::ModuleType::UefiApplication);
}
