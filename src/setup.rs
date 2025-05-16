//! This module contains hardware configuration setup, eg GPIO, DMA, and IO.

use hal::{
    dma::{self, DmaInput, DmaInterrupt},
    gpio::{OutputSpeed, Pin, PinMode, Port},
};

// Example pattern for managing DMA channels and peripherals. Set up A/R for your application.
// pub const IMU_DMA_PERIPH: DmaPeriph = DmaPeriph::Dma1;
// pub const GNSS_DMA_PERIPH: DmaPeriph = DmaPeriph::Dma1;

// pub const IMU_TX_CH: DmaChannel = DmaChannel::C1;
// pub const IMU_RX_CH: DmaChannel = DmaChannel::C2;

// pub const GNSS_TX_CH: DmaChannel = DmaChannel::C3;
// pub const GNSS_RX_CH: DmaChannel = DmaChannel::C4;

// pub type SpiImu = Spi<SPI1>;
// pub type UartGnss = Usart<USART2>;

pub fn setup_pins() {
    // let mut can_rx = Pin::new(Port::A, 11, PinMode::Alt(9));
    // let mut can_tx = Pin::new(Port::B, 9, PinMode::Alt(9));
    //
    // let _uart_gnss_tx = Pin::new(Port::A, 2, PinMode::Alt(7));
    // let _uart_gnss_rx = Pin::new(Port::A, 3, PinMode::Alt(7));
    //
    // can_tx.output_speed(OutputSpeed::VeryHigh);
    // can_rx.output_speed(OutputSpeed::VeryHigh);
    //
    // let mut sda1 = Pin::new(Port::B, 7, PinMode::Alt(4));
    // let mut scl1 = Pin::new(Port::A, 15, PinMode::Alt(4));
    //
    // sda1.output_type(OutputType::OpenDrain);
    // scl1.output_type(OutputType::OpenDrain);
    // sda1.pull(Pull::Up);
    // scl1.pull(Pull::Up);
}

pub fn setup_dma() {
    // dma::enable_mux1(); // Only required on some variants, like G4.

    // dma::mux(IMU_DMA_PERIPH, IMU_TX_CH, DmaInput::Spi1Tx);
    // dma::mux(IMU_DMA_PERIPH, IMU_RX_CH, DmaInput::Spi1Rx);
    //
    // dma::mux(GNSS_DMA_PERIPH, GNSS_TX_CH, DmaInput::Usart2Tx);
    // dma::mux(GNSS_DMA_PERIPH, GNSS_RX_CH, DmaInput::Usart2Rx);
}
