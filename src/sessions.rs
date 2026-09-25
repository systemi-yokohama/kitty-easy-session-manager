use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::ui::{prompt, show_message};

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

/// Quote an argument for `kitten @ action`, which re-splits its arguments shell-style.
fn quote_action_arg(arg: &str) -> String {
    format!("'{}'", arg.replace('\'', "'\\''"))
}

/// Check a typed session name and return it trimmed, or say why it is refused.
pub fn validate_session_name(input: &str) -> Result<String, String> {
    let name = input.trim();
    if name.is_empty() {
        return Err("The name is empty.".to_string());
    }
    if name.starts_with('.') {
        return Err("The name must not start with '.'.".to_string());
    }
    if let Some(c) = name
        .chars()
        .find(|c| matches!(c, '/' | '\\' | '\'' | '"') || c.is_control())
    {
        let shown = match c {
            '/' => "a slash",
            '\\' => "a backslash",
            '\'' => "a single quote",
            '"' => "a double quote",
            _ => "control characters",
        };
        return Err(format!("The name must not contain {}.", shown));
    }
    if name.ends_with(SESSION_EXTENSION) {
        return Err(format!("The name must not end with '{}'.", SESSION_EXTENSION));
    }
    if name == NO_SESSION {
        return Err(format!("'{}' is reserved.", name));
    }
    Ok(name.to_string())
}

pub fn session_dir() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME is not set.".to_string())?;
    Ok(PathBuf::from(home).join(".config/kitty/sessions"))
}

/// A missing directory is an empty list; it is created when the first session is created.
pub fn list_sessions(dir: &Path) -> Result<Vec<String>, String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(format!(
                "Could not read the sessions directory '{}': {}.",
                dir.display(),
                err
            ))
        }
    };
    let mut sessions: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            name.strip_suffix(SESSION_EXTENSION).map(str::to_string)
        })
        .collect();
    sessions.sort();
    Ok(sessions)
}

pub fn session_filename(session: &str) -> String {
    format!("{}{}", session, SESSION_EXTENSION)
}

/// Poll until `condition` holds; kitty applies remote actions asynchronously.
fn wait_until(condition: impl Fn() -> bool) -> bool {
    for _ in 0..10 {
        if condition() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    condition()
}

/// Switch to a session. `kitten @ action goto_session` exits 0 even when it did nothing, so
/// success is decided from kitty's state: the user must end up focused in that session.
pub fn goto_session(dir: &Path, session: &str) -> Result<(), String> {
    let name = session.trim_end_matches(SESSION_EXTENSION);
    let path = dir.join(session);
    let output = kitten_command()
        .args([
            "@",
            "action",
            "goto_session",
            &quote_action_arg(&path.to_string_lossy()),
        ])
        .output()
        .map_err(|err| format!("Could not run kitten: {}.", err))?;
    if !output.status.success() {
        return Err(format!(
            "Could not open '{}': {}",
            name,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    if wait_until(|| in_session(name)) {
        Ok(())
    } else {
        Err(format!("Could not open '{}': kitty did not switch to it.", name))
    }
}

/// Go to a tab that belongs to no session, creating one when none exists. `session:^$` matches
/// tabs not created in a session; the active tab is always shown by `tab_bar_filter`.
pub fn goto_no_session() -> Result<(), String> {
    if !any_tab_matches("session:^$") {
        let output = kitten_command()
            .args(["@", "launch", "--type=tab", "--cwd=current"])
            .output()
            .map_err(|err| format!("Could not run kitten: {}.", err))?;
        if !output.status.success() {
            return Err(format!(
                "Could not create a tab: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
    }
    let status = kitten_command()
        .args(["@", "focus-tab", "--match", "session:^$"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|err| format!("Could not run kitten: {}.", err))?;
    if status.success() {
        Ok(())
    } else {
        Err("Could not go to a tab outside the sessions.".to_string())
    }
}

/// Returns true when the picker should exit: the session was created and switched to.
pub fn create_session(dir: &Path) -> bool {
    let input = prompt("New session name: ");
    if input.trim().is_empty() {
        return false;
    }
    let name = match validate_session_name(&input) {
        Ok(name) => name,
        Err(reason) => {
            show_message(&reason);
            return false;
        }
    };
    let filename = session_filename(&name);
    let path = dir.join(&filename);
    if path.exists() {
        show_message(&format!("Session '{}' already exists.", name));
        return false;
    }
    // Minimal session: one tab + launch (opens default shell).
    // "launch" with no args uses kitty's configured shell.
    // The kitty-unserialize-data form in saved sessions is for
    // restoring existing windows and must not be used here.
    if let Err(err) = fs::create_dir_all(dir).and_then(|_| fs::write(&path, "new_tab\nlaunch\n")) {
        show_message(&format!("Failed to create '{}': {}.", filename, err));
        return false;
    }
    match goto_session(dir, &filename) {
        Ok(()) => true,
        Err(err) => {
            show_message(&format!(
                "Session '{}' was created but could not be opened. {}",
                name, err
            ));
            false
        }
    }
}

/// Save the current state of the session `name` to its own file `session`.
/// `kitten @ action` may report success even when nothing matched, so success means the file
/// was actually rewritten.
fn save_snapshot(dir: &Path, name: &str, session: &str) -> bool {
    let path = dir.join(session);
    let before = fs::metadata(&path).and_then(|m| m.modified()).ok();
    let status = kitten_command()
        .args([
            "@",
            "action",
            "save_as_session",
            "--base-dir",
            &quote_action_arg(&dir.to_string_lossy()),
            "--save-only",
            "--use-foreground-process",
            &quote_action_arg(&format!("--match={}", session_match(name))),
            &quote_action_arg(session),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if !status.map(|s| s.success()).unwrap_or(false) {
        return false;
    }
    match (fs::metadata(&path).and_then(|m| m.modified()).ok(), before) {
        (Some(after), Some(before)) => after > before,
        (Some(_), None) => true,
        (None, _) => false,
    }
}

/// Returns true when the picker must exit: renaming an open session switches to the renamed
/// session, leaving the picker's overlay behind in a tab the user is no longer in.
pub fn rename_session(dir: &Path, session: &str) -> bool {
    let old = session.trim_end_matches(SESSION_EXTENSION);
    let input = prompt(&format!("Rename '{}' to: ", old));
    if input.trim().is_empty() {
        return false;
    }
    let new = match validate_session_name(&input) {
        Ok(name) => name,
        Err(reason) => {
            show_message(&reason);
            return false;
        }
    };
    if new == old {
        return false;
    }
    let new_filename = session_filename(&new);
    let new_path = dir.join(&new_filename);
    let old_path = dir.join(session);
    if new_path.exists() {
        show_message(&format!("'{}' already exists.", new));
        return false;
    }

    // Nothing is running in a session that is not open, so only the file changes.
    if !session_is_open(old) {
        if let Err(err) = fs::rename(&old_path, &new_path) {
            show_message(&format!("Failed to rename '{}': {}.", old, err));
        }
        return false;
    }

    let confirm = prompt(&format!(
        "Renaming '{}' to '{}' closes its open tabs and re-opens them from a saved snapshot. \
         Running programs are started again and scrollback is lost. Continue? [y/N]: ",
        old, new
    ));
    if !confirm.eq_ignore_ascii_case("y") {
        return false;
    }

    // The old tabs are closed last, only once the renamed session is open, so a failure at any
    // earlier step leaves the old session as it was.
    if !save_snapshot(dir, old, session) {
        show_message(&format!(
            "Could not save a snapshot of '{}'. Nothing was changed.",
            old
        ));
        return false;
    }
    if let Err(err) = fs::rename(&old_path, &new_path) {
        show_message(&format!("Failed to rename '{}': {}.", old, err));
        return false;
    }
    if goto_session(dir, &new_filename).is_err() {
        let undone = fs::rename(&new_path, &old_path).is_ok();
        show_message(&format!(
            "Could not open '{}'. {}",
            new,
            if undone {
                format!("The rename was undone and '{}' is unchanged.", old)
            } else {
                format!("Its file is now '{}'; '{}' is still open.", new_filename, old)
            }
        ));
        return false;
    }
    close_session(old);
    true
}

/// Escape a session name so it can be used inside a kitty `session:` match expression.
fn regex_escape(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_whitespace() {
            // A literal space would split the match expression into two tokens.
            out.push_str("\\s");
            continue;
        }
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
        .args(["@", "action", "close_session", &quote_action_arg(name)])
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
        show_message(&format!(
            "Cannot delete '{}': you are in it and there is no other tab to move to.",
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
        show_message(&format!("Failed to delete '{}': {}.", name, err));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_plain_and_spaced_names() {
        assert_eq!(validate_session_name("blog").unwrap(), "blog");
        assert_eq!(validate_session_name("  my project  ").unwrap(), "my project");
        assert_eq!(validate_session_name("a.b+c[1](2)").unwrap(), "a.b+c[1](2)");
    }

    #[test]
    fn refuses_unsafe_names() {
        for bad in [
            "", "   ", ".hidden", "..", "../evil", "a/b", "a\\b", "it's", "say \"hi\"",
            "tab\there", "notes.kitty-session", NO_SESSION,
        ] {
            assert!(validate_session_name(bad).is_err(), "{:?} should be refused", bad);
        }
    }

    #[test]
    fn regex_escape_keeps_names_one_token() {
        assert_eq!(regex_escape("my proj"), "my\\sproj");
        assert_eq!(regex_escape("a.b+c"), "a\\.b\\+c");
        assert_eq!(regex_escape("x[1](2)"), "x\\[1\\]\\(2\\)");
        assert_eq!(session_match("test"), "session:^test$");
        assert!(!session_match("my proj").contains(' '));
    }

    #[test]
    fn missing_sessions_directory_is_an_empty_list() {
        let dir = std::env::temp_dir().join("esm-test-definitely-missing-dir");
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(list_sessions(&dir).unwrap(), Vec::<String>::new());
    }

    #[test]
    fn lists_only_session_files_sorted() {
        let dir = std::env::temp_dir().join(format!("esm-test-list-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for f in ["b.kitty-session", "a b.kitty-session", "notes.txt"] {
            fs::write(dir.join(f), "").unwrap();
        }
        assert_eq!(list_sessions(&dir).unwrap(), vec!["a b", "b"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn action_args_are_single_quoted() {
        assert_eq!(quote_action_arg("/a b/c.kitty-session"), "'/a b/c.kitty-session'");
        assert_eq!(quote_action_arg("it's"), "'it'\\''s'");
    }
}
