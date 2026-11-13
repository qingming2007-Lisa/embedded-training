//! Complete the two TODOs in this file.

use embassy_stm32::{
    Peripherals,
    can::Can,
    gpio::{Output, Flex, Level, Speed}
};

use crate::swd::Swd;

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<embassy_stm32::usart::BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // 使用 P11 接口上的引脚
    // SWCLK = PE13, SWDIO = PE9 (图中的 PE09)
    let swclk = Output::new(p.PE13, Level::Low, Speed::VeryHigh);
    let swdio = Flex::new(p.PE9);
    let swd = Some(Swd::new(swclk, swdio));

    // 为了不冲突，暂时关闭 UART 自测
    let uart = None;

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart }, swd)
} 