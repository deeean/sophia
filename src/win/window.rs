use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, SET_WINDOW_POS_FLAGS, SetForegroundWindow, SetWindowPos, SHOW_WINDOW_CMD, ShowWindow, ShowWindowAsync, SW_MAXIMIZE, SW_MINIMIZE, SW_SHOWNORMAL, SWP_NOMOVE, SWP_NOSIZE};
use crate::geometry::Rect;
use crate::utils::{decode_wide, encode_wide};

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

        match tokio::spawn(async move {

            unsafe {
                let len = GetWindowTextLengthW(hwnd);
                let mut buffer = vec![0u16; len as usize + 1];
                GetWindowTextW(hwnd, &mut buffer);
                decode_wide(&buffer)
            }
        }).await {
            Ok(text) => Ok(text),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }

    #[napi]
    pub async fn get_window_rect(&self) -> Result<Rect> {
        let hwnd = self.hwnd;

        match tokio::spawn(async move {
            let mut rect = windows::Win32::Foundation::RECT::default();

            unsafe {
                let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
            }

            Rect {
                left: rect.left,
                top: rect.top,
                right: rect.right,
                bottom: rect.bottom,
            }
        }).await {
            Ok(rect) => Ok(rect),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }

    async fn show_window(&self, state: SHOW_WINDOW_CMD) -> Result<()> {
        let hwnd = self.hwnd;

        match tokio::spawn(async move {
            unsafe {
                ShowWindow(hwnd, state);
            }
        }).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
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

        match tokio::spawn(async move {
            unsafe {
                let _ = ShowWindowAsync(hwnd, SW_SHOWNORMAL);
            };

            let res = unsafe {
                SetForegroundWindow(hwnd)
            };

            res.0 != 0
        }).await {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }

    async fn set_window_pos(&self, x: i32, y: i32, width: i32, height: i32, flags: SET_WINDOW_POS_FLAGS) -> Result<()> {
        let hwnd = self.hwnd;

        match tokio::spawn(async move {
            unsafe {
                SetWindowPos(
                    hwnd,
                    None,
                    x,
                    y,
                    width,
                    height,
                    flags,
                )
            }
        }).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }


    #[napi]
    pub async fn get_foreground_window() -> Result<Option<Window>> {
        match tokio::spawn(async move {
            let hwnd = unsafe { GetForegroundWindow() };

            if hwnd.0 == 0 {
                None
            } else {
                Some(Window { hwnd })
            }
        }).await {
            Ok(window) => Ok(window),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }

    #[napi]
    pub async fn find_window_by_title(title: String) -> Result<Option<Window>> {
        match tokio::spawn(async move {
            let hwnd = unsafe {
                FindWindowW(None, PCWSTR(encode_wide(title).as_ptr()))
            };

            if hwnd.0 == 0 {
                None
            } else {
                Some(Window { hwnd })
            }
        }).await {
            Ok(window) => Ok(window),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }

    #[napi]
    pub async fn find_window_by_class_name(classname: String) -> Result<Option<Window>> {
        match tokio::spawn(async move {
            let hwnd = unsafe {
                FindWindowW(PCWSTR(encode_wide(classname).as_ptr()), None)
            };

            if hwnd.0 == 0 {
                None
            } else {
                Some(Window { hwnd })
            }
        }).await {
            Ok(window) => Ok(window),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Error: {:?}", e),
            )),
        }
    }
}