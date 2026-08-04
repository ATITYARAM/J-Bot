#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
};
use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let config = esp_hal::Config::default();
        config
    });

    let mut delayed = Delay::new();
    println!("GPIO 8 Digital Toggle Test Initialized...");

    // Configure GPIO 8 as a standard digital output pin with default configurations
    let mut pin = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    loop {
        println!("GPIO 8: HIGH (3.3V expected)");
        pin.set_high();
        delayed.delay_ms(1000);

        println!("GPIO 8: LOW (0V expected)");
        pin.set_low();
        delayed.delay_ms(1000);
    }
}
