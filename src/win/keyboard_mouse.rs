use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSE_EVENT_FLAGS, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, KEYBD_EVENT_FLAGS, KEYEVENTF_UNICODE, SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN, };
use crate::geometry::Point;

#[napi]
#[derive(Debug)]
pub enum Key {
    None = 0,
    Back = 8,
    Tab = 9,
    LineFeed = 10,
    Clear = 12,
    Enter = 13,
    Shift = 16,
    Control = 17,
    Alt = 18,
    Pause = 19,
    CapsLock = 20,
    Esc = 27,
    Space = 32,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    ArrowLeft = 37,
    ArrowUp = 38,
    ArrowRight = 39,
    ArrowDown = 40,
    Insert = 45,
    Delete = 46,
    D0 = 48,
    D1 = 49,
    D2 = 50,
    D3 = 51,
    D4 = 52,
    D5 = 53,
    D6 = 54,
    D7 = 55,
    D8 = 56,
    D9 = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftWin = 91,
    RightWin = 92,
    Apps = 93,
    Sleep = 95,
    NumPad0 = 96,
    NumPad1 = 97,
    NumPad2 = 98,
    NumPad3 = 99,
    NumPad4 = 100,
    NumPad5 = 101,
    NumPad6 = 102,
    NumPad7 = 103,
    NumPad8 = 104,
    NumPad9 = 105,
    Multiply = 106,
    Add = 107,
    Separator = 108,
    Subtract = 109,
    Decimal = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F13 = 124,
    F14 = 125,
    F15 = 126,
    F16 = 127,
    F17 = 128,
    F18 = 129,
    F19 = 130,
    F20 = 131,
    F21 = 132,
    F22 = 133,
    F23 = 134,
    F24 = 135,
    NumLock = 144,
    ScrollLock = 145,
    LeftShift = 160,
    RightShift = 161,
    LeftControl = 162,
    RightControl = 163,
    LeftAlt = 164,
    RightAlt = 165,
}

bitflags::bitflags! {
    pub struct Modifiers: u32 {
        const Alt = 0x01;
        const AltGraph = 0x2;
        const CapsLock = 0x4;
        const Control = 0x8;
        const Fn = 0x10;
        const FnLock = 0x20;
        const Meta = 0x40;
        const NumLock = 0x80;
        const ScrollLock = 0x100;
        const Shift = 0x200;
        const Symbol = 0x400;
        const SymbolLock = 0x800;
        const HYPER = 0x1000;
        const SUPER = 0x2000;
    }
}

#[napi]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[napi]
pub async fn mouse_move(x: i32, y: i32) -> Result<bool> {
    match tokio::spawn(async move {
        mouse_move_inner(x, y);
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn mouse_press(button: MouseButton) -> Result<bool> {
    match tokio::spawn(async move {
        let down = match button {
            MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
            MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN,
        };

        mouse_event(down, 0, 0, 0, 0);
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn mouse_release(button: MouseButton) -> Result<bool> {
    match tokio::spawn(async move {
        let up = match button {
            MouseButton::Left => MOUSEEVENTF_LEFTUP,
            MouseButton::Right => MOUSEEVENTF_RIGHTUP,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
        };

        mouse_event(up, 0, 0, 0, 0);
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn mouse_click(button: MouseButton, x: i32, y: i32) -> Result<bool> {
    match tokio::spawn(async move {
        let (down, up) = match button {
            MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
            MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
            MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
        };

        mouse_move_inner(x, y);
        mouse_event(down, x, y, 0, 0);
        mouse_event(up, x, y, 0, 0);
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn key_click(key: Key) -> Result<bool> {
    match tokio::spawn(async move {
        unsafe {
            let mut inputs = Vec::new();

            let mut input = INPUT::default();
            input.r#type = INPUT_KEYBOARD;
            input.Anonymous.ki.wVk = VIRTUAL_KEY(key as u16);
            input.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS::from(KEYEVENTF_UNICODE);
            input.Anonymous.ki.time = 0;
            inputs.push(input);

            input.Anonymous.ki.dwFlags |= KEYEVENTF_KEYUP;
            inputs.push(input);

            SendInput(inputs.as_slice(), std::mem::size_of::<INPUT>() as i32);
        }
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn key_press(key: Key) -> Result<bool> {
    match tokio::spawn(async move {
        unsafe {
            let mut input = INPUT::default();
            input.r#type = INPUT_KEYBOARD;
            input.Anonymous.ki.wVk = VIRTUAL_KEY(key as u16);
            input.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS::from(KEYEVENTF_UNICODE);
            input.Anonymous.ki.time = 0;
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn key_release(key: Key) -> Result<bool> {
    match tokio::spawn(async move {
        unsafe {
            let mut input = INPUT::default();
            input.r#type = INPUT_KEYBOARD;
            input.Anonymous.ki.wVk = VIRTUAL_KEY(key as u16);
            input.Anonymous.ki.dwFlags = KEYBD_EVENT_FLAGS::from(KEYEVENTF_UNICODE | KEYEVENTF_KEYUP);
            input.Anonymous.ki.time = 0;
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
    }).await {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn get_mouse_position() -> Result<Point> {
    match tokio::spawn(async move {
        get_mouse_position_inner()
    }).await {
        Ok(pos) => Ok(pos),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn type_text(text: String) -> Result<()> {
    match tokio::spawn(async move {
        unsafe {
            let text = text
                .encode_utf16()
                .collect::<Vec<_>>();

            let mut inputs = Vec::new();

            for c in text {
                let mut input = INPUT::default();
                input.r#type = INPUT_KEYBOARD;
                input.Anonymous.ki.dwFlags = KEYEVENTF_UNICODE;
                input.Anonymous.ki.wScan = c;
                input.Anonymous.ki.time = 0;
                inputs.push(input);

                input.Anonymous.ki.dwFlags |= KEYEVENTF_KEYUP;
                inputs.push(input);
            }

            SendInput(inputs.as_slice(), std::mem::size_of::<INPUT>() as i32);
        }
    }).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

fn get_mouse_position_inner() -> Point {
    let mut position = windows::Win32::Foundation::POINT { x: 0, y: 0 };
    unsafe {
        let _ = GetCursorPos(&mut position);
    }

    Point::new(position.x, position.y)
}

fn mouse_event(dw_flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, dw_data: i32, dw_extra_info: usize) {
    unsafe {
        let x = dx * 65536 / GetSystemMetrics(SM_CXSCREEN);
        let y = dy * 65536 / GetSystemMetrics(SM_CYSCREEN);
        windows::Win32::UI::Input::KeyboardAndMouse::mouse_event(dw_flags, x, y, dw_data, dw_extra_info);
    }
}

fn mouse_move_inner(x: i32, y: i32) {
    mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x, y, 0, 0);
}