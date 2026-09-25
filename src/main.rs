mod sessions;
mod ui;

use sessions::{
    create_session, delete_session, goto_no_session, goto_session, list_sessions, rename_session,
    session_dir, session_filename, NO_SESSION,
};
use ui::{fatal, run_fzf, show_message};

fn main() {
    let dir = session_dir().unwrap_or_else(|err| fatal(&err));

    loop {
        let mut items = list_sessions(&dir).unwrap_or_else(|err| fatal(&err));
        items.insert(0, NO_SESSION.to_string());

        let (key, target) = run_fzf(&items).unwrap_or_else(|err| fatal(&err));

        if key == "ctrl-n" {
            if create_session(&dir) {
                // Switched to the new session; the picker's overlay is in the tab we left.
                break;
            }
            continue;
        }

        if target.is_empty() {
            break;
        }

        let is_pseudo = target == NO_SESSION;
        let actual_target = if is_pseudo {
            target.clone()
        } else {
            session_filename(&target)
        };

        match key.as_str() {
            "ctrl-r" => {
                if !is_pseudo && rename_session(&dir, &actual_target) {
                    // The session was re-opened under its new name; leave the old tab.
                    break;
                }
                // Loop back to show the picker again
            }
            "ctrl-d" => {
                if !is_pseudo && delete_session(&dir, &actual_target) {
                    // We were in the deleted session; exiting closes the picker's overlay.
                    break;
                }
                // Loop back
            }
            _ => {
                // Plain Enter
                let result = if is_pseudo {
                    goto_no_session()
                } else {
                    goto_session(&dir, &actual_target)
                };
                match result {
                    Ok(()) => break,
                    Err(err) => show_message(&err),
                }
            }
        }
    }
}
