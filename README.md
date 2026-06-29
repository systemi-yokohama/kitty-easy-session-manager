# Kitty Easy Session Manager

A small session manager for [kitty](https://sw.kovidgoyal.net/kitty/) that makes named kitty sessions easier to create, switch, rename, delete, display, and save.

The project currently contains three pieces:

- `kitty-esm-switcher`: a Rust CLI that opens an `fzf` picker for kitty session files.
- `tab_bar.py`: a custom kitty tab bar renderer that shows the active session name before the normal tabs.
- `watcher.py`: a kitty watcher/kitten helper that saves loaded sessions and can save on quit.

## Current Assumptions

This is currently tailored to a personal kitty setup:

- Session files are stored in `~/.config/kitty/sessions`.
- Session files use the `.kitty-session` extension.
- `fzf` is expected to be available on `PATH`.
- Nerd Fonts are usable on your terminal to display the session icon on tab bar.
- Notifications use `osascript`, so the watcher notification path is macOS-specific.

These paths are not configurable yet.

## Requirements

- kitty
- Rust toolchain
- `fzf`
- macOS, if you want the current notification behavior in `watcher.py`

## Build

```sh
cargo build --release
```

The binary is created at:

```text
target/release/kitty-esm-switcher
```

## Usage

Run the switcher from a kitty window:

```sh
target/release/kitty-esm-switcher
```

The picker supports:

- `Enter`: open the selected session, or create a new one from `[+ New Session]`
- `Ctrl-r`: rename the selected session
- `Ctrl-d`: delete the selected session

New sessions are written as minimal kitty session files containing one tab and one launched shell.

## Kitty Files

### Tab Bar

`tab_bar.py` is intended to be used as kitty's custom tab bar file. It draws a session block before the regular tabs.

Relevant kitty config:

```conf
tab_bar_style custom
tab_powerline_style round
tab_bar_min_tabs 1
```

### Watcher

`watcher.py` uses kitty's Python APIs to save loaded sessions into the session directory. It can be wired into kitty as a watcher or invoked as a kitten-style helper, depending on your kitty config.

The watcher saves each loaded session with a `.kitty-session` filename and uses kitty's `save_as_session` behavior with foreground process tracking enabled.

## Repository Layout

```text
.
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── src/
│   ├── main.rs
│   ├── sessions.rs
│   └── ui.rs
├── tab_bar.py
└── watcher.py
```

## License

MIT
