//! Complete the two TODOs in this file.

use embassy_stm32::{
    Peripherals,
    can::Can,
    usart::{BufferedUart, Config as UartConfig, BufferedInterruptHandler},
    bind_interrupts,
    peripherals::UART8,
    gpio::{Output, Flex, Level, Speed}
};
use static_cell::StaticCell;

use crate::swd::Swd;

bind_interrupts!(struct Irqs {
    UART8 => BufferedInterruptHandler<UART8>;
});

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: SWCLK = PE14 (P16-13), SWDIO = PE15 (P16-18)
    let swclk = Output::new(p.PE14, Level::Low, Speed::VeryHigh);
    let swdio = Flex::new(p.PE15);
    let swd = Some(Swd::new(swclk, swdio));

    // TODO 2: UART8 115200 8N1
    // RX = PE0 (P16-11), TX = PE1 (P16-12)
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 115_200;
    
    static TX_BUF: StaticCell<[u8; 4096]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 1024]> = StaticCell::new();
    let tx_buf = TX_BUF.init([0; 4096]);
    let rx_buf = RX_BUF.init([0; 1024]);

    let uart = BufferedUart::new(
        p.UART8,
        p.PE0, // RX
        p.PE1, // TX
        rx_buf,
        tx_buf,
        Irqs,
        uart_config,
    ).unwrap();

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart: Some(uart) }, swd)
}