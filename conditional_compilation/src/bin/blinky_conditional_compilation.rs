#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    #[cfg(board="stm32h7")]
    let config = Default::default();

    #[cfg(board="stm32h7rs")]
    let config = {
        let mut config = Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.hse = Some(Hse {
                freq: Hertz(24_000_000),
                mode: HseMode::Oscillator,
            });
            config.rcc.pll1 = Some(Pll {
                source: PllSource::HSE,
                prediv: PllPreDiv::DIV3,
                mul: PllMul::MUL150,
                divp: Some(PllDiv::DIV2),
                divq: None,
                divr: None,
            });
            config.rcc.sys = Sysclk::PLL1_P; // 600 Mhz
            config.rcc.ahb_pre = AHBPrescaler::DIV2; // 300 Mhz
            config.rcc.apb1_pre = APBPrescaler::DIV2; // 150 Mhz
            config.rcc.apb2_pre = APBPrescaler::DIV2; // 150 Mhz
            config.rcc.apb4_pre = APBPrescaler::DIV2; // 150 Mhz
            config.rcc.apb5_pre = APBPrescaler::DIV2; // 150 Mhz
            config.rcc.voltage_scale = VoltageScale::HIGH;
        }
        config
    };

    let p = embassy_stm32::init(config);
    info!("Hello World!");

    #[cfg(board="stm32h7")]
    let led_pin = p.PB14;

    #[cfg(board="stm32h7rs")]
    let led_pin = p.PD10;

    let mut led = Output::new(led_pin, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(500).await;

        info!("low");
        led.set_low();
        Timer::after_millis(500).await;
    }
}
