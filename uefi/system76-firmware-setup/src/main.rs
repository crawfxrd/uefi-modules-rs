// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2025 System76, Inc.

#![no_std]
#![no_main]
#![allow(non_snake_case)]

#[macro_use]
extern crate memoffset;
#[macro_use]
extern crate uefi_std as std;

use std::prelude::*;

use core::ptr;

mod display;
mod fde;
mod hii;
pub mod image;
mod key;
mod rng;
mod security;
mod ui;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> Status {
    let uefi = std::system_table();

    let _ = (uefi.BootServices.SetWatchdogTimer)(0, 0, 0, ptr::null());

    if let Err(err) = fde::Fde::install() {
        println!("Fde error: {:?}", err);
        let _ = key::key(true);
    }

    if let Err(err) = security::install() {
        println!("security error: {:?}", err);
        let _ = key::key(true);
    }

    Status(0)
}
