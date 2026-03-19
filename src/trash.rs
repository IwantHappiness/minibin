use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows_sys::Win32::UI::{
    Shell::{SHEmptyRecycleBinW, SHQUERYRBINFO, SHQueryRecycleBinW, ShellExecuteW},
    WindowsAndMessaging::SW_SHOW,
};

// Get bin size and cout files in
pub fn recyle_bin_size() -> Option<(u64, u64)> {
    let mut info = SHQUERYRBINFO {
        cbSize: std::mem::size_of::<SHQUERYRBINFO>() as u32,
        i64Size: 0,
        i64NumItems: 0,
    };

    let result = unsafe { SHQueryRecycleBinW(std::ptr::null(), &mut info) };

    if result == 0 {
        return Some((info.i64Size as u64, info.i64NumItems as u64));
    }

    None
}

pub fn open_trash() {
    unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            std::ptr::null(),
            get_bin_path().as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOW,
        );
    }
}

pub fn clear_trash(flags: u32) {
    unsafe {
        SHEmptyRecycleBinW(std::ptr::null_mut(), std::ptr::null(), flags);
    }
}

pub fn get_bin_path() -> Vec<u16> {
    OsStr::new("shell:::{645FF040-5081-101B-9F08-00AA002F954E}")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
