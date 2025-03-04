extern crate readkey;

use readkey::Keycode;

fn main() {
    loop {
        if Keycode::Up.is_pressed() {
            println!("Up is pressed");
        }
        if Keycode::Right.is_pressed() {
            println!("Right is pressed");
        }
        if Keycode::Left.is_pressed() {
            println!("Left is pressed");
        }
        if Keycode::Down.is_pressed() {
            println!("Down is pressed");
        }
    }
}
