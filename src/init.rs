//! This module contains initialization code, run once at program start.

use critical_section::with;
use hal::{
    clocks::Clocks,
    dma::Dma,
    flash::Flash,
    init_globals, iwdg, pac, setup_nvic,
    spi::{BaudRate, Spi, SpiConfig, SpiMode},
    timer::{Timer, TimerInterrupt},
};

use crate::{CONFIG, Config, FLASH, SPI1, setup};

const TICK_TIMER_PERIOD: f32 = 0.5; // in seconds. Decrease for higher measurement precision.
const MAIN_LOOP_FREQ: f32 = 4.;

pub fn run() {
    // Set up ARM Cortex-M peripherals. These are common to many MCUs, including all STM32 ones.
    let mut cp = cortex_m::Peripherals::take().unwrap();
    // Set up peripherals specific to the microcontroller you're using.
    let dp = pac::Peripherals::take().unwrap();

    // Create an initial clock configuration that uses the MCU's internal oscillator (HSI),
    // sets the MCU to its maximum system clock speed.
    let clock_cfg = Clocks {
        hsi48_on: true,
        ..Default::default()
    };

    clock_cfg.setup().unwrap();

    let mut flash = Flash::new(dp.FLASH);
    let mut config = Config::load(&mut flash);

    // An example peripheral; configure as-required based on your applicaiton.
    let spi_cfg = SpiConfig {
        mode: SpiMode::mode3(),
        ..Default::default()
    };

    let spi = Spi::new(dp.SPI1, spi_cfg, BaudRate::Div8);

    // We use this timer to maintain a time since bootup.
    // A shorter timeout period will allow higher resolution measurements, while a longer one
    // will command an interrupt less often. (The interrupt only increments an atomic overflow counter).
    let mut tick_timer = Timer::new_tim15(
        dp.TIM15,
        1. / TICK_TIMER_PERIOD,
        Default::default(),
        &clock_cfg,
    );

    tick_timer.enable_interrupt(TimerInterrupt::Update);

    let mut main_loop_timer =
        Timer::new_tim2(dp.TIM2, MAIN_LOOP_FREQ, Default::default(), &clock_cfg);
    main_loop_timer.enable_interrupt(TimerInterrupt::Update);

    let _dma = Dma::new(dp.DMA1);

    setup::setup_pins();
    setup::setup_dma();

    // Enable the watchdog with a 0.1s timeout.
    iwdg::setup(0.1).unwrap();

    init_globals!(
        (FLASH, flash),
        (CONFIG, config),
        (SPI1, spi),
        // (USB_DEV, usb_dev),
        // (USB_SERIAL, usb_serial),
    );

    // Unmask interrupt lines, and set their priority using Cortex-M's NVIC peripheral.
    // Lower nubmers are higher priority.
    setup_nvic!(
        [
            (TIM3, 8),
            (TIM2, 7),
            // (SPI1, 2),
            // (USB_LP, 2),
        ],
        cp
    );
}
