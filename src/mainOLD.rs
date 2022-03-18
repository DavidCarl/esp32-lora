extern crate sx127x_lora;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::time::Duration;

use embedded_hal::spi::blocking::Transfer;

//use esp32_hal::spi::{self, SPI};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi;

const FREQUENCY: i64 = 915;
fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    setupsx1276();
}

fn setupsx1276() {
    #[allow(unused)]
    //let peripherals = Peripherals::take().unwrap();
    #[allow(unused)]
    //let pins = peripherals.pins;

    // SX1276 wiring to ESP32
    // Not advised to used pins 6 - 11, 16 - 17
    // DIO0  -> D2
    // RST   -> D14
    // NSS   -> D5
    // SCK   -> D18
    // MOSI  -> D23
    // MISO  -> D19
    
    //let dio0  = pins.gpio2;
    //let rst  = pins.gpio14;
    //let nss   = pins.gpio5;
    //let sck  = pins.gpio18;
    //let mosi = pins.gpio23;
    //let miso = pins.gpio19;

    //let spi = peripherals.spi2;

    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let sclk = peripherals.pins.gpio6;
    let miso = peripherals.pins.gpio2;
    let mosi = peripherals.pins.gpio7;
    let cs = peripherals.pins.gpio10;

    println!("Starting SPI loopback test");
    let config = <spi::config::Config as Default>::default().baudrate(26_i32.MHz().into());
    let mut spi = spi::Master::<spi::SPI2, _, _, _, _>::new(
        spi,
        spi::Pins {
            sclk,
            sdo: miso,
            sdi: Some(mosi),
            cs: Some(cs),
        },
        config,
    ).unwrap();

    //let mut lora = sx127x_lora::LoRa::new(spi, sc, rst, FREQUENCY, delay);
    //let mut lora = sx127x_lora::LoRa::new(
    //    spi, cs, reset,  FREQUENCY, Delay)
    //    .expect("Failed to communicate with radio module!");

    //lora.set_tx_power(17,1); //Using PA_BOOST. See your board for correct pin.
}