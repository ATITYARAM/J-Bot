#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    mcpwm::{
        operator::PwmPinConfig,
        timer::PwmWorkingMode,
        McPwm,
        PeripheralClockConfig,
    },
    time::Rate,
    usb_serial_jtag::UsbSerialJtag,
};
use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    //----------------------------------------------------
    // ESP
    //----------------------------------------------------

    let config =
        esp_hal::Config::default()
            .with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    println!("J-BOT MCPWM Driver");

    //----------------------------------------------------
    // USB
    //----------------------------------------------------

    let mut usb =
        UsbSerialJtag::new(peripherals.USB_DEVICE);

    //----------------------------------------------------
    // Direction Pins
    //----------------------------------------------------

    let mut left_dir =
        Output::new(
            peripherals.GPIO5,
            Level::High,
            OutputConfig::default(),
        );

    let mut right_dir =
        Output::new(
            peripherals.GPIO7,
            Level::High,
            OutputConfig::default(),
        );

    //----------------------------------------------------
    // MCPWM
    //----------------------------------------------------

    let clock =
        PeripheralClockConfig::with_frequency(
            Rate::from_mhz(40),
        )
        .unwrap();

    let mut mcpwm =
        McPwm::new(
            peripherals.MCPWM0,
            clock,
        );

    //----------------------------------------------------
    // LEFT PWM
    //----------------------------------------------------

    mcpwm.operator0.set_timer(&mcpwm.timer0);

    let mut left_pwm =
        mcpwm.operator0.with_pin_a(
            peripherals.GPIO4,
            PwmPinConfig::UP_ACTIVE_HIGH,
        );

    //----------------------------------------------------
    // RIGHT PWM
    //----------------------------------------------------

    mcpwm.operator1.set_timer(&mcpwm.timer1);

    let mut right_pwm =
        mcpwm.operator1.with_pin_a(
            peripherals.GPIO6,
            PwmPinConfig::UP_ACTIVE_HIGH,
        );

    //----------------------------------------------------
    // Timers
    //----------------------------------------------------

    let timer =
        clock
            .timer_clock_with_frequency(
                99,
                PwmWorkingMode::Increase,
                Rate::from_khz(20),
            )
            .unwrap();

    mcpwm.timer0.start(timer);

    mcpwm.timer1.start(timer);

    //----------------------------------------------------
    // Serial Buffer
    //----------------------------------------------------

    let mut buffer = [0u8; 64];
    let mut index = 0usize;

    loop {
        if let Ok(byte) = usb.read_byte() {
            if byte == b'\n' {
                if let Ok(packet) =
                    core::str::from_utf8(&buffer[..index])
                {
                    println!("RX -> {}", packet);

                    if let Some((left, right)) = parse(packet) {

                        // -------------------------
                        // LEFT MOTOR
                        // -------------------------

                        let left_speed = left.clamp(-255, 255);

                        if left_speed >= 0 {
                            left_dir.set_high();
                        } else {
                            left_dir.set_low();
                        }

                        let left_duty =
                            ((left_speed.abs() as u16 * 99) / 255) as u16;

                        left_pwm.set_timestamp(left_duty);

                        // -------------------------
                        // RIGHT MOTOR
                        // -------------------------

                        let right_speed = right.clamp(-255, 255);

                        if right_speed >= 0 {
                            right_dir.set_high();
                        } else {
                            right_dir.set_low();
                        }

                        let right_duty =
                            ((right_speed.abs() as u16 * 99) / 255) as u16;

                        right_pwm.set_timestamp(right_duty);
                    }
                }

                index = 0;
            } else {
                if index < buffer.len() {
                    buffer[index] = byte;
                    index += 1;
                }
            }
        }
    }
}

fn parse(packet: &str) -> Option<(i16, i16)> {
    let mut left = None;
    let mut right = None;

    for part in packet.split_whitespace() {
        if let Some(v) =
            part.strip_prefix("L:")
        {
            left = v.parse::<i16>().ok();
        }

        if let Some(v) =
            part.strip_prefix("R:")
        {
            right = v.parse::<i16>().ok();
        }
    }

    match (left, right) {
        (Some(l), Some(r)) => Some((l, r)),
        _ => None,
    }
}
