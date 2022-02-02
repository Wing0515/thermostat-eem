//! # Thermostat_EEM Firmware
//!
//! Hardware specific setup etc.

pub use stm32h7xx_hal as hal;

pub mod setup;
pub mod system_timer;

// Thermostat MAC definition
const SRC_MAC: [u8; 6] = [0x80, 0x1f, 0x12, 0x63, 0x84, 0x1b];

// Number of TX descriptors in the ethernet descriptor ring.
const TX_DESRING_CNT: usize = 4;

// Number of RX descriptors in the ethernet descriptor ring.
const RX_DESRING_CNT: usize = 4;

pub type NetworkStack = smoltcp_nal::NetworkStack<
    'static,
    hal::ethernet::EthernetDMA<'static, TX_DESRING_CNT, RX_DESRING_CNT>,
    system_timer::SystemTimer,
>;

pub type NetworkManager = smoltcp_nal::shared::NetworkManager<
    'static,
    hal::ethernet::EthernetDMA<'static, TX_DESRING_CNT, RX_DESRING_CNT>,
    system_timer::SystemTimer,
>;

pub type EthernetPhy = hal::ethernet::phy::LAN8742A<hal::ethernet::EthernetMAC>;

// Front LEDs.
pub struct LEDs {
    led0: hal::gpio::gpiog::PG9<hal::gpio::Output<hal::gpio::PushPull>>,
    led1: hal::gpio::gpiog::PG10<hal::gpio::Output<hal::gpio::PushPull>>,
    led2: hal::gpio::gpioe::PE8<hal::gpio::Output<hal::gpio::PushPull>>,
    led3: hal::gpio::gpioe::PE10<hal::gpio::Output<hal::gpio::PushPull>>,
    led4: hal::gpio::gpioe::PE12<hal::gpio::Output<hal::gpio::PushPull>>,
    led5: hal::gpio::gpiog::PG15<hal::gpio::Output<hal::gpio::PushPull>>,
    led6: hal::gpio::gpioe::PE15<hal::gpio::Output<hal::gpio::PushPull>>,
    led7: hal::gpio::gpiog::PG8<hal::gpio::Output<hal::gpio::PushPull>>,
}
