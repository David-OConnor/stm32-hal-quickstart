//! This module contains initialization code, run once at program start.

use hal::{
    clocks::Clocks,
    dma::{self, Dma},
    gpio::{Pin, PinMode, Port},
    iwdg, pac,
    timer::{Timer, TimerInterrupt},
};

use crate::setup;

const TICK_TIMER_PERIOD: f32 = 0.5; // in seconds. Decrease for higher measurement precision.
const MAIN_LOOP_FREQ: f32 = 4.;

pub fn run() {
    // Set up ARM Cortex-M peripherals. These are common to many MCUs, including all STM32 ones.
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up peripherals specific to the microcontroller you're using.
    let dp = pac::Peripherals::take().unwrap();

    // Create an initial clock configuration that uses the MCU's internal oscillator (HSI),
    // sets the MCU to its maximum system clock speed.
    let clock_cfg = Clocks::default();

    clock_cfg.setup().unwrap();

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
    iwdg::setup(0.1);
}
