use std::time::Duration;
use std::thread;
mod mcp3008;
mod gpio;

fn main() {
  let mut mcp = mcp3008::create().unwrap();

  loop {
    let values = mcp.read_all();

    println!("{:?}", values);

    thread::sleep(Duration::from_millis(500));
  };
}
