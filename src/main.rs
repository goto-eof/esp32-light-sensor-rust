use bh1750_ehal::{self};
use esp_idf_hal::delay;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::I2cConfig;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::peripherals::Peripherals;
pub fn main() {
    let peripherals = Peripherals::take().unwrap();
    println!("creating I2C bus...");
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    let config = I2cConfig::new().baudrate(400000.into());
    let i2c_instance = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();

    let mut bh1750 =
        bh1750_ehal::BH1750::new(i2c_instance, delay::Ets, bh1750_ehal::Address::ADDR_L).unwrap();
    bh1750.start_measurement(bh1750_ehal::ContinuesMeasurement::HIHGT_RES2);
    loop {
        FreeRtos::delay_ms(100u32);
        let value = bh1750.get_measurement(bh1750_ehal::ContinuesMeasurement::HIHGT_RES2);
        println!("LUX: {:.2} LUX", value);
        FreeRtos::delay_ms(100u32);
    }
}
