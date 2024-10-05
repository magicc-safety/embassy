#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let green_led = Output::new(p.PB0, Level::High, Speed::Low);
    let yellow_led = Output::new(p.PE1, Level::High, Speed::Low);
    let red_led = Output::new(p.PB14, Level::High, Speed::Low);

    let mut led_array = [green_led, yellow_led, red_led];
    
    loop {
        info!("high");
        for led in led_array.iter_mut() {
            led.set_high();
            Timer::after_millis(100).await;
        }
        // led.set_high();

        info!("low");
        for led in led_array.iter_mut() {
            led.set_low();
            Timer::after_millis(100).await;
        }
        // led.set_low();
    }
}
