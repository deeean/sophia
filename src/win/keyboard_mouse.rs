use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Mutex;
use crossbeam_channel::{Receiver, Sender, unbounded};
use lazy_static::lazy_static;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOUSE_EVENT_FLAGS, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, KEYBD_EVENT_FLAGS, KEYEVENTF_UNICODE, SendInput, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, VIRTUAL_KEY, RegisterHotKey, MOD_NOREPEAT, MOD_SHIFT, MOD_ALT, MOD_CONTROL, HOT_KEY_MODIFIERS, UnregisterHotKey};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, CW_USEDEFAULT, DefWindowProcW, DispatchMessageW, GetCursorPos, GetMessageW, GetSystemMetrics, HMENU, PeekMessageW, PM_REMOVE, RegisterClassW, SM_CXSCREEN, SM_CYSCREEN, TranslateMessage, WM_HOTKEY, WM_QUIT, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE};
use crate::geometry::Point;
use crate::utils::encode_wide;

#[napi]
#[derive(Debug, PartialEq)]
pub enum Modifiers {
    Alt = 0x01,
    AltGraph = 0x2,
    CapsLock = 0x4,
    Control = 0x8,
    Fn = 0x10,
    FnLock = 0x20,
    Meta = 0x40,
    NumLock = 0x80,
    ScrollLock = 0x100,
    Shift = 0x200,
    Symbol = 0x400,
    SymbolLock = 0x800,
    Hyper = 0x1000,
    Super = 0x2000,
}


#[napi]
#[derive(Debug, PartialEq)]
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

// Key to string impl
impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Key::None => "None".to_string(),
            Key::Back => "Back".to_string(),
            Key::Tab => "Tab".to_string(),
            Key::LineFeed => "LineFeed".to_string(),
            Key::Clear => "Clear".to_string(),
            Key::Enter => "Enter".to_string(),
            Key::Shift => "Shift".to_string(),
            Key::Control => "Control".to_string(),
            Key::Alt => "Alt".to_string(),
            Key::Pause => "Pause".to_string(),
            Key::CapsLock => "CapsLock".to_string(),
            Key::Esc => "Esc".to_string(),
            Key::Space => "Space".to_string(),
            Key::PageUp => "PageUp".to_string(),
            Key::PageDown => "PageDown".to_string(),
            Key::End => "End".to_string(),
            Key::Home => "Home".to_string(),
            Key::ArrowLeft => "ArrowLeft".to_string(),
            Key::ArrowUp => "ArrowUp".to_string(),
            Key::ArrowRight => "ArrowRight".to_string(),
            Key::ArrowDown => "ArrowDown".to_string(),
            Key::Insert => "Insert".to_string(),
            Key::Delete => "Delete".to_string(),
            Key::D0 => "D0".to_string(),
            Key::D1 => "D1".to_string(),
            Key::D2 => "D2".to_string(),
            Key::D3 => "D3".to_string(),
            Key::D4 => "D4".to_string(),
            Key::D5 => "D5".to_string(),
            Key::D6 => "D6".to_string(),
            Key::D7 => "D7".to_string(),
            Key::D8 => "D8".to_string(),
            Key::D9 => "D9".to_string(),
            Key::A => "A".to_string(),
            Key::B => "B".to_string(),
            Key::C => "C".to_string(),
            Key::D => "D".to_string(),
            Key::E => "E".to_string(),
            Key::F => "F".to_string(),
            Key::G => "G".to_string(),
            Key::H => "H".to_string(),
            Key::I => "I".to_string(),
            Key::J => "J".to_string(),
            Key::K => "K".to_string(),
            Key::L => "L".to_string(),
            Key::M => "M".to_string(),
            Key::N => "N".to_string(),
            Key::O => "O".to_string(),
            Key::P => "P".to_string(),
            Key::Q => "Q".to_string(),
            Key::R => "R".to_string(),
            Key::S => "S".to_string(),
            Key::T => "T".to_string(),
            Key::U => "U".to_string(),
            Key::V => "V".to_string(),
            Key::W => "W".to_string(),
            Key::X => "X".to_string(),
            Key::Y => "Y".to_string(),
            Key::Z => "Z".to_string(),
            Key::LeftWin => "LeftWin".to_string(),
            Key::RightWin => "RightWin".to_string(),
            Key::Apps => "Apps".to_string(),
            Key::Sleep => "Sleep".to_string(),
            Key::NumPad0 => "NumPad0".to_string(),
            Key::NumPad1 => "NumPad1".to_string(),
            Key::NumPad2 => "NumPad2".to_string(),
            Key::NumPad3 => "NumPad3".to_string(),
            Key::NumPad4 => "NumPad4".to_string(),
            Key::NumPad5 => "NumPad5".to_string(),
            Key::NumPad6 => "NumPad6".to_string(),
            Key::NumPad7 => "NumPad7".to_string(),
            Key::NumPad8 => "NumPad8".to_string(),
            Key::NumPad9 => "NumPad9".to_string(),
            Key::Multiply => "Multiply".to_string(),
            Key::Add => "Add".to_string(),
            Key::Separator => "Separator".to_string(),
            Key::Subtract => "Subtract".to_string(),
            Key::Decimal => "Decimal".to_string(),
            Key::Divide => "Divide".to_string(),
            Key::F1 => "F1".to_string(),
            Key::F2 => "F2".to_string(),
            Key::F3 => "F3".to_string(),
            Key::F4 => "F4".to_string(),
            Key::F5 => "F5".to_string(),
            Key::F6 => "F6".to_string(),
            Key::F7 => "F7".to_string(),
            Key::F8 => "F8".to_string(),
            Key::F9 => "F9".to_string(),
            Key::F10 => "F10".to_string(),
            Key::F11 => "F11".to_string(),
            Key::F12 => "F12".to_string(),
            Key::F13 => "F13".to_string(),
            Key::F14 => "F14".to_string(),
            Key::F15 => "F15".to_string(),
            Key::F16 => "F16".to_string(),
            Key::F17 => "F17".to_string(),
            Key::F18 => "F18".to_string(),
            Key::F19 => "F19".to_string(),
            Key::F20 => "F20".to_string(),
            Key::F21 => "F21".to_string(),
            Key::F22 => "F22".to_string(),
            Key::F23 => "F23".to_string(),
            Key::F24 => "F24".to_string(),
            Key::NumLock => "NumLock".to_string(),
            Key::ScrollLock => "ScrollLock".to_string(),
            Key::LeftShift => "LeftShift".to_string(),
            Key::RightShift => "RightShift".to_string(),
            Key::LeftControl => "LeftControl".to_string(),
            Key::RightControl => "RightControl".to_string(),
            Key::LeftAlt => "LeftAlt".to_string(),
            Key::RightAlt => "RightAlt".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[napi]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug)]
pub struct Hotkey {
    id: u32,
    key: Key,
    mods: Vec<Modifiers>,
}

impl Hotkey {
    pub fn new(mods: Vec<Modifiers>, key: Key) -> Self {
        let id = Hotkey::generate_hash(mods.clone(), key);

        Self {
            id,
            key,
            mods,
        }
    }

    pub fn generate_hash(mods: Vec<Modifiers>, key: Key) -> u32 {
        let mut hotkey_str = String::new();
        if mods.contains(&Modifiers::Shift) {
            hotkey_str.push_str("Shift+");
        }

        if mods.contains(&Modifiers::Control) {
            hotkey_str.push_str("Control+");
        }

        if mods.contains(&Modifiers::Alt) {
            hotkey_str.push_str("Alt+");
        }

        if mods.contains(&Modifiers::Super) {
            hotkey_str.push_str("Super+");
        }

        hotkey_str.push_str(&key.to_string());

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hotkey_str.hash(&mut hasher);
        std::hash::Hasher::finish(&hasher) as u32
    }
}

enum GlobalHotkeyMessage {
    Register(Hotkey),
    Unregister(u32),
}


fn prepare_global_hotkey() {
    std::thread::spawn(move || {
        unsafe {
            let class_name = PCWSTR(encode_wide("global_hotkey_manager").as_ptr());
            let hinstance = GetModuleHandleW(None).unwrap();

            let mut wnd_class = WNDCLASSW::default();
            wnd_class.lpfnWndProc = Some(global_hotkey_proc);
            wnd_class.lpszClassName = class_name;
            wnd_class.hInstance = HINSTANCE(hinstance.0 as _);

            RegisterClassW(&wnd_class);

            let hwnd = CreateWindowExW(
                WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_TOOLWINDOW,
                class_name,
                None,
                WS_OVERLAPPED,
                CW_USEDEFAULT,
                0,
                CW_USEDEFAULT,
                0,
                HWND::default(),
                HMENU::default(),
                hinstance,
                None
            );

            let mut msg = std::mem::zeroed();

            loop {
                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).0 != 0 {
                    if msg.message == WM_QUIT {
                        break;
                    }

                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

                while let Ok(msg) = GLOBAL_HOTKEY_CHANNEL.1.try_recv() {
                    match msg {
                        GlobalHotkeyMessage::Register(hotkey) => {
                            let mut mods = MOD_NOREPEAT;

                            hotkey.mods.iter().for_each(|it| {
                                match it {
                                    Modifiers::Shift => mods |= MOD_SHIFT,
                                    Modifiers::Super | Modifiers::Meta => mods |= MOD_CONTROL,
                                    Modifiers::Alt => mods |= MOD_ALT,
                                    Modifiers::Control => mods |= MOD_CONTROL,
                                    _ => {}
                                }
                            });

                            // @todo: error handling
                            let _ = RegisterHotKey(hwnd, hotkey.id as i32, mods, hotkey.key as u32);
                        }
                        GlobalHotkeyMessage::Unregister(id) => {
                            // @todo: error handling
                            let _ = UnregisterHotKey(hwnd, id as i32);
                        }
                    }
                }
            }
        }
    });
}

#[napi]
pub fn register_hotkey(mods: Vec<Modifiers>, key: Key, callback: JsFunction) -> u32 {
    if !*GLOBAL_HOTKEY_PREPARED.lock().unwrap() {
        prepare_global_hotkey();
        *GLOBAL_HOTKEY_PREPARED.lock().unwrap() = true;
    }

    let tsfn: ThreadsafeFunction<()> = callback.create_threadsafe_function(0, |ctx| {
        Ok(vec![0])
    }).unwrap();

    let hotkey = Hotkey::new(mods, key);
    let mut callbacks = GLOBAL_HOTKEY_CALLBACKS.lock().unwrap();
    let id = hotkey.id;
    let vec: &mut Vec<ThreadsafeFunction<()>> = callbacks.entry(id).or_insert(Vec::new());
    vec.push(tsfn);

    GLOBAL_HOTKEY_CHANNEL.0.send(GlobalHotkeyMessage::Register(hotkey)).unwrap();

    id
}

#[napi]
pub fn unregister_hotkey(id: u32) {
    let mut callbacks = GLOBAL_HOTKEY_CALLBACKS.lock().unwrap();
    callbacks.remove(&id);

    GLOBAL_HOTKEY_CHANNEL.0.send(GlobalHotkeyMessage::Unregister(id)).unwrap();
}

lazy_static! {
    static ref GLOBAL_HOTKEY_PREPARED: Mutex<bool> = Mutex::new(false);
    static ref GLOBAL_HOTKEY_CHANNEL: (Sender<GlobalHotkeyMessage>, Receiver<GlobalHotkeyMessage>) = unbounded();
    static ref GLOBAL_HOTKEY_CALLBACKS: Mutex<HashMap<u32, Vec<ThreadsafeFunction<()>>>> = Mutex::new(HashMap::new());
}

unsafe extern "system" fn global_hotkey_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_HOTKEY {
        let id = wparam.0 as u32;

        let callbacks = GLOBAL_HOTKEY_CALLBACKS.lock().unwrap();
        if let Some(vec) = callbacks.get(&id) {
            vec.iter().for_each(|it| {
                it.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
            });
        }
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
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