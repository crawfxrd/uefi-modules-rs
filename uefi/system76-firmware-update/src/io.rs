// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2025 System76, Inc.

use core::char;
use std::prelude::*;
use std::uefi::text::TextInputKey;

pub fn wait_key() -> Result<char> {
    let uefi = std::system_table();

    let mut index = 0;
    Result::from((uefi.BootServices.WaitForEvent)(1, &uefi.ConsoleIn.WaitForKey, &mut index))?;

    let mut input = TextInputKey {
        ScanCode: 0,
        UnicodeChar: 0,
    };

    Result::from((uefi.ConsoleIn.ReadKeyStroke)(uefi.ConsoleIn, &mut input))?;

    Ok(unsafe { char::from_u32_unchecked(u32::from(input.UnicodeChar)) })
}
