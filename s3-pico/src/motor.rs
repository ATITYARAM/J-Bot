#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    mcpwm::{
        operator::PwmPinConfig,
        timer::PwmWorkingMode,
        McPwm,
        PeripheralClockConfig,
    },
    time::Rate,
};
use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    //--------------------------------------------------
    // Initialize
    //--------------------------------------------------

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut delay = Delay::new();

    println!("==============================");
    println!("J-BOT Servo Tilt Test");
    println!("GPIO8");
    println!("==============================");

    //--------------------------------------------------
    // MCPWM
    //--------------------------------------------------

    let clock_cfg =
        PeripheralClockConfig::with_frequency(
            Rate::from_mhz(40),
        )
        .unwrap();

    let mut mcpwm =
        McPwm::new(
            peripherals.MCPWM0,
            clock_cfg,
        );

    //--------------------------------------------------
    // Timer 0
    //--------------------------------------------------

    let timer_cfg =
        clock_cfg
            .timer_clock_with_frequency(
                20_000,
                PwmWorkingMode::Increase,
                Rate::from_hz(50),
            )
            .unwrap();

    mcpwm.timer0.start(timer_cfg);

    //--------------------------------------------------
    // Operator 0 -> GPIO8
    //--------------------------------------------------

    mcpwm.operator0.set_timer(&mcpwm.timer0);

    let mut servo =
        mcpwm
            .operator0
            .with_pin_a(
                peripherals.GPIO8,
                PwmPinConfig::UP_ACTIVE_HIGH,
            );

    //--------------------------------------------------
    // Pulse Widths (microseconds)
    //--------------------------------------------------

    const HALT: u16 = 500;   //   0°
    const HOME: u16 = 1500;  //  90°
    const LEFT: u16 = 1667;  // 105°
    const RIGHT: u16 = 1333; //  75°

    //--------------------------------------------------
    // Initial Halt
    //--------------------------------------------------

    println!("Servo -> Halt (0°)");

    servo.set_timestamp(HALT);

    delay.delay_ms(2000);

    //--------------------------------------------------
    // Loop
    //--------------------------------------------------

    loop {
        println!("Servo -> Home (90°)");
        servo.set_timestamp(HOME);
        delay.delay_ms(1000);

        println!("Servo -> Left (105°)");
        servo.set_timestamp(LEFT);
        delay.delay_ms(1000);

        println!("Servo -> Home (90°)");
        servo.set_timestamp(HOME);
        delay.delay_ms(1000);

        println!("Servo -> Right (75°)");
        servo.set_timestamp(RIGHT);
        delay.delay_ms(1000);

        println!("Servo -> Home (90°)");
        servo.set_timestamp(HOME);
        delay.delay_ms(1000);
    }
}
