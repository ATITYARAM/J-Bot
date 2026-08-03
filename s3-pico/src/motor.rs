use esp_hal::{
    gpio::{Level, Output},
    ledc::{
        self,
        channel::{ChannelHW, ChannelIFace},
        timer::{self, TimerIFace},
        Ledc,
        LSGlobalClkSource,
        LowSpeed,
    },
    peripherals,
};

use fugit::RateExtU32;

pub struct Motor<'d> {
    left_dir: Output<'d>,
    right_dir: Output<'d>,

    left_pwm:
        ledc::channel::Channel<'d, LowSpeed, ledc::channel::Number::Channel0>,

    right_pwm:
        ledc::channel::Channel<'d, LowSpeed, ledc::channel::Number::Channel1>,
}

impl<'d> Motor<'d> {
    pub fn new(
        ledc_peripheral: peripherals::LEDC,
        gpio4: peripherals::GPIO4,
        gpio5: peripherals::GPIO5,
        gpio6: peripherals::GPIO6,
        gpio7: peripherals::GPIO7,
    ) -> Self {
        let left_dir = Output::new(gpio5, Level::Low);
        let right_dir = Output::new(gpio7, Level::Low);

        let mut ledc = Ledc::new(ledc_peripheral);

        ledc.set_global_slow_clock(
            LSGlobalClkSource::APBClk,
        );

        let mut timer =
            ledc.timer::<LowSpeed>(timer::Number::Timer0);

        timer.configure(timer::config::Config {
            duty: timer::config::Duty::Duty12Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 5.kHz(),
        })
        .unwrap();

        let mut left_pwm =
            ledc.channel(
                ledc::channel::Number::Channel0,
                gpio4,
            );

        left_pwm
            .configure(
                ledc::channel::config::Config {
                    timer: &timer,
                    duty_pct: 0,
                    pin_config:
                        ledc::channel::config::PinConfig::PushPull,
                },
            )
            .unwrap();

        let mut right_pwm =
            ledc.channel(
                ledc::channel::Number::Channel1,
                gpio6,
            );

        right_pwm
            .configure(
                ledc::channel::config::Config {
                    timer: &timer,
                    duty_pct: 0,
                    pin_config:
                        ledc::channel::config::PinConfig::PushPull,
                },
            )
            .unwrap();

        Self {
            left_dir,
            right_dir,
            left_pwm,
            right_pwm,
        }
    }

    pub fn stop(&mut self) {
        self.left_pwm.set_duty_hw(0);
        self.right_pwm.set_duty_hw(0);
    }

    pub fn set_left(&mut self, speed: i16) {
        let value = speed.clamp(-255, 255);

        if value >= 0 {
            self.left_dir.set_low();
        } else {
            self.left_dir.set_high();
        }

        let duty =
            (value.abs() as u32 * 4095 / 255) as u32;

        self.left_pwm.set_duty_hw(duty);
    }

    pub fn set_right(&mut self, speed: i16) {
        let value = speed.clamp(-255, 255);

        if value >= 0 {
            self.right_dir.set_low();
        } else {
            self.right_dir.set_high();
        }

        let duty =
            (value.abs() as u32 * 4095 / 255) as u32;

        self.right_pwm.set_duty_hw(duty);
    }

    pub fn set(
        &mut self,
        left: i16,
        right: i16,
    ) {
        self.set_left(left);
        self.set_right(right);
    }
}
