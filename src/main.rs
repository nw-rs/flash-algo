#![no_std]
#![no_main]
#![feature(asm)]

mod algo;

use core::mem::MaybeUninit;

use cortex_m::asm;
use stm32f7xx_hal::pac::{GPIOB, GPIOC, GPIOD, GPIOE, QUADSPI, RCC};

use self::algo::*;

const FLASH_ADDRESS_SIZE: u8 = 23;

//#[allow(dead_code)]
#[repr(u8)]
enum QspiWidth {
    Single = 0b01,
    Quad = 0b11,
}

/// The different QSPI functional modes.
#[repr(u8)]
enum QspiMode {
    IndirectWrite = 0b00,
    IndirectRead = 0b01,
}

/// The number of bytes required to specify addresses on the chip.
#[repr(u8)]
enum QspiSize {
    OneByte = 0b00,
    ThreeBytes = 0b10,
}

/// Commands (instructions) that can be sent to the flash chip.
#[repr(u8)]
enum Command {
    ReadStatusRegister1 = 0x05,
    WriteStatusRegister2 = 0x31,
    WriteEnable = 0x06,
    WriteEnableVolatile = 0x50,
    PageProgram = 0x02,
    EnableQPI = 0x38,
    ChipErase = 0xC7,
    Erase64KbyteBlock = 0xD8,
    ReleaseDeepPowerDown = 0xAB,
    DisableQPI = 0xFF,
}

struct NumWorksAlgo;

algo!(NumWorksAlgo);

impl FlashAlgo for NumWorksAlgo {
    fn new(_address: u32, _clock: u32, _function: u32) -> Result<Self, ErrorCode> {
        unsafe {
            let rcc = &(*RCC::ptr());
            rcc.ahb3enr.modify(|_, w| w.qspien().set_bit());

            rcc.ahb3rstr.modify(|_, w| w.qspirst().reset());
            rcc.ahb3rstr.modify(|_, w| w.qspirst().clear_bit());

            rcc.ahb1enr.modify(|_, w| {
                w.gpioben()
                    .set_bit()
                    .gpiocen()
                    .set_bit()
                    .gpioden()
                    .set_bit()
                    .gpioeen()
                    .set_bit()
            });

            // PB2<Alternate<AF9>>
            // PB6<Alternate<AF10>>
            // PC9<Alternate<AF9>>
            // PD12<Alternate<AF9>>
            // PD13<Alternate<AF9>>
            // PE2<Alternate<AF9>>

            let gpiob = &(*GPIOB::ptr());
            let gpioc = &(*GPIOC::ptr());
            let gpiod = &(*GPIOD::ptr());
            let gpioe = &(*GPIOE::ptr());

            gpiob.afrl.modify(|_, w| w.afrl2().af9().afrl6().af10());
            gpioc.afrh.modify(|_, w| w.afrh9().af9());
            gpiod.afrh.modify(|_, w| w.afrh12().af9().afrh13().af9());
            gpioe.afrl.modify(|_, w| w.afrl2().af9());

            gpiob
                .moder
                .modify(|_, w| w.moder2().alternate().moder6().alternate());
            gpioc.moder.modify(|_, w| w.moder9().alternate());
            gpiod
                .moder
                .modify(|_, w| w.moder12().alternate().moder13().alternate());
            gpioe.moder.modify(|_, w| w.moder2().alternate());

            gpiob
                .ospeedr
                .modify(|_, w| w.ospeedr2().very_high_speed().ospeedr6().very_high_speed());
            gpioc.ospeedr.modify(|_, w| w.ospeedr9().very_high_speed());
            gpiod.ospeedr.modify(|_, w| {
                w.ospeedr12()
                    .very_high_speed()
                    .ospeedr13()
                    .very_high_speed()
            });
            gpioe.ospeedr.modify(|_, w| w.ospeedr2().very_high_speed());

            let qspi = &(*QUADSPI::ptr());
            // Single flash mode with a QSPI clock prescaler of 2 (216 / 2 = 108 MHz), FIFO
            // threshold only matters for DMA and is set to 4 to allow word sized DMA requests

            // Configure controller for flash chip.
            qspi.dcr.write_with_zero(|w| {
                w.fsize()
                    .bits(FLASH_ADDRESS_SIZE - 1)
                    .csht()
                    .bits(2)
                    .ckmode()
                    .set_bit()
            });

            qspi.cr
                .write_with_zero(|w| w.prescaler().bits(3).en().set_bit());
        }
        // Turn on the chip.
        send_spi_command(Command::ReleaseDeepPowerDown, None);

        for _ in 0..1000000 {
            asm::nop();
        }

        // Enable writing to the chip so that the status register can be changed.
        send_spi_command(Command::WriteEnableVolatile, None);

        // Set QPI to enabled in the chip's status register.
        send_spi_command(Command::WriteStatusRegister2, Some(0x02));

        // Enable QPI on the chip.
        send_spi_command(Command::EnableQPI, None);

        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        qpi_command(Command::WriteEnable);
        qpi_command(Command::ChipErase);

        wait_busy();

        Ok(())
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        qpi_command(Command::WriteEnable);

        let qspi = unsafe { &(*QUADSPI::ptr()) };

        qspi.abr.write(|w| unsafe { w.bits(addr) });

        // TODO: Why doesn't this work with address bytes?
        qspi.ccr.write(|w| unsafe {
            w.fmode()
                .bits(QspiMode::IndirectWrite as u8)
                .imode()
                .bits(QspiWidth::Quad as u8)
                .abmode()
                .bits(QspiWidth::Quad as u8)
                .absize()
                .bits(QspiSize::ThreeBytes as u8)
                // .admode()
                // .bits(QspiWidth::Quad as u8)
                // .adsize()
                // .bits(QspiSize::ThreeBytes as u8)
                .instruction()
                .bits(Command::Erase64KbyteBlock as u8)
        });

        // qspi.ar.write(|w| unsafe { w.bits(addr) });

        while qspi.sr.read().busy().bit_is_set() {
            asm::nop();
        }

        wait_busy();

        Ok(())
    }

    fn program_page(&mut self, addr: u32, size: u32, data: *const u8) -> Result<(), ErrorCode> {
        qpi_command(Command::WriteEnable);
        let qspi = unsafe { &(*QUADSPI::ptr()) };
        let data = unsafe { core::slice::from_raw_parts(data, size as usize) };
        assert!(!data.is_empty());

        qspi.dlr
            .write(|w| unsafe { w.dl().bits(data.len() as u32 - 1) });

        qspi.abr.write(|w| unsafe { w.bits(addr) });

        // TODO: Why doesn't this work with address bytes?
        qspi.ccr.write(|w| unsafe {
            w.fmode()
                .bits(QspiMode::IndirectWrite as u8)
                .imode()
                .bits(QspiWidth::Quad as u8)
                .dmode()
                .bits(QspiWidth::Quad as u8)
                .abmode()
                .bits(QspiWidth::Quad as u8)
                .absize()
                .bits(QspiSize::ThreeBytes as u8)
                // .admode()
                // .bits(QspiWidth::Quad as u8)
                // .adsize()
                // .bits(QspiSize::ThreeBytes as u8)
                .instruction()
                .bits(Command::PageProgram as u8)
        });

        //qspi.ar.write(|w| unsafe { w.bits(addr) });

        for byte in data {
            while qspi.sr.read().ftf().bit_is_clear() {
                asm::nop();
            }
            unsafe {
                core::ptr::write_volatile(&qspi.dr as *const _ as *mut u8, *byte);
            }
        }

        while qspi.sr.read().busy().bit_is_set() {
            asm::nop();
        }

        wait_busy();

        Ok(())
    }
}

impl Drop for NumWorksAlgo {
    fn drop(&mut self) {
        qpi_command(Command::DisableQPI);
    }
}

fn send_spi_command(command: Command, data: Option<u8>) {
    let qspi = unsafe { &(*QUADSPI::ptr()) };
    qspi.dlr.reset();

    if let Some(data) = data {
        qspi.abr.write(|w| unsafe { w.bits(u32::from(data)) });
    }

    qspi.ccr.write(|w| unsafe {
        w.fmode()
            .bits(QspiMode::IndirectWrite as u8)
            .imode()
            .bits(QspiWidth::Single as u8)
            .instruction()
            .bits(command as u8);

        if data.is_some() {
            w.abmode()
                .bits(QspiWidth::Single as u8)
                .absize()
                .bits(QspiSize::OneByte as u8);
        }

        w
    });

    while qspi.sr.read().busy().bit_is_set() {
        asm::nop();
    }
}

fn qpi_command(command: Command) {
    let qspi = unsafe { &(*QUADSPI::ptr()) };
    qspi.ccr.write(|w| unsafe {
        w.fmode()
            .bits(QspiMode::IndirectWrite as u8)
            .imode()
            .bits(QspiWidth::Quad as u8)
            .instruction()
            .bits(command as u8)
    });

    while qspi.sr.read().busy().bit_is_set() {
        asm::nop();
    }
}

fn wait_busy() {
    while {
        let qspi = unsafe { &(*QUADSPI::ptr()) };
        qspi.dlr.write(|w| unsafe { w.dl().bits(1 - 1) });

        qspi.ccr.write(|w| unsafe {
            w.fmode()
                .bits(QspiMode::IndirectRead as u8)
                .imode()
                .bits(QspiWidth::Quad as u8)
                .dmode()
                .bits(QspiWidth::Quad as u8)
                .instruction()
                .bits(Command::ReadStatusRegister1 as u8)
        });

        let data = qspi.dr.read().bits();

        while qspi.sr.read().busy().bit_is_set() {
            asm::nop();
        }

        data as u8
    } & 0x01
        != 0
    {
        asm::nop();
    }
}
