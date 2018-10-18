# readkey

A very small library for finding out if a key is currently pressed. Currently only supports macOS.

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

