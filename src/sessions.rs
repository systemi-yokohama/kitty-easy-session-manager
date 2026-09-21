use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::ui::prompt;

pub const CREATE_NEW: &str = "[+ New Session]";
/// Pseudo entry for tabs that were not created from any session file
/// (e.g. the tabs kitty opens at startup, before anything is saved).
pub const NO_SESSION: &str = "[No Session]";
const SESSION_EXTENSION: &str = ".kitty-session";

fn command_path(command: &str, fallbacks: &[&str]) -> PathBuf {
    if let Some(path) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(command);
            if candidate.is_file() {
                return candidate;
            }
        }
    }

    fallbacks
        .iter()
        .map(PathBuf::from)
        .find(|path| path.is_file())
        .unwrap_or_else(|| PathBuf::from(command))
}

fn kitten_command() -> Command {
    Command::new(command_path(
        "kitten",
        &[
            "/Applications/kitty.app/Contents/MacOS/kitten",
            "/opt/homebrew/bin/kitten",
            "/usr/local/bin/kitten",
        ],
    ))
}

pub fn session_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".config/kitty/sessions")
}

pub fn list_sessions(dir: &Path) -> Vec<String> {
    let mut sessions: Vec<String> = fs::read_dir(dir)
        .expect("Failed to read session directory")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            name.strip_suffix(SESSION_EXTENSION).map(str::to_string)
        })
        .collect();
    sessions.sort();
    sessions
}

pub fn session_filename(session: &str) -> String {
    format!("{}{}", session, SESSION_EXTENSION)
}

pub fn goto_session(dir: &Path, session: &str) {
    let path = dir.join(session);
    let status = kitten_command()
        .args(["@", "action", "goto_session", path.to_str().unwrap()])
        .status();
    if status.map(|s| !s.success()).unwrap_or(true) {
        eprintln!("Warning: goto_session failed for '{}'", session);
    }
}

pub fn goto_no_session() {
    // `session:^$` matches tabs that were not created in a session. The active tab
    // is always shown by `tab_bar_filter`, so the tab stays visible once focused.
    let status = kitten_command()
        .args(["@", "focus-tab", "--match", "session:^$"])
        .status();
    if status.map(|s| !s.success()).unwrap_or(true) {
        eprintln!("Warning: no session-less tab to go to");
    }
}

pub fn create_session(dir: &Path) {
    let name = prompt("New session name: ");
    if name.is_empty() {
        return;
    }
    let filename = session_filename(&name);
    let path = dir.join(&filename);
    if path.exists() {
        eprintln!("Session '{}' already exists.", filename);
        return;
    }
    // Minimal session: one tab + launch (opens default shell).
    // "launch" with no args uses kitty's configured shell.
    // The kitty-unserialize-data form in saved sessions is for
    // restoring existing windows and must not be used here.
    fs::write(&path, "new_tab\nlaunch\n").expect("Failed to create session file");
    goto_session(dir, &filename);
}

pub fn rename_session(dir: &Path, session: &str) {
    let new_name = prompt(&format!(
        "Rename '{}' to: ",
        session.trim_end_matches(SESSION_EXTENSION)
    ));
    if new_name.is_empty() {
        return;
    }
    let new_filename = session_filename(&new_name);
    let new_path = dir.join(&new_filename);
    if new_path.exists() {
        eprintln!("'{}' already exists.", new_filename);
        return;
    }

    let old_stem = session.trim_end_matches(SESSION_EXTENSION);
    let old_path = dir.join(session);

    // Save current session state so it can be restored under the new name.
    kitten_command()
        .args([
            "@",
            "action",
            "save_as_session",
            "--base-dir",
            dir.to_str().unwrap(),
            "--save-only",
            "--use-foreground-process",
            "--match",
            &format!("session:{}", old_stem),
            session,
        ])
        .status()
        .ok();

    fs::rename(&old_path, &new_path).expect("Failed to rename session file");
    goto_session(dir, &new_filename);

    kitten_command()
        .args([
            "@",
            "close-tab",
            "--match",
            &format!("session:{}", old_stem),
        ])
        .status()
        .ok();
}

/// Escape regex metacharacters so a session name can be used inside a kitty `session:` match.
fn regex_escape(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if "\\.+*?()|[]{}^$#".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// True if any tab matches `expr` (`kitten @ ls --match-tab` exits non-zero otherwise).
fn any_tab_matches(expr: &str) -> bool {
    kitten_command()
        .args(["@", "ls", "--match-tab", expr])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn session_match(name: &str) -> String {
    format!("session:^{}$", regex_escape(name))
}

fn session_is_open(name: &str) -> bool {
    any_tab_matches(&session_match(name))
}

fn in_session(name: &str) -> bool {
    any_tab_matches(&format!("state:focused and {}", session_match(name)))
}

/// A tab outside the session that we can move to. The focused tab is excluded: it holds the
/// picker's overlay, which belongs to no session and would otherwise always match.
fn elsewhere_match(name: &str) -> String {
    format!("not state:focused and not {}", session_match(name))
}

fn other_tabs_exist(name: &str) -> bool {
    any_tab_matches(&elsewhere_match(name))
}

fn focus_tab(expr: &str) {
    kitten_command()
        .args(["@", "focus-tab", "--match", expr])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok();
}

/// Leave `name`: previous session first, then a session-less tab, then any other tab.
/// `goto_session -1` silently does nothing when kitty has no session history.
fn move_away_from(name: &str) {
    kitten_command()
        .args(["@", "action", "goto_session", "-1"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok();
    if !in_session(name) {
        return;
    }
    focus_tab("session:^$");
    if !in_session(name) {
        return;
    }
    focus_tab(&elsewhere_match(name));
}

/// Closes every window of the session. The picker's own overlay window belongs to no session,
/// so it is not closed here.
fn close_session(name: &str) {
    kitten_command()
        .args(["@", "action", "close_session", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok();
}

/// Returns true when the picker must exit: the user was in the deleted session, so the picker's
/// overlay sits in a tab that no longer belongs anywhere useful and closes with the process.
pub fn delete_session(dir: &Path, session: &str) -> bool {
    let name = session.trim_end_matches(SESSION_EXTENSION);
    let open = session_is_open(name);
    let current = open && in_session(name);

    // Closing the last tabs would quit kitty, so refuse before touching anything.
    if current && !other_tabs_exist(name) {
        prompt(&format!(
            "Cannot delete '{}': you are in it and there is no other tab to move to. Press Enter to go back.",
            name
        ));
        return false;
    }

    let mut message = format!("Delete '{}'?", name);
    if open {
        message.push_str(" Its open tabs will be closed.");
    }
    if current {
        message.push_str(
            " You will be moved to the previous session, or to [No Session] if there is none.",
        );
    }
    let confirm = prompt(&format!("{} [y/N]: ", message));
    if !confirm.eq_ignore_ascii_case("y") {
        return false;
    }

    if let Err(err) = fs::remove_file(dir.join(session)) {
        prompt(&format!(
            "Failed to delete '{}': {}. Press Enter to go back.",
            name, err
        ));
        return false;
    }

    if current {
        move_away_from(name);
    }
    if open {
        close_session(name);
    }
    current
}
