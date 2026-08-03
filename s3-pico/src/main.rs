#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output},
    ledc::{
        self,
        channel::{ChannelHW, ChannelIFace},
        timer::{self, TimerIFace},
        Ledc,
        LSGlobalClkSource,
        LowSpeed,
    },
};

use fugit::RateExtU32;
use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    // ---------------------------------
    // ESP Initialization
    // ---------------------------------

    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let mut delay = Delay::new();

    println!("J-BOT Motor Test");

    // ---------------------------------
    // Direction Pin
    // GPIO5
    // ---------------------------------

    let mut dir = Output::new(peripherals.GPIO5, Level::Low);

    // Forward

    dir.set_low();

    // ---------------------------------
    // LEDC PWM
    // GPIO4
    // ---------------------------------

    let mut ledc = Ledc::new(peripherals.LEDC);

    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);

    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty12Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 5.kHz(),
        })
        .unwrap();

    let mut pwm =
        ledc.channel(ledc::channel::Number::Channel0, peripherals.GPIO4);

    pwm.configure(ledc::channel::config::Config {
        timer: &timer,
        duty_pct: 0,
        pin_config: ledc::channel::config::PinConfig::PushPull,
    })
    .unwrap();

    // ---------------------------------
    // 50% Duty
    // ---------------------------------

    pwm.set_duty_hw(2048);

    println!("Motor Running...");

    loop {
        delay.delay_ms(1000);
    }
}
