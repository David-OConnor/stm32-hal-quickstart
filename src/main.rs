//! This example shows a complete project, including file structure, and config
//! needed to flash using an ST-Link. The project structure is based on
//! [Knurling's app-template](https://github.com/knurling-rs/app-template).
//! This file demonstrates an overview of this library's features.

//! See the syntax example in the main STM32-HAL repo for a more detailed example.

#![no_main]
#![no_std]

use core::{cell::RefCell, sync::atomic::Ordering};

use cortex_m::{self};
use cortex_m_rt::entry;
use critical_section::{Mutex, with};
use defmt::println;
// These lines are part of our setup for debug printing.
use defmt_rtt as _;
// Import parts of this library we use. You could use this style, or perhaps import
// less here.
use hal::{
    self, access_global,
    flash::{Bank, Flash},
    gpio, low_power, make_globals,
    pac::{self, TIM2, TIM15, interrupt},
    timer::{self, TICK_OVERFLOW_COUNT, Timer},
};
use panic_probe as _;

mod init;
mod setup;
mod system_status;

make_globals!(
    (FLASH, Flash),
    // (SPI_IMU, SpiImu),
    // (USB_DEV, UsbDevice<'static, UsbBusType>),
    // (USB_SERIAL, SerialPort<'static, UsbBusType>),
    (CONFIG, Config),
);

// Adjust this based on your MCU's flash, and storage requirements.
const FLASH_PAGE_ONBOARD: usize = 63;

pub struct Config {}

impl Config {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    pub fn to_bytes(&self) -> [u8; 0] {
        let mut result = [0; 0];
        result
    }

    pub fn save(&self, flash: &mut Flash) {
        flash.erase_page(Bank::B1, FLASH_PAGE_ONBOARD).ok();
        flash
            .write_page(Bank::B1, FLASH_PAGE_ONBOARD, &self.to_bytes())
            .ok();
    }

    pub fn load(flash: &mut Flash) -> Self {
        let mut buf = [0; 0];
        flash.read(Bank::B1, FLASH_PAGE_ONBOARD, 0, &mut buf);

        Self::from_bytes(&buf)
    }
}

#[entry]
fn main() -> ! {
    // This line is required to prevent the debugger from disconnecting on entering WFI.
    // This appears to be a limitation of many STM32 families. Not required in production code,
    // and significantly increases power consumption in low-power modes. Not required if not using WFI.
    // hal::debug_workaround();

    init::run();

    loop {
        // low_power::sleep_now();
        cortex_m::asm::nop();
    }
}

#[interrupt]
/// Main loop.
fn TIM2() {
    timer::clear_update_interrupt(2);
    println!("Main loop timer");
}

#[interrupt]
/// Increments the tick overflow.
fn TIM15() {
    timer::clear_update_interrupt(15);
    TICK_OVERFLOW_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[interrupt]
/// An EXTI interrupt. (e.g. interrupt pin)
fn EXTI1() {
    gpio::clear_exti_interrupt(1);

    with(|cs| {
        // access_global!(SPI_IMU, spi, cs);
    });
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
