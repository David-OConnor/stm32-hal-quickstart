//! This module contains initialization code, run once at program start.

use cortex_m::peripheral::NVIC;
use critical_section::with;

use hal::{
    clocks::Clocks,
    dma::{self, Dma},
    flash::Flash,
    iwdg, pac,
    timer::{Timer, TimerInterrupt},
};

use crate::{setup, Config, CONFIG, FLASH, TICK_TIMER, TICK_TIMER_PERIOD};

const MAIN_LOOP_FREQ: f32 = 2.;

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
    // todo: Enable
    // iwdg::setup(0.1);

    with(|cs| {
        FLASH.borrow(cs).replace(Some(flash));

        // USB_DEV.borrow(cs).replace(usb_dev);
        // USB_SERIAL.borrow(cs).replace(Some(usb_serial));

        TICK_TIMER.borrow(cs).replace(Some(tick_timer));
        CONFIG.borrow(cs).replace(Some(config));
    });

    unsafe {
        // NVIC::unmask(pac::Interrupt::USB_LP);
        NVIC::unmask(pac::Interrupt::TIM1_CC);
        NVIC::unmask(pac::Interrupt::TIM2);

        // Set interrupt priority. See the reference manual's NVIC section for details.
        // Lower value is higher priority.
        // cp.NVIC.set_priority(pac::Interrupt::USB_LP, 2);
        cp.NVIC.set_priority(pac::Interrupt::TIM15, 8); // Tick
        cp.NVIC.set_priority(pac::Interrupt::TIM2, 7); // Main loop
    }
}
