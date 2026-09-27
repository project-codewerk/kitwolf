#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod game;
mod input;
mod kits;

use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Manager, Window, WindowEvent};

use input::HookEvent;
use kits::{Click, Kit, KitStore};

/// Pause after handing focus back to the game before the first replayed click.
const REPLAY_FOCUS_SETTLE: Duration = Duration::from_millis(150);
/// Pause after every click.
const REPLAY_CLICK_GAP: Duration = Duration::from_millis(100);
/// Extra pause before a Ctrl+click, e.g. to let the buy menu catch up. A kit without any
/// Ctrl+clicks waits this before every click.
const REPLAY_CTRL_WAIT: Duration = Duration::from_millis(400);

fn waits_before(click: &Click, kit_has_ctrl: bool) -> bool {
    click.ctrl || !kit_has_ctrl
}

#[derive(Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Mode {
    /// Overlay is invisible and click-through.
    Idle,
    /// Overlay menu is open and takes input.
    Menu,
    /// Left clicks in the game are being captured; F8 finishes.
    Recording,
    /// A kit is being clicked through; F8 cancels.
    Replaying,
}

struct Session {
    mode: Mode,
    /// The game window the overlay was opened over; focus returns here.
    game: Option<isize>,
    captured: Vec<Click>,
    /// A finished recording waiting to be named.
    pending: Option<Vec<Click>>,
    store: KitStore,
}

struct Kitwolf {
    session: Mutex<Session>,
    overlay_hwnd: AtomicIsize,
    cancel_replay: AtomicBool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    mode: Mode,
    kits: Vec<Kit>,
    /// Clicks captured so far in the current recording.
    captured: usize,
    /// How many of those were Ctrl+clicks.
    captured_ctrl: usize,
    /// Click count of a finished recording waiting to be named.
    pending: Option<usize>,
    /// How many of the pending clicks were Ctrl+clicks.
    pending_ctrl: usize,
}

impl Session {
    fn snapshot(&self) -> Snapshot {
        Snapshot {
            mode: self.mode,
            kits: self.store.kits.clone(),
            captured: self.captured.len(),
            captured_ctrl: self.captured.iter().filter(|click| click.ctrl).count(),
            pending: self.pending.as_ref().map(Vec::len),
            pending_ctrl: self.pending.iter().flatten().filter(|click| click.ctrl).count(),
        }
    }
}

fn kitwolf(app: &AppHandle) -> tauri::State<'_, Kitwolf> {
    app.state::<Kitwolf>()
}

fn emit_state(app: &AppHandle) {
    let snapshot = kitwolf(app).session.lock().unwrap().snapshot();
    let _ = app.emit_all("state-changed", snapshot);
}

/// Toggle whether the overlay lets mouse input through to the game.
fn set_click_through(hwnd: isize, click_through: bool) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::*;

    let hwnd = HWND(hwnd as *mut _);
    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let transparent = WS_EX_TRANSPARENT.0 as isize;
        let style = if click_through { style | transparent } else { style & !transparent };
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style);
    }
}

/// Switch mode, route input and focus accordingly, and tell the UI.
fn set_mode(app: &AppHandle, mode: Mode) {
    let state = kitwolf(app);
    let game = {
        let mut session = state.session.lock().unwrap();
        session.mode = mode;
        session.game
    };
    input::CAPTURE_HOTKEY.store(mode != Mode::Idle, Ordering::SeqCst);
    input::RECORD_CLICKS.store(mode == Mode::Recording, Ordering::SeqCst);

    let overlay_hwnd = state.overlay_hwnd.load(Ordering::SeqCst);
    set_click_through(overlay_hwnd, mode != Mode::Menu);
    if mode == Mode::Menu {
        game::bring_to_front(overlay_hwnd);
        if let Some(overlay) = app.get_window("main") {
            let _ = overlay.set_focus();
        }
    } else if let Some(game) = game {
        game::bring_to_front(game);
    }
    emit_state(app);
}

/// Clicks only count while the game (or the click-through overlay above it) is active.
fn game_is_active(app: &AppHandle) -> bool {
    let state = kitwolf(app);
    let foreground = game::foreground_window();
    state.session.lock().unwrap().game == Some(foreground)
        || foreground == state.overlay_hwnd.load(Ordering::SeqCst)
}

fn on_hotkey(app: &AppHandle) {
    let state = kitwolf(app);
    let mode = state.session.lock().unwrap().mode;
    match mode {
        Mode::Idle => {
            if let Some(game) = game::foreground_game() {
                state.session.lock().unwrap().game = Some(game);
                set_mode(app, Mode::Menu);
            }
        }
        Mode::Menu => set_mode(app, Mode::Idle),
        Mode::Recording => {
            {
                let mut session = state.session.lock().unwrap();
                let captured = std::mem::take(&mut session.captured);
                session.pending = Some(captured);
            }
            set_mode(app, Mode::Menu);
        }
        Mode::Replaying => state.cancel_replay.store(true, Ordering::SeqCst),
    }
}

fn on_click(app: &AppHandle, click: Click) {
    if !game_is_active(app) {
        return;
    }
    {
        let state = kitwolf(app);
        let mut session = state.session.lock().unwrap();
        if session.mode != Mode::Recording {
            return;
        }
        session.captured.push(click);
    }
    emit_state(app);
}

#[tauri::command]
fn get_state(app: AppHandle) -> Snapshot {
    kitwolf(&app).session.lock().unwrap().snapshot()
}

#[tauri::command]
async fn close_menu(app: AppHandle) {
    if kitwolf(&app).session.lock().unwrap().mode == Mode::Menu {
        set_mode(&app, Mode::Idle);
    }
}

#[tauri::command]
async fn start_recording(app: AppHandle) {
    {
        let state = kitwolf(&app);
        let mut session = state.session.lock().unwrap();
        session.captured.clear();
        session.pending = None;
    }
    set_mode(&app, Mode::Recording);
}

#[tauri::command]
async fn save_recording(app: AppHandle, name: String) -> Result<(), String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("Name the kit first.".into());
    }
    {
        let state = kitwolf(&app);
        let mut session = state.session.lock().unwrap();
        let clicks = session.pending.take().ok_or("Nothing recorded.")?;
        if let Err(e) = session.store.add(name, clicks.clone()) {
            session.pending = Some(clicks);
            return Err(e);
        }
    }
    emit_state(&app);
    Ok(())
}

#[tauri::command]
async fn discard_recording(app: AppHandle) {
    kitwolf(&app).session.lock().unwrap().pending = None;
    emit_state(&app);
}

#[tauri::command]
async fn delete_kit(app: AppHandle, id: String) -> Result<(), String> {
    kitwolf(&app).session.lock().unwrap().store.remove(&id)?;
    emit_state(&app);
    Ok(())
}

#[tauri::command]
async fn replay_kit(app: AppHandle, id: String) -> Result<(), String> {
    let clicks = {
        let state = kitwolf(&app);
        let session = state.session.lock().unwrap();
        if session.game.is_none() {
            return Err("Open the overlay from inside WARDOGS.".into());
        }
        session.store.get(&id).ok_or("Kit not found.")?.clicks.clone()
    };
    kitwolf(&app).cancel_replay.store(false, Ordering::SeqCst);
    set_mode(&app, Mode::Replaying);

    thread::spawn(move || {
        let kit_has_ctrl = clicks.iter().any(|click| click.ctrl);
        thread::sleep(REPLAY_FOCUS_SETTLE);
        for click in clicks {
            // Stop on F8, or if the player alt-tabbed away mid-replay.
            if kitwolf(&app).cancel_replay.load(Ordering::SeqCst) || !game_is_active(&app) {
                break;
            }
            if waits_before(&click, kit_has_ctrl) {
                thread::sleep(REPLAY_CTRL_WAIT);
                if kitwolf(&app).cancel_replay.load(Ordering::SeqCst) {
                    break;
                }
            }
            input::click_at(click.x, click.y, click.ctrl);
            thread::sleep(REPLAY_CLICK_GAP);
        }
        set_mode(&app, Mode::Idle);
    });
    Ok(())
}

/// Stretch the overlay window over the monitor it opened on.
fn cover_monitor(window: &Window) -> tauri::Result<()> {
    if let Some(monitor) = window.current_monitor()? {
        window.set_position(*monitor.position())?;
        window.set_size(*monitor.size())?;
    }
    Ok(())
}

/// Layered, always-on-top, click-through: input goes straight to the game.
fn make_overlay(window: &Window) -> tauri::Result<isize> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::*;

    let hwnd = window.hwnd()?.0;
    unsafe {
        let handle = HWND(hwnd as *mut _);
        let current = GetWindowLongPtrW(handle, GWL_EXSTYLE);
        let style = WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
        SetWindowLongPtrW(handle, GWL_EXSTYLE, current | style.0 as isize);
    }
    Ok(hwnd)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let overlay = app.get_window("main").unwrap();
            cover_monitor(&overlay)?;
            let overlay_hwnd = make_overlay(&overlay)?;

            let kits_path = app
                .path_resolver()
                .app_config_dir()
                .expect("no app config directory")
                .join("kits.json");
            app.manage(Kitwolf {
                session: Mutex::new(Session {
                    mode: Mode::Idle,
                    game: None,
                    captured: Vec::new(),
                    pending: None,
                    store: KitStore::load(kits_path),
                }),
                overlay_hwnd: AtomicIsize::new(overlay_hwnd),
                cancel_replay: AtomicBool::new(false),
            });

            // Hook callbacks must return fast, so they only forward events to this thread.
            let (events_tx, events_rx) = mpsc::channel();
            input::start(events_tx);
            let handle = app.handle();
            thread::spawn(move || {
                for event in events_rx {
                    match event {
                        HookEvent::Hotkey => on_hotkey(&handle),
                        HookEvent::Click { x, y, ctrl } => on_click(&handle, Click { x, y, ctrl }),
                    }
                }
            });
            Ok(())
        })
        .on_window_event(|event| {
            // The overlay can't be closed by the user, so closing the control window quits the app.
            if let WindowEvent::CloseRequested { .. } = event.event() {
                if event.window().label() == "control" {
                    event.window().app_handle().exit(0);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            close_menu,
            start_recording,
            save_recording,
            discard_recording,
            delete_kit,
            replay_kit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
