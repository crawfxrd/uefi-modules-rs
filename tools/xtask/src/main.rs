// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2025 System76, Inc.

mod build;
mod cli;

use clap::Parser;

// TODO: Declare outside of tool.
pub const UEFI_MODULES: &[&str] =
    &["system76-gop-policy", "system76-firmware-setup", "system76-firmware-update"];

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Command::Build => build::build().unwrap(),
    }
}
