use core::{i16, u16};
use defmt::debug;
use embassy_rp::i2c::Error;
use embedded_hal_async::i2c::I2c;
use heapless::Vec;
use {defmt_rtt as _, panic_probe as _};

const DEVICE_ADDRESS: u8 = 0x68;

pub struct Accelerometer<I2C> {
    i2c: I2C,
}

impl<I2C: I2c> Accelerometer<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub async fn init(&mut self) -> Result<(), Error>
    where
        Error: From<<I2C as embedded_hal_async::i2c::ErrorType>::Error>,
    {
        self.i2c.write(DEVICE_ADDRESS, &[0x75]).await?; // who am I
        self.i2c.write(DEVICE_ADDRESS, &[0x6B, 0x00]).await?; // power on

        self.i2c.write(DEVICE_ADDRESS, &[0x6A, 0x40]).await?; // enable fifo
        self.i2c.write(DEVICE_ADDRESS, &[0x19, 0xA0]).await?; // sample rate 1
        self.i2c.write(DEVICE_ADDRESS, &[0x1A, 0x00]).await?; // sample rate 10 Hz

        self.i2c.write(DEVICE_ADDRESS, &[0x23, 0x08]).await?; // fifo accelerometer

        self.i2c.write(DEVICE_ADDRESS, &[0x1B, 0x00]).await?; // GYRO_CONFIG
        self.i2c.write(DEVICE_ADDRESS, &[0x1C, 0x00]).await?; // ACCEL_CONFIG

        Ok(())
    }

    pub async fn read_fifo(&mut self) -> Result<Vec<(i16, i16, i16), 100>, Error>
    where
        Error: From<<I2C as embedded_hal_async::i2c::ErrorType>::Error>,
    {
        let mut result = Vec::<(i16, i16, i16), 100>::new();

        let mut fifo_size = [0; 2];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[0x72], &mut fifo_size)
            .await?;

        let mut fifo_size: u16 = u16::from_be_bytes(fifo_size);
        debug!("fifo_size: {}", fifo_size);

        let mut buffer = [0; 6];
        while fifo_size >= 6 && !result.is_full() {
            self.i2c
                .write_read(DEVICE_ADDRESS, &[0x74], &mut buffer)
                .await?;

            let x = i16::from_be_bytes(buffer[0..2].try_into().unwrap());
            let y = i16::from_be_bytes(buffer[2..4].try_into().unwrap());
            let z = i16::from_be_bytes(buffer[4..6].try_into().unwrap());

            result.push((x, y, z)).unwrap();

            fifo_size -= 6;
        }

        return Ok(result);
    }
}
