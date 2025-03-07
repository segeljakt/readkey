#![allow(unused)]
#![allow(dead_code)]

#[repr(i32)]
enum CGEventSourceStateID {
    Private = -1,
    Combined = 0,
    System = 1,
}

const LSHIFT: u32 = (1 << 1) + (1 << 17);
const LCTRL: u32 = 1 + (1 << 18);
const LOPTION: u32 = (1 << 5) + (1 << 19);
const LCOMMAND: u32 = (1 << 3) + (1 << 20);
const RSHIFT: u32 = (1 << 2) + (1 << 17);
const RCTRL: u32 = (1 << 13) + (1 << 18);
const ROPTION: u32 = (1 << 6) + (1 << 19);
const RCOMMAND: u32 = (1 << 4) + (1 << 20);

/// Carbon's virtual keycodes, found [here](https://snipplr.com/view/42797/).
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Keycode {
  A                      = 0x00,
  S                      = 0x01,
  D                      = 0x02,
  F                      = 0x03,
  H                      = 0x04,
  G                      = 0x05,
  Z                      = 0x06,
  X                      = 0x07,
  C                      = 0x08,
  V                      = 0x09,
  B                      = 0x0B,
  Q                      = 0x0C,
  W                      = 0x0D,
  E                      = 0x0E,
  R                      = 0x0F,
  Y                      = 0x10,
  T                      = 0x11,
  _1                     = 0x12,
  _2                     = 0x13,
  _3                     = 0x14,
  _4                     = 0x15,
  _6                     = 0x16,
  _5                     = 0x17,
  Equal                  = 0x18,
  _9                     = 0x19,
  _7                     = 0x1A,
  Minus                  = 0x1B,
  _8                     = 0x1C,
  _0                     = 0x1D,
  RightBracket           = 0x1E,
  O                      = 0x1F,
  U                      = 0x20,
  LeftBracket            = 0x21,
  I                      = 0x22,
  P                      = 0x23,
  L                      = 0x25,
  J                      = 0x26,
  Quote                  = 0x27,
  K                      = 0x28,
  Semicolon              = 0x29,
  Backslash              = 0x2A,
  Comma                  = 0x2B,
  Slash                  = 0x2C,
  N                      = 0x2D,
  M                      = 0x2E,
  Period                 = 0x2F,
  Grave                  = 0x32,
  KeypadDecimal          = 0x41,
  KeypadMultiply         = 0x43,
  KeypadPlus             = 0x45,
  KeypadClear            = 0x47,
  KeypadDivide           = 0x4B,
  KeypadEnter            = 0x4C,
  KeypadMinus            = 0x4E,
  KeypadEquals           = 0x51,
  Keypad0                = 0x52,
  Keypad1                = 0x53,
  Keypad2                = 0x54,
  Keypad3                = 0x55,
  Keypad4                = 0x56,
  Keypad5                = 0x57,
  Keypad6                = 0x58,
  Keypad7                = 0x59,
  Keypad8                = 0x5B,
  Keypad9                = 0x5C,
  /// Keycodes for keys that are independent of keyboard layout
  Return                 = 0x24,
  Tab                    = 0x30,
  Space                  = 0x31,
  Delete                 = 0x33,
  Escape                 = 0x35,
  Command                = 0x37,
  Shift                  = 0x38,
  CapsLock               = 0x39,
  Option                 = 0x3A,
  Control                = 0x3B,
  RightShift             = 0x3C,
  RightOption            = 0x3D,
  RightControl           = 0x3E,
  Function               = 0x3F,
  F17                    = 0x40,
  VolumeUp               = 0x48,
  VolumeDown             = 0x49,
  Mute                   = 0x4A,
  F18                    = 0x4F,
  F19                    = 0x50,
  F20                    = 0x5A,
  F5                     = 0x60,
  F6                     = 0x61,
  F7                     = 0x62,
  F3                     = 0x63,
  F8                     = 0x64,
  F9                     = 0x65,
  F11                    = 0x67,
  F13                    = 0x69,
  F16                    = 0x6A,
  F14                    = 0x6B,
  F10                    = 0x6D,
  F12                    = 0x6F,
  F15                    = 0x71,
  Help                   = 0x72,
  Home                   = 0x73,
  PageUp                 = 0x74,
  ForwardDelete          = 0x75,
  F4                     = 0x76,
  End                    = 0x77,
  F2                     = 0x78,
  PageDown               = 0x79,
  F1                     = 0x7A,
  Left                   = 0x7B,
  Right                  = 0x7C,
  Down                   = 0x7D,
  Up                     = 0x7E,
  /// ISO keyboards only
  Section                = 0x0A,
  /// JIS keyboards only
  Yen                    = 0x5D,
  Underscore             = 0x5E,
  KeypadComma            = 0x5F,
  Eisu                   = 0x66,
  Kana                   = 0x68,
  /// Something missing
  RightCommand           = 0x90,
}

#[link(name = "AppKit", kind = "framework")]
extern {
  fn CGEventSourceKeyState(state: CGEventSourceStateID, keycode: Keycode) -> bool;
  fn CGEventSourceFlagsState(state: CGEventSourceStateID) -> u32;
}

impl Keycode {
  /// Returns true if key is currently pressed.
  /// ```
  /// use readkey::Keycode;
  /// loop {
  ///   println!("State of Up key: {:?}, ", Keycode::Up.is_pressed());
  /// }
  /// ```
  #[inline(always)]
  pub fn is_pressed(self) -> bool {
    unsafe {
      let m = CGEventSourceFlagsState(CGEventSourceStateID::Combined);
      match self {
        Keycode::Command => m & LCOMMAND == LCOMMAND,
        Keycode::Shift => m & LSHIFT == LSHIFT,
        Keycode::Option => m & LOPTION == LOPTION,
        Keycode::Control => m & LCTRL == LCTRL,
        Keycode::RightCommand => m & RCOMMAND == RCOMMAND,
        Keycode::RightShift => m & RSHIFT == RSHIFT,
        Keycode::RightOption => m & ROPTION == ROPTION,
        Keycode::RightControl => m & RCTRL == RCTRL,
        _ => CGEventSourceKeyState(CGEventSourceStateID::Combined, self),
      }
    }
  }
}
