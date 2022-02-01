//! # Thermostat_EEM Firmware
//!
//! Hardware specific setup etc.

pub use stm32h7xx_hal as hal;

pub mod setup;
// pub mod system_timer;

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
