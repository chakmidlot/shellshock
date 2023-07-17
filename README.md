Shake detector for a no-gun dry fire training
=============================================

An electronic tool to train pull the trigger with minimum shaking.

The project consists of:
* [Firmware (in progress)](#firmware)
* [3D MODEL (not started yet)](#3d-model);
* [Collector service (not started yet)](#collector-service)


Firmware
--------

The firmware monitors the sensors, calculates the shaking magnitude and shows
it on the screen.

It is located in the `/firmware` directory

### Debug

Connect your Pico to your second Pico like it is described
[in the Internet](https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry).

### Deploy

You can deploy the code to RPi Pico:
1. build the sources with `cargo build --release`
1. connect your Pico to USB with the on-board button pressed
1. deploy the binary with `elf2uf2-rs -d target/thumbv6m-none-eabi/release/shellshock`


### Hardware components

#### [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)

The microcontroller that processes the trigger button, the sensors and show
the information on the display.

#### [OLED I2C 128x64 based on SSD1306](https://www.amazon.com/Hosyond-Display-Self-Luminous-Compatible-Raspberry/dp/B09T6SJBV5)

A screen to show how much the gun shakes.

#### [Accelerometer](https://www.amazon.com/Treedix-Gyroscope-Acceleration-Accelerometer-Converter/dp/B0BK3MBDZ1)

Accelerometer to measure shakiness.

#### A button to to detect trigger pressed

#### A spring to pull the trigger back

#### A spring to support the trigger button


3D MODEL
--------

The gun handle and the components placement is done as an STL built from
the OpenSCAD model.


Collector service
-----------------

REST service to collect the measurements.
