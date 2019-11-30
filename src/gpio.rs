use rppal::gpio::{Gpio, IoPin, Mode};

#[derive(Debug)]
pub enum Errors {
  FailedInit,
}

pub struct GPIOPin {
  pin_num: u8,
  pin: IoPin,
}

impl GPIOPin {
  pub fn new(pin_num: u8) -> Result<GPIOPin, Errors> {
    let controller = match Gpio::new() {
      Ok(c) => c,
      Err(_) => return Err(Errors::FailedInit),
    };
    
    let mut pin = match controller.get(pin_num) {
      Ok(p) => p,
      Err(_) => return Err(Errors::FailedInit),
    };

    return Ok(GPIOPin {
      pin_num: pin_num,
      pin: pin.into_io(Mode::Input),
    });
  }

  pub fn set_input(&mut self) {
    self.pin.set_mode(Mode::Input);
  }

  pub fn set_output(&mut self) {
    self.pin.set_mode(Mode::Output);
  }

  pub fn is_high(&self) -> bool {
    return self.pin.is_high();
  }

  pub fn is_low(&self) -> bool {
    return self.pin.is_low();
  }

  pub fn set_low(&mut self) {
    self.pin.set_low();
  }

  pub fn set_high(&mut self) {
    self.pin.set_high();
  }
}
