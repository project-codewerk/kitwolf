//! Global keyboard/mouse hooks (F8 hotkey, click recording) and click replay.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT,
    KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MOUSEEVENTF_ABSOLUTE,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_VIRTUALDESK,
    MOUSE_EVENT_FLAGS, MOUSEINPUT, VIRTUAL_KEY, VK_CONTROL, VK_F8,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, GetSystemMetrics, SetWindowsHookExW, HC_ACTION, KBDLLHOOKSTRUCT,
    LLKHF_INJECTED, LLMHF_INJECTED, MSG, MSLLHOOKSTRUCT, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN,
    SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN, WM_LBUTTONDOWN,
    WM_SYSKEYDOWN,
};

use crate::game;

pub const HOTKEY: u16 = VK_F8.0;

/// Replay timing. Games often ignore clicks that arrive too fast after a cursor move.
const MOVE_SETTLE: Duration = Duration::from_millis(25);
const CLICK_HOLD: Duration = Duration::from_millis(20);

/// Left Ctrl as a hardware scan code, so games that read raw keyboard input see it too.
const LEFT_CTRL_SCANCODE: u16 = 0x1D;

pub enum HookEvent {
    Hotkey,
    Click { x: i32, y: i32, ctrl: bool },
}

/// Swallow the hotkey even when the game isn't focused (overlay open, recording, replaying).
pub static CAPTURE_HOTKEY: AtomicBool = AtomicBool::new(false);
/// Report left clicks while a kit is being recorded.
pub static RECORD_CLICKS: AtomicBool = AtomicBool::new(false);

static EVENTS: OnceLock<Sender<HookEvent>> = OnceLock::new();
static HOTKEY_HELD: AtomicBool = AtomicBool::new(false);
static HOTKEY_SWALLOWED: AtomicBool = AtomicBool::new(false);

/// Install the hooks on a dedicated thread (low-level hooks need a message loop).
pub fn start(events: Sender<HookEvent>) {
    let _ = EVENTS.set(events);
    thread::spawn(|| unsafe {
        let module: HINSTANCE = GetModuleHandleW(None).expect("module handle").into();
        let _keyboard = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), module, 0)
            .expect("failed to install keyboard hook");
        let _mouse = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), module, 0)
            .expect("failed to install mouse hook");
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {}
    });
}

fn send(event: HookEvent) {
    if let Some(events) = EVENTS.get() {
        let _ = events.send(event);
    }
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let info = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        if info.vkCode == HOTKEY as u32 && (info.flags & LLKHF_INJECTED).0 == 0 {
            let message = wparam.0 as u32;
            if message == WM_KEYDOWN || message == WM_SYSKEYDOWN {
                // Holding the key auto-repeats key-downs; only act on the first one.
                if !HOTKEY_HELD.swap(true, Ordering::SeqCst) {
                    let capture = CAPTURE_HOTKEY.load(Ordering::SeqCst) || game::foreground_game().is_some();
                    HOTKEY_SWALLOWED.store(capture, Ordering::SeqCst);
                    if capture {
                        send(HookEvent::Hotkey);
                    }
                }
            } else {
                HOTKEY_HELD.store(false, Ordering::SeqCst);
            }
            if HOTKEY_SWALLOWED.load(Ordering::SeqCst) {
                return LRESULT(1);
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32
        && wparam.0 as u32 == WM_LBUTTONDOWN
        && RECORD_CLICKS.load(Ordering::SeqCst)
    {
        let info = &*(lparam.0 as *const MSLLHOOKSTRUCT);
        if info.flags & LLMHF_INJECTED == 0 {
            let ctrl = GetAsyncKeyState(VK_CONTROL.0 as i32) < 0;
            send(HookEvent::Click { x: info.pt.x, y: info.pt.y, ctrl });
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

fn send_mouse(dx: i32, dy: i32, flags: MOUSE_EVENT_FLAGS) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT { dx, dy, mouseData: 0, dwFlags: flags, time: 0, dwExtraInfo: 0 },
        },
    };
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };
}

fn send_ctrl(flags: KEYBD_EVENT_FLAGS) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: LEFT_CTRL_SCANCODE,
                dwFlags: KEYEVENTF_SCANCODE | flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };
}

/// Move the cursor to screen position (x, y) and left-click, holding Ctrl if `ctrl`.
pub fn click_at(x: i32, y: i32, ctrl: bool) {
    // Absolute mouse input uses 0..=65535 across the whole virtual desktop.
    let (left, top, width, height) = unsafe {
        (
            GetSystemMetrics(SM_XVIRTUALSCREEN),
            GetSystemMetrics(SM_YVIRTUALSCREEN),
            GetSystemMetrics(SM_CXVIRTUALSCREEN).max(2),
            GetSystemMetrics(SM_CYVIRTUALSCREEN).max(2),
        )
    };
    let nx = ((x - left) as i64 * 65535 / (width - 1) as i64) as i32;
    let ny = ((y - top) as i64 * 65535 / (height - 1) as i64) as i32;

    send_mouse(nx, ny, MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK);
    if ctrl {
        send_ctrl(KEYBD_EVENT_FLAGS(0));
    }
    thread::sleep(MOVE_SETTLE);
    send_mouse(0, 0, MOUSEEVENTF_LEFTDOWN);
    thread::sleep(CLICK_HOLD);
    send_mouse(0, 0, MOUSEEVENTF_LEFTUP);
    if ctrl {
        thread::sleep(CLICK_HOLD);
        send_ctrl(KEYEVENTF_KEYUP);
    }
}
