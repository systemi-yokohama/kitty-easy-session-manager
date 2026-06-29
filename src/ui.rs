use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

const FZF: &str = "/opt/homebrew/bin/fzf";

/// Run fzf with the given items. Returns (key_pressed, selected_item).
/// key_pressed is empty string on plain Enter.
pub fn run_fzf(items: &[String]) -> (String, String) {
    let input = items.join("\n");

    let mut child = Command::new(FZF)
        .args([
            "--prompt=Sessions> ",
            "--layout=reverse",
            "--border=rounded",
            "--height=40%",
            "--expect=ctrl-r,ctrl-d",
            "--header=enter:open  ctrl-r:rename  ctrl-d:delete",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to launch fzf - is it installed at /opt/homebrew/bin/fzf?");

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
