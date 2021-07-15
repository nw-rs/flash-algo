#![no_std]
#![no_main]
#![feature(asm)]

mod algo;

use core::mem::MaybeUninit;
use rustworks::external_flash::ExternalFlash;
use rustworks::external_flash::Indirect;
use rustworks::stm32f7xx_hal::delay::Delay;
use rustworks::stm32f7xx_hal::gpio::GpioExt;
use rustworks::stm32f7xx_hal::gpio::Speed;
use rustworks::stm32f7xx_hal::pac::Peripherals;
use rustworks::stm32f7xx_hal::prelude::*;
use rustworks::stm32f7xx_hal::rcc::{HSEClock, HSEClockMode};

use self::algo::*;

struct NumWorksAlgo {
    external_flash: ExternalFlash<Indirect>,
}

algo!(NumWorksAlgo);

impl FlashAlgo for NumWorksAlgo {
    fn new(_address: u32, _clock: u32, _function: u32) -> Result<Self, ErrorCode> {
        let mut peripherals = Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap();

        let gpiob = peripherals.GPIOB.split();
        let gpioc = peripherals.GPIOC.split();
        let gpiod = peripherals.GPIOD.split();
        let gpioe = peripherals.GPIOE.split();

        let qspi_pins = (
            gpiob.pb2.into_alternate_af9().set_speed(Speed::VeryHigh),
            gpiob.pb6.into_alternate_af10().set_speed(Speed::VeryHigh),
            gpioc.pc9.into_alternate_af9().set_speed(Speed::VeryHigh),
            gpiod.pd12.into_alternate_af9().set_speed(Speed::VeryHigh),
            gpiod.pd13.into_alternate_af9().set_speed(Speed::VeryHigh),
            gpioe.pe2.into_alternate_af9().set_speed(Speed::VeryHigh),
        );

        let external_flash =
            ExternalFlash::new(&mut peripherals.RCC, peripherals.QUADSPI, qspi_pins);

        let rcc = peripherals.RCC.constrain();
        let clocks = rcc
            .cfgr
            .hse(HSEClock::new(8.mhz(), HSEClockMode::Oscillator))
            .use_pll()
            .sysclk(rustworks::HCLK.hz())
            .freeze();
        let mut delay = Delay::new(cp.SYST, clocks);

        Ok(Self {
            external_flash: external_flash.init(&mut delay),
        })
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        self.external_flash.chip_erase();
        Ok(())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        self.external_flash.block_erase_4k(addr);
        Ok(())
    }

    fn program_page(&mut self, addr: u32, size: u32, data: *const u8) -> Result<(), ErrorCode> {
        self.external_flash.program_page(addr, unsafe {
            core::slice::from_raw_parts(data, size as usize)
        });
        Ok(())
    }
}
