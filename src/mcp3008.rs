use std::io::{Read, Write};
use spidev::{Spidev, SpidevOptions, SpiModeFlags, SpidevTransfer};

use crate::gpio::GPIOPin;

#[derive(Debug)]
pub enum Errors {
  FailedInit,
  FailedRead
}

pub struct MCP3008 {
  spi: Spidev,
  cs: GPIOPin
}

pub fn create(chip_select: u8) -> Result<MCP3008, Errors> {
  let options = SpidevOptions::new()
    .bits_per_word(8)
    .max_speed_hz(1_000_000)
    .mode(SpiModeFlags::SPI_MODE_0)
    .build();

  let mut spi = match Spidev::open("/dev/spidev0.0") {
    Ok(spi) => spi,
    Err(_) => return Err(Errors::FailedInit)
  };

  match spi.configure(&options) { 
    Ok(_) => (),
    Err(_) => return Err(Errors::FailedInit)
  };

  let mut cs = match GPIOPin::new(chip_select) {
    Ok(cs) => cs,
    Err(_) => return Err(Errors::FailedInit)
  };

  cs.set_output();

  return Ok(MCP3008 {
    spi: spi,
    cs: cs
  });
}

impl MCP3008 {
  pub fn read(&mut self, channel: u8) -> Result<u16, Errors> {
    let write = [1, 8+channel << 4, 0];
    let mut read: [u8; 3] = [0; 3];
    let mut transfer = SpidevTransfer::read_write(&write, &mut read);
    
    self.cs.set_low();
    match self.spi.transfer(&mut transfer) {
      Ok(_) => (),
      Err(_) => return Err(Errors::FailedRead)
    };
    self.cs.set_high();

    return Ok((((read[1] as u16) << 8) | (read[2] as u16)) & 1023);
  }

  pub fn read_all(&mut self) -> Vec<usize> {
    let mut values = Vec::new();
    for i in 0..8 {
      match self.read(i as u8) {
        Ok(value) => values.push(value as usize),
        Err(_) => values.push(1024)
      };
    }
    return values;
  }
}