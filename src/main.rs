//! This example shows a complete project, including file structure, and config
//! needed to flash using an ST-Link. The project structure is based on
//! [Knurling's app-template](https://github.com/knurling-rs/app-template).
//! This file demonstrates an overview of this library's features.

//! See the syntax example in the main STM32-HAL repo for a more detailed example.

#![no_main]
#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::{self};
use cortex_m_rt::{entry, interrupt};
use defmt::println;
// These lines are part of our setup for debug printing.
use defmt_rtt as _;
// Import parts of this library we use. You could use this style, or perhaps import
// less here.
use hal::{
    self, access_global,
    flash::{Bank, Flash},
    gpio::Pin,
    low_power, make_globals,
    pac::{self, TIM1, TIM2},
    timer,
    timer::Timer,
};
use panic_probe as _;

mod init;
mod setup;
mod system_status;

make_globals!(
    (FLASH, Flash),
    (TICK_TIMER, Timer<TIM3>),
    (MAIN_LOOP_TIMER, Timer<TIM2>),
    // (USB_DEV, UsbDevice<'static, UsbBusType>),
    // (USB_SERIAL, SerialPort<'static, UsbBusType>),
    (CONFIG, Config),
);

// Adjust this based on your MCU's flash, and storage requirements.
const FLASH_PAGE_ONBOARD: usize = 63;

// We use a hardware counter to measure relative system time. This is the number of times
// it has overflowed. (timer expired)
const TICK_TIMER_PERIOD: f32 = 0.5; // in seconds. Decrease for higher measurement precision.
pub static TICK_OVERFLOW_COUNT: AtomicU32 = AtomicU32::new(0);

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

// #[rtic::app(device = pac, peripherals = false)]
// mod app {
//     use super::*;
//
//     #[shared]
//     pub struct Shared {
//         pub config: Config,
//         pub system_status: system_status::SystemStatus,
//     }
//
//     #[local]
//     pub struct Local {}
//
//     #[init]
//     fn init(cx: init::Context) -> (Shared, Local) {
//         crate::init::run(cx)
//     }
//
//     #[idle(shared = [], local = [])]
//     /// In this function, we perform setup code that must occur with interrupts enabled.
//     fn idle(_cx: idle::Context) -> ! {
//         loop {
//             asm::nop();
//         }
//     }
// }

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

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[interrupt]
/// Main loop.
fn TIM2() {
    timer::clear_update_interrupt(2);
    println!("Main loop");
}

#[interrupt]
/// Increments the tick overflow.
fn TIM1() {
    timer::clear_update_interrupt(1);
    TICK_OVERFLOW_COUNT.fetch_add(1, Ordering::Relaxed);
}