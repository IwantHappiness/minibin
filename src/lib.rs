use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows_sys::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows_sys::Win32::UI::Shell::{SHEmptyRecycleBinW, ShellExecuteW};
use windows_sys::Win32::UI::WindowsAndMessaging::{DefWindowProcW, SW_SHOW, WM_LBUTTONDOWN};

pub enum Message {
    Exit,
    Open,
    Empty,
    Settings,
}

#[allow(unused)]
pub enum Icon {
    Empty = 0,
    Quarter = 25,
    Half = 50,
    ThreeQuarters = 75,
    Full = 100,
}

pub fn open_trash() {
    let recycle_bin_path: Vec<u16> = OsStr::new("::{645FF040-5081-101B-9F08-00AA002F954E}")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            std::ptr::null(),
            recycle_bin_path.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOW,
        );
    }
}

pub fn clear_trash() {
    let recycle_bin_path: Vec<u16> = OsStr::new("")
        .encode_wide()
        .chain(std::iter::once(0)) // Добавляем нулевой символ в конец
        .collect();

    unsafe {
        SHEmptyRecycleBinW(std::ptr::null_mut(), recycle_bin_path.as_ptr(), 0);
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) {
    if msg == 1 {
        let event = (lparam & 0xFFFF) as u32;
        match event {
            WM_LBUTTONDOWN => {
                open_trash();
            }

            WM_RBUTTONUP => {}
            _ => {}
        }
    }

    unsafe {
        DefWindowProcW(hwnd, msg, wparam, lparam);
    }
}
