

extern crate readkey;

use readkey::Keycode;

fn main() {
  loop {
    println!("State of Up key: {}", Keycode::Up.is_pressed());
  }
}
