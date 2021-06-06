use std::fmt::Display;

pub type Callback = dyn 'static + Fn(Event) + Send;

/// In the event a listner fails to operate, you may expect a ListenerError
#[derive(Debug)]
#[non_exhaustive]
pub enum ListenError {
  /// MacOS specific, occurs when the system fails to tie into the global
  /// event loop ("Tap", as it happens to be called in Apple land)
  EventTapError,

  /// MacOS Specific, occurs when the system fails to register the loop
  /// primitive against the host system.
  LoopSourceError,

  /// Windows Specific, occurs when the hook fails to initialize
  HookError(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
  pub keyset: Keyset,
  pub action: Action,
}

impl Event {
  pub fn new(keyset: Keyset, action: Action) -> Event {
    Event { keyset, action }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
  KeyPress,
  KeyRelease,
}

/// A representation of a [`Key`] and 0+ modifiers for matching against
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Keyset {
  /// The invidiual [`Key`] of interest
  pub key: Key,
  /// A series of modifier keys (alt, option, super, etc) that we should look for
  /// when matching against [`Keyset::key`]. Including none means we won't look for
  /// any during matching!
  pub mods: Vec<Key>,
}

impl Keyset {
  /// Creates a new [`Keyset`]
  pub fn new(key: Key, mods: Vec<Key>) -> Keyset {
    Keyset { key, mods }
  }
}

impl Default for Keyset {
  /// Convenience for scenarios where one might need to initialize a bare variable
  /// As a result we just pick [`Key::Alt`] for the sake of picking something.
  fn default() -> Self {
    Keyset {
      key: Key::Alt,
      mods: Vec::new(),
    }
  }
}

impl Display for Keyset {
  /// Will render a [`Keyset`] in syntax of `{Modifiers-Key}`, just for legibility
  /// in logs and such.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mods = self
      .mods
      .iter()
      .map(Key::to_string)
      .collect::<Vec<String>>()
      .join("-");
    if mods.is_empty() {
      write!(f, "{{{}}}", self.key)
    } else {
      write!(f, "{{{}-{}}}", mods, self.key)
    }
  }
}

impl Display for Key {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
  /// Alt key on Linux and Windows (option key on macOS)
  Alt,
  AltGr,
  Backspace,
  CapsLock,
  ControlLeft,
  ControlRight,
  Delete,
  DownArrow,
  End,
  Escape,
  F1,
  F10,
  F11,
  F12,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  Home,
  LeftArrow,
  /// also known as "windows", "super", and "command"
  MetaLeft,
  /// also known as "windows", "super", and "command"
  MetaRight,
  PageDown,
  PageUp,
  Return,
  RightArrow,
  ShiftLeft,
  ShiftRight,
  Space,
  Tab,
  UpArrow,
  PrintScreen,
  ScrollLock,
  Pause,
  NumLock,
  BackQuote,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  Num0,
  Minus,
  Equal,
  KeyQ,
  KeyW,
  KeyE,
  KeyR,
  KeyT,
  KeyY,
  KeyU,
  KeyI,
  KeyO,
  KeyP,
  LeftBracket,
  RightBracket,
  KeyA,
  KeyS,
  KeyD,
  KeyF,
  KeyG,
  KeyH,
  KeyJ,
  KeyK,
  KeyL,
  SemiColon,
  Quote,
  BackSlash,
  IntlBackslash,
  KeyZ,
  KeyX,
  KeyC,
  KeyV,
  KeyB,
  KeyN,
  KeyM,
  Comma,
  Dot,
  Slash,
  Insert,
  KpReturn,
  KpMinus,
  KpPlus,
  KpMultiply,
  KpDivide,
  Kp0,
  Kp1,
  Kp2,
  Kp3,
  Kp4,
  Kp5,
  Kp6,
  Kp7,
  Kp8,
  Kp9,
  KpDelete,
  Function,
  Unknown(u32),
}
