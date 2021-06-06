use crate::traits::{Action, Callback, Event, Key, Keyset, ListenError};
use std::{convert::TryInto, ptr::null_mut, sync::Mutex};
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{
  CallNextHookEx, GetMessageA, SetWindowsHookExA, HC_ACTION, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL,
  WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

pub static mut HOOK: HHOOK = null_mut();
lazy_static! {
  static ref GLOBAL_CALLBACK: Mutex<Box<Callback>> = Mutex::new(Box::new(|_: Event| {}));
}

pub fn listen<F>(callback: F) -> Result<(), ListenError>
where
  F: 'static + Fn(Event) + Send,
{
  unsafe {
    {
      let mut cb = GLOBAL_CALLBACK.lock().unwrap();
      *cb = Box::new(callback);
    }
    let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(raw_callback), null_mut(), 0);
    if hook.is_null() {
      let error = GetLastError();
      return Err(ListenError::HookError(error));
    }
    HOOK = hook;
    GetMessageA(null_mut(), null_mut(), 0, 0);
  }
  Ok(())
}

unsafe extern "system" fn raw_callback(code: i32, param: WPARAM, lpdata: LPARAM) -> LRESULT {
  if code == HC_ACTION {
    let event = match param.try_into() {
      Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => Some(Event::new(
        Keyset::new(get_key(lpdata), Vec::new()),
        Action::KeyPress,
      )),
      Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => Some(Event::new(
        Keyset::new(get_key(lpdata), Vec::new()),
        Action::KeyRelease,
      )),
      _ => None,
    };
    if let Some(e) = event {
      let cb = GLOBAL_CALLBACK.lock().unwrap();
      cb(e);
    }
  }
  CallNextHookEx(HOOK, code, param, lpdata)
}

const ALT: u32 = 164;
const ALT_GR: u32 = 165;
const BACKSPACE: u32 = 0x08;
const CAPS_LOCK: u32 = 20;
const CONTROL_LEFT: u32 = 162;
const DOWN_ARROW: u32 = 40;
const ESCAPE: u32 = 27;
const F1: u32 = 112;
const F10: u32 = 121;
const F11: u32 = 122;
const F12: u32 = 123;
const F2: u32 = 113;
const F3: u32 = 114;
const F4: u32 = 115;
const F5: u32 = 116;
const F6: u32 = 117;
const F7: u32 = 118;
const F8: u32 = 119;
const F9: u32 = 120;
const LEFT_ARROW: u32 = 37;
const META_LEFT: u32 = 91;
const RETURN: u32 = 0x0D;
const RIGHT_ARROW: u32 = 39;
const SHIFT_LEFT: u32 = 160;
const SHIFT_RIGHT: u32 = 161;
const SPACE: u32 = 32;
const TAB: u32 = 0x09;
const UP_ARROW: u32 = 38;
const BACK_QUOTE: u32 = 192;
const NUM1: u32 = 49;
const NUM2: u32 = 50;
const NUM3: u32 = 51;
const NUM4: u32 = 52;
const NUM5: u32 = 53;
const NUM6: u32 = 54;
const NUM7: u32 = 55;
const NUM8: u32 = 56;
const NUM9: u32 = 57;
const NUM0: u32 = 48;
const MINUS: u32 = 189;
const EQUAL: u32 = 187;
const KEY_Q: u32 = 81;
const KEY_W: u32 = 87;
const KEY_E: u32 = 69;
const KEY_R: u32 = 82;
const KEY_T: u32 = 84;
const KEY_Y: u32 = 89;
const KEY_U: u32 = 85;
const KEY_I: u32 = 73;
const KEY_O: u32 = 79;
const KEY_P: u32 = 80;
const LEFT_BRACKET: u32 = 219;
const RIGHT_BRACKET: u32 = 221;
const KEY_A: u32 = 65;
const KEY_S: u32 = 83;
const KEY_D: u32 = 68;
const KEY_F: u32 = 70;
const KEY_G: u32 = 71;
const KEY_H: u32 = 72;
const KEY_J: u32 = 74;
const KEY_K: u32 = 75;
const KEY_L: u32 = 76;
const SEMI_COLON: u32 = 186;
const QUOTE: u32 = 222;
const BACK_SLASH: u32 = 220;
const KEY_Z: u32 = 90;
const KEY_X: u32 = 88;
const KEY_C: u32 = 67;
const KEY_V: u32 = 86;
const KEY_B: u32 = 66;
const KEY_N: u32 = 78;
const KEY_M: u32 = 77;
const COMMA: u32 = 188;
const DOT: u32 = 190;
const SLASH: u32 = 191;

unsafe fn get_key(lpdata: LPARAM) -> Key {
  let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
  match kb.vkCode {
    ALT => Key::Alt,
    ALT_GR => Key::AltGr,
    BACKSPACE => Key::Backspace,
    CAPS_LOCK => Key::CapsLock,
    CONTROL_LEFT => Key::ControlLeft,
    DOWN_ARROW => Key::DownArrow,
    ESCAPE => Key::Escape,
    F1 => Key::F1,
    F10 => Key::F10,
    F11 => Key::F11,
    F12 => Key::F12,
    F2 => Key::F2,
    F3 => Key::F3,
    F4 => Key::F4,
    F5 => Key::F5,
    F6 => Key::F6,
    F7 => Key::F7,
    F8 => Key::F8,
    F9 => Key::F9,
    LEFT_ARROW => Key::LeftArrow,
    META_LEFT => Key::MetaLeft,
    RETURN => Key::Return,
    RIGHT_ARROW => Key::RightArrow,
    SHIFT_LEFT => Key::ShiftLeft,
    SHIFT_RIGHT => Key::ShiftRight,
    SPACE => Key::Space,
    TAB => Key::Tab,
    UP_ARROW => Key::UpArrow,
    BACK_QUOTE => Key::BackQuote,
    NUM1 => Key::Num1,
    NUM2 => Key::Num2,
    NUM3 => Key::Num3,
    NUM4 => Key::Num4,
    NUM5 => Key::Num5,
    NUM6 => Key::Num6,
    NUM7 => Key::Num7,
    NUM8 => Key::Num8,
    NUM9 => Key::Num9,
    NUM0 => Key::Num0,
    MINUS => Key::Minus,
    EQUAL => Key::Equal,
    KEY_Q => Key::KeyQ,
    KEY_W => Key::KeyW,
    KEY_E => Key::KeyE,
    KEY_R => Key::KeyR,
    KEY_T => Key::KeyT,
    KEY_Y => Key::KeyY,
    KEY_U => Key::KeyU,
    KEY_I => Key::KeyI,
    KEY_O => Key::KeyO,
    KEY_P => Key::KeyP,
    LEFT_BRACKET => Key::LeftBracket,
    RIGHT_BRACKET => Key::RightBracket,
    KEY_A => Key::KeyA,
    KEY_S => Key::KeyS,
    KEY_D => Key::KeyD,
    KEY_F => Key::KeyF,
    KEY_G => Key::KeyG,
    KEY_H => Key::KeyH,
    KEY_J => Key::KeyJ,
    KEY_K => Key::KeyK,
    KEY_L => Key::KeyL,
    SEMI_COLON => Key::SemiColon,
    QUOTE => Key::Quote,
    BACK_SLASH => Key::BackSlash,
    KEY_Z => Key::KeyZ,
    KEY_X => Key::KeyX,
    KEY_C => Key::KeyC,
    KEY_V => Key::KeyV,
    KEY_B => Key::KeyB,
    KEY_N => Key::KeyN,
    KEY_M => Key::KeyM,
    COMMA => Key::Comma,
    DOT => Key::Dot,
    SLASH => Key::Slash,
    code => Key::Unknown(code.into()),
  }
}
