# readkey

A very small library for finding out if a key is currently pressed on macOS.

# Example usage

Check if the `Up` key is currently pressed:

```rust
use readkey::Keycode;

fn main() {
  loop {
    println!("State of Up key: {}", Keycode::Up.is_pressed());
  }
}
```

# Related

[readmouse](https://crates.io/crates/readkey) - Read the mouse location on macOS.
