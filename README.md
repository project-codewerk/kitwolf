# Kitwolf

An in-game overlay for **WARDOGS** on Windows. Record your kit purchase once, then rebuy it with a single key press.

Kitwolf is a fan-made tool. It isn't affiliated with or endorsed by the makers of WARDOGS.

## Download

Get the latest `.msi` installer from the [**Releases**](../../releases/latest) page. Run it, then start Kitwolf from the Start menu.

Requirements: Windows 10 or 11, and the Microsoft Edge WebView2 runtime. WebView2 ships with Windows 11 and current Windows 10 builds; the installer fetches it if it's missing.

## Using it

1. Run WARDOGS in **borderless windowed** mode. Exclusive fullscreen hides every overlay.
2. In game, press `F8` to open the overlay.
3. **Record new kit.** The overlay steps aside. Buy your kit as usual; every left click in the game is recorded. Press `F8` to finish, then name the kit.
4. **Ctrl+click the bag.** Buying a bag opens a popup, and the next click has to wait for it. Hold Ctrl when you click to buy the bag. Kitwolf records that it was a Ctrl+click.
5. **Rebuy.** Press `F8` and pick a kit, or press `1`–`9`. The overlay closes and the clicks are replayed. Press `F8` to cancel a replay. Alt-tabbing away also stops it.

The Kitwolf window that opens alongside the overlay lists your kits and lets you delete them. Closing it quits the app.

`F8` is only intercepted while WARDOGS (or the overlay) is focused, so other apps still receive it.

Kits are saved to `%APPDATA%\uk.co.codewerk.kitwolf\kits.json` as screen coordinates. Re-record them if you change resolution or UI scale.

### Replay timing

Kitwolf can't tell what each click does. A Ctrl+click is how you mark the one that needs time (the bag purchase):

| Kit | Plain click | Ctrl+click |
|---|---|---|
| Has Ctrl+clicks | click, then 100 ms | wait 400 ms, click (with Ctrl held), then 100 ms |
| No Ctrl+clicks | wait 400 ms, click, then 100 ms (every click plays safe) | — |

The delays are `REPLAY_CTRL_WAIT` and `REPLAY_CLICK_GAP` in [`src-tauri/src/main.rs`](src-tauri/src/main.rs).

## Building from source

Kitwolf is built with [Tauri v1](https://v1.tauri.app/), Vue 3 and Vite.

### Prerequisites

- [Node.js](https://nodejs.org/) 18 or newer
- [Rust](https://rustup.rs/) (stable, MSVC toolchain)
- Microsoft C++ Build Tools ("Desktop development with C++")
- WebView2 runtime

See the [Tauri v1 prerequisites guide](https://v1.tauri.app/v1/guides/getting-started/prerequisites) for details.

### Run in development

```sh
npm install
npm run tauri dev
```

To preview the UI in a plain browser without Tauri, run `npm run dev` and open `http://localhost:3440/?mode=menu`. Other options are `?mode=naming`, `?mode=recording` and `?view=control`. These use mock data.

### Build the installer

```sh
npm run tauri build
```

The `.msi` is written to `src-tauri/target/release/bundle/msi/`.

## How it works

Two windows (see [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json)):

| Label     | What it is |
| --------- | ---------- |
| `main`    | The overlay. Transparent, undecorated, always-on-top, stretched over the monitor. It's layered and `WS_EX_TRANSPARENT` (click-through) except while the menu is open. |
| `control` | Normal window: instructions and kit management. Closing it quits the app. |

The Rust side ([`src-tauri/src/`](src-tauri/src/)) owns all behaviour. The Vue UI only renders the state it's sent.

| File        | Responsibility |
| ----------- | -------------- |
| `main.rs`   | Mode state machine (`idle → menu → recording → replaying`) and the commands the UI calls. Sends a `state-changed` event after every change. |
| `input.rs`  | Low-level keyboard and mouse hooks (F8, recording left clicks) and click replay via `SendInput`. |
| `game.rs`   | Checks whether the foreground window is WARDOGS (by process name, falling back to window title) and moves focus between the game and the overlay. |
| `kits.rs`   | Loads and saves kits to JSON. |

The frontend is in [`src/`](src/). `components/ui/` holds the shared UI components (Button, Panel, Badge, Input, Icon…) and `styles/tokens/` holds the design tokens.
