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
pub fn run_fzf(items: &[String]) -> (String, String) {
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
        .expect("Failed to launch fzf - is it installed and on PATH?");

    // Write items to fzf stdin then close it.
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).ok();
    }

    let output = child.wait_with_output().expect("Failed to wait for fzf");

    let text = String::from_utf8_lossy(&output.stdout);
    let mut lines = text.lines();

    let key = lines.next().unwrap_or("").to_string();
    let target = lines.next().unwrap_or("").to_string();

    (key, target)
}

pub fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().ok();
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).ok();
    line.trim().to_string()
}
