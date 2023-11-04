#![no_std]
#![feature(asm_const)]

mod enable;
mod disable;

#[cfg(feature = "enable")]
use enable as mmu;
#[cfg(feature = "disable")]
use disable as mmu;

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;

pub unsafe fn pre_mmu() {
    mmu::pre_mmu();
}

pub unsafe fn enable_mmu() {
    mmu::enable_mmu();
}

pub unsafe fn post_mmu() {
    mmu::post_mmu();
}
