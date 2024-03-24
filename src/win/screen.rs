use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, RGBQUAD, SRCCOPY};
use windows::Win32::UI::WindowsAndMessaging::{GetDesktopWindow, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use crate::screen::{ImageData};
use crate::geometry::Point;

fn create_bitmap_info(width: i32, height: i32) -> BITMAPINFO {
    unsafe {
        let mut bmi = std::mem::zeroed::<BITMAPINFOHEADER>();

        bmi.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.biWidth = width;
        bmi.biHeight = -height;
        bmi.biPlanes = 1;
        bmi.biBitCount = 32;
        bmi.biCompression = BI_RGB.0;
        bmi.biSizeImage = 0;
        bmi.biXPelsPerMeter = 0;
        bmi.biYPelsPerMeter = 0;
        bmi.biClrUsed = 0;
        bmi.biClrImportant = 0;

        BITMAPINFO {
            bmiHeader: bmi,
            bmiColors: [RGBQUAD::default(); 1],
        }
    }
}


#[napi]
pub async fn get_screen_size() -> Result<Point> {
    match tokio::spawn(async move {
        unsafe {
            let width = GetSystemMetrics(SM_CXSCREEN);
            let height = GetSystemMetrics(SM_CYSCREEN);
            Point::new(width, height)
        }
    }).await {
        Ok(size) => Ok(size),
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}

#[napi]
pub async fn take_screenshot(x: i32, y: i32, width: i32, height: i32) -> Result<ImageData> {
    match tokio::spawn(async move {
        unsafe {
            let hwnd = GetDesktopWindow();
            let h_window_dc = GetDC(hwnd);
            
            let h_dc = CreateCompatibleDC(h_window_dc);
            if h_dc.is_invalid() {
                return Err("CreateCompatibleDC failed".to_string());
            }

            let h_bitmap = CreateCompatibleBitmap(h_window_dc, width, height);
            if h_bitmap.is_invalid() {
                return Err("CreateCompatibleBitmap failed".to_string());
            }

            let res = SelectObject(h_dc, h_bitmap);
            if res.is_invalid()  {
                return Err("SelectObject failed".to_string());
            }

            let mut bitmap_info = create_bitmap_info(width, height);

            let size: usize = (width * height) as usize * 4;
            let mut buf: Vec<u8> = vec![0; size];

            let res = BitBlt(h_dc, 0, 0, width, height, h_window_dc, x, y, SRCCOPY);
            if res.is_err() {
                return Err("BitBlt failed".to_string());
            }

            GetDIBits(h_dc, h_bitmap, 0, height as u32, Some(buf.as_mut_ptr() as *mut _), &mut bitmap_info, DIB_RGB_COLORS, );

            ReleaseDC(hwnd, h_window_dc);
            DeleteDC(h_dc);
            DeleteObject(h_bitmap);

            for i in (0..buf.len()).step_by(4) {
                let b = buf[i];
                let r = buf[i + 2];
                buf[i] = r;
                buf[i + 2] = b;
            }

            Ok(ImageData {
                data: buf,
                width: width as u32,
                height: height as u32,
                pixel_width: 4,
            })
        }
    }).await {
        Ok(res) => {
            match res {
                Ok(data) => Ok(data),
                Err(e) => Err(Error::new(
                    Status::GenericFailure,
                    format!("Error: {:?}", e),
                )),
            }
        },
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}