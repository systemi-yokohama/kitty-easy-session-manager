use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

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

/// Run fzf with the given items. Returns (key_pressed, selected_item).
/// key_pressed is empty string on plain Enter.
pub fn run_fzf(items: &[String]) -> Result<(String, String), String> {
    let input = items.join("\n");

    let fzf = command_path("fzf", &["/opt/homebrew/bin/fzf", "/usr/local/bin/fzf"]);

    let mut child = Command::new(fzf)
        .args([
            "--prompt=Sessions> ",
            "--layout=reverse",
            "--border=rounded",
            "--height=40%",
            "--expect=ctrl-n,ctrl-r,ctrl-d",
            "--header=enter:open, ctrl-n:create new, ctrl-r:rename, ctrl-d:delete, esc: quit",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Could not start fzf ({}). Is it installed and on PATH?", err))?;

    // Write items to fzf stdin then close it.
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).ok();
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Failed to wait for fzf: {}.", err))?;

    // fzf exits 130 on Esc and 1 when nothing matched; only 2 means it failed.
    if output.status.code() == Some(2) {
        return Err("fzf reported an error.".to_string());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut lines = text.lines();

    let key = lines.next().unwrap_or("").to_string();
    let target = lines.next().unwrap_or("").to_string();

    Ok((key, target))
}

pub fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().ok();
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).ok();
    line.trim().to_string()
}

/// Tell the user something and wait, since the picker's screen is redrawn right after.
pub fn show_message(message: &str) {
    prompt(&format!("{} Press Enter to go back.", message));
}

/// Tell the user why the picker cannot continue, then exit.
pub fn fatal(message: &str) -> ! {
    prompt(&format!("{} Press Enter to close.", message));
    std::process::exit(1);
}
