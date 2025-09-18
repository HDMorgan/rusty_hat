use std::{error::Error, result, thread::sleep, time::Duration};

use rpi_pal::{gpio::{Gpio}, spi::{Bus, Mode, Segment, SlaveSelect, Spi}};

fn create_btg_read_request(requested_address: u8) -> u8 {
    let read_address_mask: u8 = 0xFE;
    let offset_address: u8 = requested_address << 1u8;
    let result = offset_address & read_address_mask;
    result
}

fn read_register(spi: &Spi, register_address: u8) -> rpi_pal::spi::Result<[u8; 4]> {
    let write_buffer = [register_address, 0, 0, 0];
    let _write_segment = Segment::with_write(&write_buffer);

    let mut read_buffer = [0u8; 4];
    let _read_segment = Segment::with_read(&mut read_buffer);

    match spi.transfer(&mut read_buffer, &write_buffer) {
        Ok(it) => println!("Recieved bytes: {:?}", it),
        Err(err) => return Err(err),
    };

    print!("{:?}", write_buffer);
    println!("{:?}", read_buffer);

    Ok(read_buffer)
}

fn hard_reset() -> rpi_pal::gpio::Result<()>
{
    let gpio = Gpio::new()?;
    
    let reset_bcm_number = 12;
    let mut reset_pin = gpio.get(reset_bcm_number)?.into_output();
    reset_pin.set_reset_on_drop(false);

    let sleep_duration = Duration::from_millis(10);

    reset_pin.set_high();
    sleep(sleep_duration);

    reset_pin.set_low();
    sleep(sleep_duration);

    reset_pin.set_high();
    sleep(sleep_duration);

    Ok(())
}

fn main() -> result::Result<(), Box<dyn Error>> {
    let temp = create_btg_read_request(0x02);
    let _addr = temp;
    println!("Read address: {:#x}", temp);
    println!("Read address: {:#x}", _addr);

    hard_reset()?;

    let spi1 =  Spi::new(Bus::Spi0, SlaveSelect::Ss0, 5_000_000, Mode::Mode0)?;

    println!("{}", spi1.bits_per_word()?);

    read_register(&spi1, temp)?;

    Ok(())
}