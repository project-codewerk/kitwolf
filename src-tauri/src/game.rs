//! Finding the WARDOGS window and moving focus between it and the overlay.

use windows::core::PWSTR;
use windows::Win32::Foundation::{CloseHandle, HWND};
use windows::Win32::System::Threading::{
    AttachThreadInput, GetCurrentThreadId, OpenProcess, QueryFullProcessImageNameW,
    PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::WindowsAndMessaging::{
    BringWindowToTop, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    SetForegroundWindow,
};

/// A window belongs to the game if its executable name contains this (case-insensitive),
/// e.g. `WARDOGS.exe` or `WARDOGS-Win64-Shipping.exe`. When the process can't be queried
/// (e.g. anti-cheat protection), the window title is matched instead.
/// Override with `KITWOLF_GAME=notepad` to test without the game.
const GAME_MATCH: &str = "wardogs";

fn game_match() -> &'static str {
    static MATCH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    MATCH.get_or_init(|| {
        std::env::var("KITWOLF_GAME").map(|s| s.to_lowercase()).unwrap_or_else(|_| GAME_MATCH.into())
    })
}

pub fn foreground_window() -> isize {
    unsafe { GetForegroundWindow().0 as isize }
}

/// The game window, if it is the active window right now.
pub fn foreground_game() -> Option<isize> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        return None;
    }
    let is_game = match process_name(hwnd) {
        Some(name) => name.to_lowercase().contains(game_match()),
        None => window_title(hwnd).to_lowercase().contains(game_match()),
    };
    is_game.then_some(hwnd.0 as isize)
}

fn process_name(hwnd: HWND) -> Option<String> {
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == 0 {
        return None;
    }
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = [0u16; 1024];
        let mut len = buf.len() as u32;
        let result =
            QueryFullProcessImageNameW(process, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len);
        let _ = CloseHandle(process);
        result.ok()?;
        let path = String::from_utf16_lossy(&buf[..len as usize]);
        path.rsplit('\\').next().map(str::to_owned)
    }
}

fn window_title(hwnd: HWND) -> String {
    let mut buf = [0u16; 512];
    let len = unsafe { GetWindowTextW(hwnd, &mut buf) };
    String::from_utf16_lossy(&buf[..len.max(0) as usize])
}

/// Make `hwnd` the foreground window. Windows only lets the process that owns the
/// foreground window change it, so borrow the current foreground thread's input state.
pub fn bring_to_front(hwnd: isize) {
    unsafe {
        let target = HWND(hwnd as *mut _);
        let foreground_thread = GetWindowThreadProcessId(GetForegroundWindow(), None);
        let own_thread = GetCurrentThreadId();
        let attached = foreground_thread != 0
            && foreground_thread != own_thread
            && AttachThreadInput(own_thread, foreground_thread, true).as_bool();
        let _ = BringWindowToTop(target);
        let _ = SetForegroundWindow(target);
        if attached {
            let _ = AttachThreadInput(own_thread, foreground_thread, false);
        }
    }
}
