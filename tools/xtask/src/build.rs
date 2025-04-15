// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2025 System76, Inc.

use std::io::{self, BufRead, BufReader};
use std::process;

pub fn build() -> io::Result<()> {
    let mut cargo = process::Command::new("cargo");

    cargo.arg("build").args(["--profile", "release"]).args(["--target", "x86_64-unknown-uefi"]);

    for pkg in crate::UEFI_MODULES {
        cargo.args(["--package", pkg]);
    }

    let child = cargo.stdout(process::Stdio::piped()).spawn()?;
    let stdout = child.stdout.expect("no output from cargo");
    let reader = BufReader::new(stdout);
    reader.lines().map_while(Result::ok).for_each(|line| println!("{}", line));

    Ok(())
}
