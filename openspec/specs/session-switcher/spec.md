# session-switcher Specification

## Purpose
The session switcher is an fzf picker, opened in a kitty overlay, that lets the user list, open, create, rename and delete saved kitty sessions, and jump back to tabs that belong to no session.

## Requirements

### Requirement: Session listing
The picker SHALL list `[No Session]` first, followed by every `<name>.kitty-session` file in `~/.config/kitty/sessions`, shown as `<name>` without the extension and sorted alphabetically. Files with any other extension SHALL NOT be listed.

#### Scenario: Saved sessions are listed
- **WHEN** the sessions directory contains `docker.kitty-session`, `config.kitty-session` and `notes.txt`
- **THEN** the picker shows `[No Session]`, `config`, `docker` in that order and does not show `notes`

### Requirement: Open a session
Pressing Enter on a session name SHALL switch kitty to that session, using the session file in the sessions directory, and close the picker.

#### Scenario: Open a saved session
- **WHEN** the user presses Enter on `docker`
- **THEN** kitty switches to the `docker` session and the picker closes

### Requirement: Return to session-less tabs
Pressing Enter on `[No Session]` SHALL focus a tab that does not belong to any session and close the picker. `Ctrl-r` and `Ctrl-d` on `[No Session]` SHALL do nothing and the picker SHALL stay open.

_Verification: verified in a kitty 0.48.2 test instance (2026-09-21): Enter on `[No Session]` focused the session-less tab and the picker exited. With no session-less tab present, it changed nothing and the picker exited silently._

#### Scenario: Go back to the startup tab
- **WHEN** the user started kitty without a session, switched to `docker`, and presses Enter on `[No Session]`
- **THEN** kitty focuses a tab that belongs to no session and the picker closes

#### Scenario: Rename is ignored on the pseudo entry
- **WHEN** the user presses `Ctrl-r` on `[No Session]`
- **THEN** no prompt appears, nothing changes and the picker is shown again

### Requirement: Create a session
`Ctrl-n` SHALL prompt for a session name and, if the name is non-empty and no session file with that name exists, create `<name>.kitty-session` containing one tab with one shell and switch kitty to it. An empty name or an existing name SHALL create nothing and overwrite nothing, and the picker SHALL be shown again.

#### Scenario: Create a new session
- **WHEN** the user presses `Ctrl-n` and enters `blog`
- **THEN** `blog.kitty-session` is created with one tab running the default shell and kitty switches to `blog`

#### Scenario: Name already taken
- **WHEN** the user presses `Ctrl-n` and enters the name of an existing session
- **THEN** the existing file is left unchanged and the picker is shown again

#### Scenario: Empty name
- **WHEN** the user presses `Ctrl-n` and submits an empty name
- **THEN** nothing is created and the picker is shown again

### Requirement: Rename a session
`Ctrl-r` on a session SHALL prompt for a new name. An empty name or a name that already exists SHALL change nothing. Otherwise the session file SHALL be renamed to `<new>.kitty-session` and kitty SHALL switch to the session under its new name.

#### Scenario: Rename to a free name
- **WHEN** the user presses `Ctrl-r` on `test` and enters `scratch`
- **THEN** `test.kitty-session` no longer exists, `scratch.kitty-session` exists and kitty is in the `scratch` session

#### Scenario: Rename to an existing name
- **WHEN** the user presses `Ctrl-r` on `test` and enters `docker`, which exists
- **THEN** both files are unchanged

### Requirement: Delete a session
`Ctrl-d` on a session SHALL ask for confirmation with a `[y/N]` prompt that names the session. If the session has open tabs, the prompt SHALL say that they will be closed. If the session is the one the user is currently in, the prompt SHALL additionally say that the user will be moved to the previously active session, or to a session-less tab when there is none.

Answering `y` or `Y` SHALL delete the session: remove its file and close all of its open tabs. When the user is currently in the session, the switcher SHALL first move them to the previously active session or, if kitty has no previous session, to a tab that belongs to no session, and only then close the session's tabs. If there is no tab outside the session to move to, the delete SHALL be refused: the file and the tabs SHALL be left unchanged and the user SHALL be told why.

Any other answer SHALL keep the session unchanged. The picker SHALL be shown again afterwards unless the user was in the deleted session, in which case the picker SHALL exit. A deleted session's file SHALL NOT reappear when sessions are saved later.

_Verification: every scenario below was run against the implementation in a kitty 0.48.2 test instance on 2026-09-21, including the fallback to any other tab when there is no history and no session-less tab. Names with whitespace are not covered._

#### Scenario: Confirm deletion
- **WHEN** the user presses `Ctrl-d` on `test`, which has no open tabs, and answers `y`
- **THEN** `test.kitty-session` is removed, no tab changes, and the picker no longer lists `test`

#### Scenario: Delete an open session the user is not in
- **WHEN** the user is in `config`, presses `Ctrl-d` on `docker`, which has open tabs, and answers `y`
- **THEN** the prompt warned that `docker`'s tabs will be closed, `docker.kitty-session` is removed, `docker`'s tabs are closed, the user stays in `config`, and the picker is shown again without `docker`

#### Scenario: Delete the current session with a previous session
- **WHEN** the user went from `config` to `docker`, presses `Ctrl-d` on `docker` and answers `y`
- **THEN** the prompt said the user would be moved to the previous session, the user lands in `config`, `docker`'s tabs are closed, and a later save does not recreate `docker.kitty-session`

#### Scenario: Delete the current session without a previous session
- **WHEN** kitty started with a session-less tab, the user opened `docker` once, presses `Ctrl-d` on `docker` and answers `y`
- **THEN** the user lands in the session-less tab and `docker`'s tabs are closed

#### Scenario: Nowhere to go
- **WHEN** the user is in `docker`, which holds every open tab, presses `Ctrl-d` on `docker` and answers `y`
- **THEN** the file and the tabs are left unchanged and the user is told the session cannot be deleted because there is no other tab to move to

#### Scenario: Decline deletion
- **WHEN** the user presses `Ctrl-d` on `test` and answers `n` or just presses Enter
- **THEN** `test.kitty-session` is kept and its tabs stay open

### Requirement: Leave the picker
Pressing Esc, or pressing Enter with no item selected, SHALL close the picker without changing anything.

#### Scenario: Escape
- **WHEN** the user presses Esc in the picker
- **THEN** the picker closes and the current session is unchanged
