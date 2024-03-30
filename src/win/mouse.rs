use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSE_EVENT_FLAGS, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use crate::geometry::Point;
use crate::utils::handle_result;

#[napi]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[napi]
pub struct Mouse {

}

#[napi]
impl Mouse {
    #[napi(js_name = "move")]
    pub async fn mouse_move(x: i32, y: i32) -> Result<()> {
        let task = tokio::spawn(async move {
            mouse_move_inner(x, y);

            Ok(())
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn press(button: MouseButton) -> Result<()> {
        let task = tokio::spawn(async move {
            let down = match button {
                MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
                MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
                MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN,
            };

            mouse_event(down, 0, 0, 0, 0);

            Ok(())
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn release(button: MouseButton) -> Result<()> {
        let task = tokio::spawn(async move {
            let up = match button {
                MouseButton::Left => MOUSEEVENTF_LEFTUP,
                MouseButton::Right => MOUSEEVENTF_RIGHTUP,
                MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
            };

            mouse_event(up, 0, 0, 0, 0);

            Ok(())
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn click(button: MouseButton, x: i32, y: i32) -> Result<()> {
        let task = tokio::spawn(async move {
            let (down, up) = match button {
                MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
                MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
                MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
            };

            mouse_move_inner(x, y);
            mouse_event(down, x, y, 0, 0);
            mouse_event(up, x, y, 0, 0);

            Ok(())
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn get_position() -> Result<Point> {
        let task = tokio::spawn(async move {
            Ok(get_mouse_position_inner())
        });

        handle_result(task).await
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