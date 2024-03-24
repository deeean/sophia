use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSE_EVENT_FLAGS, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, KEYBD_EVENT_FLAGS, KEYEVENTF_UNICODE, SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use crate::geometry::Point;

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

fn keybd_event(b_vk: u8, b_scan: u8, dw_flags: KEYBD_EVENT_FLAGS, dw_extra_info: usize) {
    unsafe {
        windows::Win32::UI::Input::KeyboardAndMouse::keybd_event(b_vk, b_scan, dw_flags, dw_extra_info);
    }
}

fn mouse_move_inner(x: i32, y: i32) {
    mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, x, y, 0, 0);
}