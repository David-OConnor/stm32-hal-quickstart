//! This example shows a template complete project, including file structure, and config
//! needed to flash using an ST-Link, or USB. It demonstrates initialization of peripherals,
//! with a minimal project structure. It demonstrates an overview of this library's features.

//! See the syntax example in the main STM32-HAL repo for a more detailed example.

#![no_main]
#![no_std]

use core::sync::atomic::Ordering;

use cortex_m;
use cortex_m_rt::entry;
use critical_section::with;
use defmt::println;
// These lines are part of our setup for debug printing.
use defmt_rtt as _;
use hal::spi::Spi;
// Import parts of this library we use. You could use this style, or perhaps import
// less here.
use hal::{
    self, access_global,
    flash::{Bank, Flash},
    gpio::{self, Edge}, low_power, make_globals,
    pac::{SPI1, interrupt},
    timer::{self, TICK_OVERFLOW_COUNT},
};
use panic_probe as _;

mod init;
mod setup;
mod system_status;

// Use this macro to manage global state, controlled with critical section mutexes.
// Note: There is nothing STM-32-specific about this; if you wish to use this
// pattern on other MCUs, copy+paste these macros from the HAL's `lib.rs` into
// your firmware directly.
make_globals!(
    (FLASH, Flash),
    (SPI1, Spi<SPI1>),
    // (USB_DEV, UsbDevice<'static, UsbBusType>),
    // (USB_SERIAL, SerialPort<'static, UsbBusType>),
    (CONFIG, Config),
);

// Adjust this based on your MCU's flash, and storage requirements.
const FLASH_PAGE_ONBOARD: usize = 63;
const CONFIG_SIZE: usize = 1;

/// Used to store application configuration data to and from flash memory.
pub struct Config {}

impl Config {
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }

    fn to_bytes(&self) -> [u8; CONFIG_SIZE] {
        let mut result = [0; CONFIG_SIZE];
        // Update here based on your data.

        result
    }

    pub fn save(&self, flash: &mut Flash) {
        if flash.erase_page(Bank::B1, FLASH_PAGE_ONBOARD).is_err() {
            println!("Error erasing flash");
        }
        if let Err(e) = flash.write_page(Bank::B1, FLASH_PAGE_ONBOARD, &self.to_bytes()) {
            println!("Error writing flash");
        }
    }

    pub fn load(flash: &mut Flash) -> Self {
        let mut buf = [0; CONFIG_SIZE];
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

    with(|cs| {
        access_global!(CONFIG, config, cs);

        // We now have access to our global config struct in this ISR's
        // context, as the `config` variable.
    });
}

#[interrupt]
/// Increments the tick overflow.
fn TIM3() {
    timer::clear_update_interrupt(3);
    TICK_OVERFLOW_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[interrupt]
/// An EXTI interrupt. (e.g. interrupt pin)
fn EXTI1() {
    gpio::clear_exti_interrupt(1, Edge::Falling);

    // Take action as required, e.g. if a button is pushed.
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
