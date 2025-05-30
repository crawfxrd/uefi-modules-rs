// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2025 System76, Inc.

use intel_spi::{Mapper, PhysicalAddress, VirtualAddress};

pub struct UefiMapper;

impl Mapper for UefiMapper {
    unsafe fn map_aligned(
        &mut self,
        address: PhysicalAddress,
        _size: usize,
    ) -> Result<VirtualAddress, &'static str> {
        Ok(VirtualAddress(address.0))
    }

    unsafe fn unmap_aligned(
        &mut self,
        _address: VirtualAddress,
        _size: usize,
    ) -> Result<(), &'static str> {
        Ok(())
    }

    fn page_size(&self) -> usize {
        //TODO: get dynamically
        4096
    }
}
