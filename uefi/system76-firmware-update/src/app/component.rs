// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2025 System76, Inc.

use std::prelude::*;

pub trait Component {
    fn name(&self) -> &str;
    fn path(&self) -> &str;
    fn model(&self) -> &str;
    fn version(&self) -> &str;
    fn validate(&self) -> Result<bool>;
    fn flash(&self) -> Result<()>;
}
