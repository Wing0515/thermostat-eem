// Thermostat ADC struct.

use num_enum::TryFromPrimitive;
use shared_bus_rtic::SharedBus;

use super::ad7172::{Ad7172, AdcReg, Adcmode, Channel, Filtcon, Ifmode, Setupcon};

use super::hal::{
    gpio::{gpioe::*, Alternate, Output, PushPull, AF5},
    hal::blocking::delay::DelayUs,
    hal::digital::v2::OutputPin,
    prelude::*,
    rcc::{rec, CoreClocks},
    spi,
    spi::{Enabled, Spi},
    stm32::SPI4,
};

#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum InputChannel {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

type SB = SharedBus<Spi<SPI4, Enabled>>;
type O = Output<PushPull>;
type Adcs = (
    Ad7172<SB, PE0<O>>,
    Ad7172<SB, PE1<O>>,
    Ad7172<SB, PE3<O>>,
    Ad7172<SB, PE4<O>>,
);
type SpiPins = (
    PE2<Alternate<AF5>>,
    PE5<Alternate<AF5>>,
    PE6<Alternate<AF5>>,
);

pub struct AdcPins {
    pub cs: (PE0<O>, PE1<O>, PE3<O>, PE4<O>),
}

pub struct Adc {
    pub adcs: Adcs,
}

impl Adc {
    /// Construct a new ADC driver for all Thermostat input channels.
    ///
    /// # Args
    /// * `clocks` - Reference to CoreClocks
    /// * `spi4_rec` - Peripheral Reset and Enable Control for SPI4
    /// * `spi4` - SPI4 peripheral
    /// * `sck` - Spi sck pin
    /// * `miso` - Spi miso pin
    /// * `mosi` - Spi mosi pin
    /// * `pins` - ADC chip select pins.
    pub fn new(
        delay: &mut impl DelayUs<u16>,
        clocks: &CoreClocks,
        spi4_rec: rec::Spi4,
        spi4: SPI4,
        spi_pins: SpiPins,
        mut pins: AdcPins,
    ) -> Self {
        // set all CS high first
        pins.cs.0.set_high().unwrap();
        pins.cs.1.set_high().unwrap();
        pins.cs.2.set_high().unwrap();
        pins.cs.3.set_high().unwrap();

        // SPI at 1 MHz. SPI MODE_0: idle low, capture on first transition
        let spi: Spi<_, _, u8> = spi4.spi(
            (spi_pins.0, spi_pins.1, spi_pins.2),
            spi::MODE_0,
            1.mhz(),
            spi4_rec,
            clocks,
        );

        let bus_manager = shared_bus_rtic::new!(spi, Spi<SPI4, Enabled>);

        let mut adc = Adc {
            adcs: (
                Ad7172::new(delay, bus_manager.acquire(), pins.cs.0).unwrap(),
                Ad7172::new(delay, bus_manager.acquire(), pins.cs.1).unwrap(),
                Ad7172::new(delay, bus_manager.acquire(), pins.cs.2).unwrap(),
                Ad7172::new(delay, bus_manager.acquire(), pins.cs.3).unwrap(),
            ),
        };

        Adc::setup_adc(&mut adc.adcs.0);
        Adc::setup_adc(&mut adc.adcs.1);
        Adc::setup_adc(&mut adc.adcs.2);
        Adc::setup_adc(&mut adc.adcs.3);

        adc
    }

    /// Setup an adc on Thermostat-EEM.
    fn setup_adc<CS>(adc: &mut Ad7172<SB, CS>)
    where
        CS: OutputPin,
        <CS>::Error: core::fmt::Debug,
    {
        // Setup ADCMODE register. Internal reference, internal clock, no delay, continuous conversion.
        adc.write(
            AdcReg::ADCMODE,
            Adcmode::RefEn::ENABLED
                | Adcmode::Mode::CONTINOUS_CONVERSION
                | Adcmode::Clocksel::EXTERNAL_CLOCK,
        );

        // Setup IFMODE register. Only enable data stat to get channel info on conversions.
        adc.write(AdcReg::IFMODE, Ifmode::DataStat::ENABLED);

        // enable first channel and configure Ain0, Ain1,
        // set config 0 for first channel.
        adc.write(
            AdcReg::CH0,
            Channel::SetupSel::SETUP_0 | Channel::Ainpos::AIN0 | Channel::Ainneg::AIN1,
        );

        // enable second channel and configure Ain2, Ain3,
        // set config 0 for second channel too.
        adc.write(
            AdcReg::CH1,
            Channel::SetupSel::SETUP_0 | Channel::Ainpos::AIN2 | Channel::Ainneg::AIN3,
        );

        // Setup firstconfiguration register
        adc.write(
            AdcReg::SETUPCON0,
            Setupcon::BiUnipolar::UNIPOLAR
                | Setupcon::Refbufn::ENABLED
                | Setupcon::Refbufp::ENABLED
                | Setupcon::Ainbufn::ENABLED
                | Setupcon::Ainbufp::ENABLED
                | Setupcon::Refsel::EXTERNAL,
        );

        // Setup first filter configuration register. 10Hz data rate. Sinc5Sinc1 Filter. No postfilter.
        adc.write(
            AdcReg::FILTCON0,
            Filtcon::Order::SINC5SINC1 | Filtcon::Odr::ODR_10,
        );
    }
}
