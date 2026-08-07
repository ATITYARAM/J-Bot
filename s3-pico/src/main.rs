#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
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

    //--------------------------------------------------
    // ESP
    //--------------------------------------------------

    let peripherals =
        esp_hal::init(
            esp_hal::Config::default(),
        );

    println!("==============================");
    println!("J-BOT Servo Firmware");
    println!("==============================");

    //--------------------------------------------------
    // USB
    //--------------------------------------------------

    let mut usb =
        UsbSerialJtag::new(
            peripherals.USB_DEVICE,
        );

    //--------------------------------------------------
    // MCPWM
    //--------------------------------------------------

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

    //--------------------------------------------------
    // 50Hz Servo
    //--------------------------------------------------

    let timer =
        clock
            .timer_clock_with_frequency(
                20_000,
                PwmWorkingMode::Increase,
                Rate::from_hz(50),
            )
            .unwrap();

    mcpwm.timer0.start(timer);

    mcpwm.operator0.set_timer(&mcpwm.timer0);

    let mut servo =
        mcpwm
            .operator0
            .with_pin_a(
                peripherals.GPIO8,
                PwmPinConfig::UP_ACTIVE_HIGH,
            );

    //--------------------------------------------------
    // Home
    //--------------------------------------------------

    servo.set_timestamp(angle_to_pulse(90));

    println!("Servo Home");

    //--------------------------------------------------
    // Buffer
    //--------------------------------------------------

    let mut buffer = [0u8; 32];
    let mut index = 0usize;

    //--------------------------------------------------
    // Loop
    //--------------------------------------------------

    loop {

        if let Ok(byte) = usb.read_byte() {

            if byte == b'\n' || byte == b'\r' {

                if index > 0 {

                    if let Ok(packet) =
                        core::str::from_utf8(
                            &buffer[..index],
                        )
                    {
                        println!("RX -> {}", packet);

                        if let Some(angle) =
                            parse_angle(packet)
                        {
                            servo.set_timestamp(
                                angle_to_pulse(angle),
                            );

                            println!(
                                "Servo -> {}",
                                angle,
                            );
                        }
                    }

                    index = 0;
                }
            }
            else {

                if index < buffer.len() {
                    buffer[index] = byte;
                    index += 1;
                }
            }
        }
    }
}

fn parse_angle(
    packet: &str,
) -> Option<u16> {

    packet
        .strip_prefix("T:")
        .and_then(|v| v.trim().parse::<u16>().ok())
}

fn angle_to_pulse(
    angle: u16,
) -> u16 {

    let angle =
        angle.clamp(75, 105);

    1333
        + (((angle - 75) as u32 * 334) / 30) as u16
}
