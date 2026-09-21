mod sessions;
mod ui;

use sessions::{
    create_session, delete_session, goto_no_session, goto_session, list_sessions, rename_session,
    session_dir, session_filename, CREATE_NEW, NO_SESSION,
};
use ui::run_fzf;

fn main() {
    let dir = session_dir();

    loop {
        let sessions = list_sessions(&dir);
        let mut items = sessions;
        items.insert(0, NO_SESSION.to_string());

        let (key, target) = run_fzf(&items);

        if key == "ctrl-n" {
            create_session(&dir);
            continue;
        }

        if target.is_empty() {
            break;
        }

        let is_pseudo = target == CREATE_NEW || target == NO_SESSION;

        let actual_target = if !is_pseudo {
            session_filename(&target)
        } else {
            target.clone()
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
                if target == CREATE_NEW {
                    create_session(&dir);
                } else if target == NO_SESSION {
                    goto_no_session();
                } else {
                    goto_session(&dir, &actual_target);
                }
                break;
            }
        }
    }
}
