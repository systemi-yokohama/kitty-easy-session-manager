# Kitty Easy Session Manager (Kitty ESM)

<div align="center">

##### Make it esay to handle sessions on Kitty Terminal

[![Rust](https://img.shields.io/badge/rust-%23E05D44.svg?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org/)
[![Python](https://img.shields.io/badge/python-%233776AB.svg?style=for-the-badge&logo=python&logoColor=white)](https://www.python.org/)
[![FZF](https://img.shields.io/badge/fzf-%23ED417D.svg?style=for-the-badge&logo=fzf&logoColor=white)](https://github.com/junegunn/fzf)

<img src="assets/esm-logo.png" alt="esm-logo" width="320" />

</div>

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

## Installation

> This project is still highly under development. The install flow is manual, paths are not configurable yet, and future versions may change the setup.

Clone this repository into your kitty config directory:

```sh
git clone https://github.com/systemi-yokohama/kitty-easy-session-manager ~/.config/kitty/easy-session-manager
cd ~/.config/kitty/easy-session-manager
```

Build the switcher binary from the cloned repository:

```sh
cargo build --release
```

Create the sessions directory:

```sh
mkdir -p ~/.config/kitty/sessions
```

Add the ESM watcher, tab bar, tab filtering, and key mappings to `~/.config/kitty/kitty.conf`:

```conf
# Autosave watcher
watcher $HOME/.config/kitty/easy-session-manager/watcher.py

# Show only tabs belonging to the active session, plus session-less tabs.
tab_bar_filter session:~

tab_bar_style custom
tab_powerline_style round
tab_bar_min_tabs 1

# Save the active session with a notification.
map cmd+s kitten $HOME/.config/kitty/easy-session-manager/watcher.py save-active-session-with-notification

# Open the session switcher in an overlay window.
map cmd+shift+s launch --type=overlay --allow-remote-control $HOME/.config/kitty/easy-session-manager/target/release/kitty-esm-switcher
```

Restart kitty after updating the config.

## Uninstallation

Remove the cloned repository:

```sh
rm -rf ~/.config/kitty/easy-session-manager
```

Remove the ESM lines from `~/.config/kitty/kitty.conf` if you added them:

```conf
watcher $HOME/.config/kitty/easy-session-manager/watcher.py
tab_bar_filter session:~
tab_bar_style custom
tab_powerline_style round
tab_bar_min_tabs 1
map cmd+s kitten $HOME/.config/kitty/easy-session-manager/watcher.py save-active-session-with-notification
map cmd+shift+s launch --type=overlay --allow-remote-control $HOME/.config/kitty/easy-session-manager/target/release/kitty-esm-switcher
```

Session files are stored in `~/.config/kitty/sessions`. Remove that directory only if you no longer need the saved sessions.

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
├── assets/
│   └── icon.svg
├── src/
│   ├── main.rs
│   ├── sessions.rs
│   └── ui.rs
├── tab_bar.py
└── watcher.py
```

## License

MIT
