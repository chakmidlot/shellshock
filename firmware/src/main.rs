#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

mod accelerometer;
mod display;
mod ring;
mod shake_analyzer;

use crate::accelerometer::Accelerometer;
use crate::display::Display;
use crate::shake_analyzer::ShakeAnalyzer;
use defmt::debug;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{self, Config, InterruptHandler};
use embassy_rp::peripherals::{I2C0, I2C1};
use embassy_time::Instant;
use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::{DisplayRotation, DisplaySize128x64};
use ssd1306::{I2CDisplayInterface, Ssd1306};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs1 {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

bind_interrupts!(struct Irqs0 {
    I2C0_IRQ => InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_2;
    let scl = p.PIN_3;

    let i2c_display = i2c::I2c::new_blocking(&p.I2C1, &scl, &sda, Config::default());
    let i2c_accelerometer =
        i2c::I2c::new_async(&p.I2C0, &p.PIN_5, &p.PIN_4, Irqs0, Config::default());

    let mut accelerometer = Accelerometer::new(i2c_accelerometer);
    accelerometer.init().await.unwrap();

    let interface = I2CDisplayInterface::new(i2c_display);
    let mut ssd1306 = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    ssd1306.init().unwrap();

    let mut display = Display::new(ssd1306);
    let mut shake_analyzer = ShakeAnalyzer::new();

    loop {
        let acc_t = Instant::now();
        let acceleration = accelerometer.read_fifo().await.unwrap();
        for value in acceleration {
            shake_analyzer.add(value.0);
        }

        let acc_t_end = acc_t.elapsed().as_millis();

        // before was 12
        debug!("time reading acceleration: {}", acc_t_end);

        let shaker_t = Instant::now();
        let level = shake_analyzer.get_current_level();
        let values = shake_analyzer.last_values();
        let shaker_end_t = shaker_t.elapsed().as_micros();

        debug!("shaker t (us): {}", shaker_end_t);

        debug!("level: {}", level);
        debug!("n values: {}", values.len());
        if values.len() > 5 {
            debug!(
                "values: {} {} {} {} {}",
                values[0], values[1], values[2], values[3], values[4]
            );
        }

        let start_t = Instant::now();
        display.show_shaking(level, values);
        let display_t = start_t.elapsed().as_millis();

        debug!("display_time (ms): {}", display_t);

        // Timer::after(Duration::from_millis(100)).await;
    }
}
