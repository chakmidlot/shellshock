use defmt::debug;
use embassy_time::Instant;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::{Line, Rectangle};
use heapless::Vec;
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize, Ssd1306,
};

type Disp<I, S> = Ssd1306<I, S, BufferedGraphicsMode<S>>;

pub struct Display<I, S>
where
    S: DisplaySize,
{
    display: Disp<I, S>,
}

impl<I, S> Display<I, S>
where
    S: DisplaySize,
    I: WriteOnlyDataCommand,
{
    pub fn new(display: Disp<I, S>) -> Self {
        Self { display }
    }

    pub fn show_shaking(&mut self, last_value: i16, values: Vec<i16, 50>) {
        Rectangle::new(Point::new(28, 23), Size::new(100, 41))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut self.display)
            .unwrap();

        values
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let x = 128 - i as i32 * 2;
                let v_norm = v * 100 / 100 * 40 / 100;

                Line::new(
                    Point::new(x, (63 - &v_norm) as i32),
                    Point::new(x - 2, (63 - &v_norm) as i32),
                )
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .pixels()
            })
            .flatten()
            .draw(&mut self.display)
            .unwrap();

        let v_norm = last_value as i32 * 100 / 100 * 40 / 100;
        Rectangle::new(Point::new(0, 23), Size::new(4, 40))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .pixels()
            .chain(
                Rectangle::new(Point::new(0, 23 + 40 - v_norm), Size::new(4, v_norm as u32))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .pixels(),
            )
            .draw(&mut self.display)
            .unwrap();

        let x = Instant::now();
        self.display.flush().unwrap();
        let t = x.elapsed().as_millis();

        debug!("flush time: {}", t);
    }
}
