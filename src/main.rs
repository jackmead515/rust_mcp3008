use std::time::Duration;
use std::thread;
mod mcp3008;
mod gpio;

fn main() {
  let chip_select = 25;
  let mut mcp = mcp3008::create(chip_select).unwrap();

  loop {
    let values = mcp.read_all();
    println!("{:?}", values);
    thread::sleep(Duration::from_millis(1000));
  };
}
