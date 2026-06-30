mod sessions;
mod ui;

use sessions::{
    create_session, delete_session, goto_session, list_sessions, rename_session, session_dir,
    session_filename, CREATE_NEW,
};
use ui::run_fzf;

fn main() {
    let dir = session_dir();

    loop {
        let sessions = list_sessions(&dir);
        let items = sessions;

        let (key, target) = run_fzf(&items);

        if key == "ctrl-n" {
            create_session(&dir);
            continue;
        }

        if target.is_empty() {
            break;
        }

        let actual_target = if target != CREATE_NEW {
            session_filename(&target)
        } else {
            target.clone()
        };

        match key.as_str() {
            "ctrl-r" => {
                if target != CREATE_NEW {
                    rename_session(&dir, &actual_target);
                }
                // Loop back to show the picker again
            }
            "ctrl-d" => {
                if target != CREATE_NEW {
                    delete_session(&dir, &actual_target);
                }
                // Loop back
            }
            _ => {
                // Plain Enter
                if target == CREATE_NEW {
                    create_session(&dir);
                } else {
                    goto_session(&dir, &actual_target);
                }
                break;
            }
        }
    }
}
