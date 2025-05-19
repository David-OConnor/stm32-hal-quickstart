//! This module contains hardware configuration setup, eg GPIO, DMA, and IO.

use hal::{
    dma::{self, DmaChannel, DmaInput, DmaPeriph},
    gpio::{Pin, PinMode, Port},
    pac::{SPI1, USART2},
    spi::Spi,
    usart::Usart,
};

// Example pattern for managing DMA channels and peripherals. Set up A/R for your application.
pub const SPI1_DMA_PERIPH: DmaPeriph = DmaPeriph::Dma1;
pub const UART1_DMA_PERIPH: DmaPeriph = DmaPeriph::Dma1;

// An example SPI peripheral.
pub const SPI_TX_CH: DmaChannel = DmaChannel::C1;
pub const SPI_RX_CH: DmaChannel = DmaChannel::C2;

// An example UART peripheral.
pub const UART_TX_CH: DmaChannel = DmaChannel::C3;
pub const UART_RX_CH: DmaChannel = DmaChannel::C4;

pub type SpiPeriph1 = Spi<SPI1>;
pub type UartPeriph1 = Usart<USART2>;

/// Set this up as-required for your project.
pub fn setup_pins() {
    let _sck1 = Pin::new(Port::A, 5, PinMode::Alt(5));
    let _miso1 = Pin::new(Port::A, 6, PinMode::Alt(5));
    let _mosi1 = Pin::new(Port::A, 7, PinMode::Alt(5));

    let mut cs = Pin::new(Port::A, 4, PinMode::Output);
    cs.set_high();

    // let mut can_rx = Pin::new(Port::A, 11, PinMode::Alt(9));
    // let mut can_tx = Pin::new(Port::B, 9, PinMode::Alt(9));
    //
    let _uart1_tx = Pin::new(Port::A, 2, PinMode::Alt(7));
    let _uart1_rx = Pin::new(Port::A, 3, PinMode::Alt(7));
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

    dma::mux(SPI1_DMA_PERIPH, SPI_TX_CH, DmaInput::Spi1Tx);
    dma::mux(SPI1_DMA_PERIPH, SPI_RX_CH, DmaInput::Spi1Rx);

    dma::mux(UART1_DMA_PERIPH, SPI_RX_CH, DmaInput::Usart1Tx);
    dma::mux(UART1_DMA_PERIPH, SPI_RX_CH, DmaInput::Usart1Rx);
}
