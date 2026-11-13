//! Complete the two TODOs in this file.

use embassy_stm32::{
    Peripherals,
    can::Can,
    gpio::{Output, Flex, Level, Speed},
    usart::{BufferedUart, Config as UartConfig, BufferedInterruptHandler},
    bind_interrupts,
    peripherals::UART7
};
use static_cell::StaticCell;

use crate::swd::Swd;

bind_interrupts!(struct Irqs {
    UART7 => BufferedInterruptHandler<UART7>;
});

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // SWD: 使用 P11 的 PE13 和 PE9
    let swclk = Output::new(p.PE13, Level::Low, Speed::VeryHigh);
    let swdio = Flex::new(p.PE9);
    let swd = Some(Swd::new(swclk, swdio));

    // UART7: 使用图中的 PE7(RX) 和 PE8(TX)
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 115_200;
    
    static TX_BUF: StaticCell<[u8; 1024]> = StaticCell::new();
    static RX_BUF: StaticCell<[u8; 1024]> = StaticCell::new();
    let tx_buf = TX_BUF.init([0; 1024]);
    let rx_buf = RX_BUF.init([0; 1024]);

    let uart = BufferedUart::new(
        p.UART7,
        p.PE7, // RX
        p.PE8, // TX
        rx_buf,
        tx_buf,
        Irqs,
        uart_config,
    ).unwrap();

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart: Some(uart) }, swd)
}