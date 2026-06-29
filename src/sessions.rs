use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ui::prompt;

pub const CREATE_NEW: &str = "[+ New Session]";
const SESSION_EXTENSION: &str = ".kitty-session";

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
    let status = Command::new("kitten")
        .args(["@", "action", "goto_session", path.to_str().unwrap()])
        .status();
    if status.map(|s| !s.success()).unwrap_or(true) {
        eprintln!("Warning: goto_session failed for '{}'", session);
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
    Command::new("kitten")
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

    Command::new("kitten")
        .args([
            "@",
            "close-tab",
            "--match",
            &format!("session:{}", old_stem),
        ])
        .status()
        .ok();
}

pub fn delete_session(dir: &Path, session: &str) {
    let confirm = prompt(&format!("Delete '{}'? [y/N]: ", session));
    if confirm.eq_ignore_ascii_case("y") {
        fs::remove_file(dir.join(session)).expect("Failed to delete session");
        println!("Deleted '{}'", session);
    }
}
