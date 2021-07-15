#![no_std]
#![no_main]
#![feature(asm)]

mod algo;

use core::mem::MaybeUninit;

use self::algo::*;

struct NumWorksAlgo;

algo!(NumWorksAlgo);

impl FlashAlgo for NumWorksAlgo {
    fn new(_address: u32, _clock: u32, _function: u32) -> Result<Self, ErrorCode> {
        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        Err(ErrorCode::new(0x70d0).unwrap())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        Ok(())
    }

    fn program_page(&mut self, addr: u32, size: u32, data: *const u8) -> Result<(), ErrorCode> {
        Ok(())
    }
}
