extern crate sx127x_lora;

use embedded_hal::digital::blocking::InputPin;
use embedded_hal::digital::blocking::IoPin;
use embedded_hal::digital::blocking::OutputPin;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::thread;
use std::time::Duration;

use embedded_hal::spi::blocking::Transfer;
use esp_idf_hal::delay::Ets;
//use embedded_hal::digital::v1::OutputPin;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
const FREQUENCY: i64 = 915;
fn main() {

    setupsx1276();
}

fn setupsx1276() {
    #[allow(unused)]
    #[allow(unused)]


    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let sclk = peripherals.pins.gpio6;
    let miso = peripherals.pins.gpio19;
    let mosi = peripherals.pins.gpio23;
    let cs = peripherals.pins.gpio10;
    let rst  = peripherals.pins.gpio14;

    println!("Starting SPI loopback test");
    let config = <spi::config::Config as Default>::default().baudrate(26.MHz().into());
    /*let mut spi = spi::Master::<spi::SPI2, _, _, _, _>::new(
    /    spi,
        spi::Pins {
            sclk,
            sdo: miso,
            sdi: Some(mosi),
            cs: Some(cs),
        },
        config,
    ).unwrap();*/
    let mut lora = sx127x_lora::LoRa::new(spi.into_output().unwrap(), cs.into_output().unwrap(), rst.into_input_output().unwrap(), FREQUENCY,Ets);
    
    //let mut lora = sx127x_lora::LoRa::new(
    //    spi, cs, reset,  FREQUENCY, Delay)
    //    .expect("Failed to communicate with radio module!");

    //lora.set_tx_power(17,1); //Using PA_BOOST. See your board for correct pin.
}