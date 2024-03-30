use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, SET_WINDOW_POS_FLAGS, SetForegroundWindow, SetWindowPos, SHOW_WINDOW_CMD, ShowWindow, ShowWindowAsync, SW_MAXIMIZE, SW_MINIMIZE, SW_SHOWNORMAL, SWP_NOMOVE, SWP_NOSIZE};
use crate::geometry::Rect;
use crate::utils::{decode_wide, encode_wide, handle_result};

#[napi]
pub struct Window {
    hwnd: HWND
}

#[napi]
impl Window {
    #[napi]
    pub async fn minimize(&self) -> Result<()> {
        self.show_window(SW_MINIMIZE).await
    }

    #[napi]
    pub async fn maximize(&self) -> Result<()> {
        self.show_window(SW_MAXIMIZE).await
    }

    #[napi]
    pub async fn get_title(&self) -> Result<String> {
        let hwnd = self.hwnd;

        let task = tokio::spawn(async move {

            unsafe {
                let len = GetWindowTextLengthW(hwnd);
                let mut buffer = vec![0u16; len as usize + 1];
                GetWindowTextW(hwnd, &mut buffer);
                Ok(decode_wide(&buffer))
            }
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn get_window_rect(&self) -> Result<Rect> {
        let hwnd = self.hwnd;

        let task = tokio::spawn(async move {
            let mut rect = windows::Win32::Foundation::RECT::default();

            unsafe {
                let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
            }

            Ok(Rect {
                left: rect.left,
                top: rect.top,
                right: rect.right,
                bottom: rect.bottom,
            })
        });

        handle_result(task).await
    }

    async fn show_window(&self, state: SHOW_WINDOW_CMD) -> Result<()> {
        let hwnd = self.hwnd;

        let task = tokio::spawn(async move {
            unsafe {
                ShowWindow(hwnd, state);
            }

            Ok(())
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn set_position(&self, x: i32, y: i32) -> Result<()> {
        self.set_window_pos(x, y, 0, 0, SWP_NOSIZE).await
    }

    #[napi]
    pub async fn set_size(&self, width: i32, height: i32) -> Result<()> {
        self.set_window_pos(0, 0, width, height, SWP_NOMOVE).await
    }

    #[napi]
    pub async fn foreground(&self) -> Result<bool> {
        let hwnd = self.hwnd;

        let task = tokio::spawn(async move {
            unsafe {
                let _ = ShowWindowAsync(hwnd, SW_SHOWNORMAL);
            };

            let res = unsafe {
                SetForegroundWindow(hwnd)
            };

            Ok(res.0 != 0)
        });

        handle_result(task).await
    }

    async fn set_window_pos(&self, x: i32, y: i32, width: i32, height: i32, flags: SET_WINDOW_POS_FLAGS) -> Result<()> {
        let hwnd = self.hwnd;

        let task = tokio::spawn(async move {
            unsafe {
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    x,
                    y,
                    width,
                    height,
                    flags,
                );
            }

            Ok(())
        });

        handle_result(task).await
    }


    #[napi]
    pub async fn get_foreground_window() -> Result<Option<Window>> {
        let task = tokio::spawn(async move {
            let hwnd = unsafe { GetForegroundWindow() };

            if hwnd.0 == 0 {
                Ok(None)
            } else {
                Ok(Some(Window { hwnd }))
            }
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn find_window_by_title(title: String) -> Result<Option<Window>> {
        let task = tokio::spawn(async move {
            let hwnd = unsafe {
                FindWindowW(None, PCWSTR(encode_wide(title).as_ptr()))
            };

            if hwnd.0 == 0 {
                Ok(None)
            } else {
                Ok(Some(Window { hwnd }))
            }
        });

        handle_result(task).await
    }

    #[napi]
    pub async fn find_window_by_class_name(classname: String) -> Result<Option<Window>> {
        let task = tokio::spawn(async move {
            let hwnd = unsafe {
                FindWindowW(PCWSTR(encode_wide(classname).as_ptr()), None)
            };

            if hwnd.0 == 0 {
                Ok(None)
            } else {
                Ok(Some(Window { hwnd }))
            }
        });

        handle_result(task).await
    }
}