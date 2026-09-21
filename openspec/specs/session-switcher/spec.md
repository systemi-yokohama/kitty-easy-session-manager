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
`Ctrl-d` on a session SHALL ask for confirmation with a `[y/N]` prompt. Answering `y` or `Y` SHALL remove the session file. Any other answer SHALL keep it. The picker SHALL be shown again afterwards.

#### Scenario: Confirm deletion
- **WHEN** the user presses `Ctrl-d` on `test` and answers `y`
- **THEN** `test.kitty-session` is removed and the picker no longer lists `test`

#### Scenario: Decline deletion
- **WHEN** the user presses `Ctrl-d` on `test` and answers `n` or just presses Enter
- **THEN** `test.kitty-session` is kept

### Requirement: Leave the picker
Pressing Esc, or pressing Enter with no item selected, SHALL close the picker without changing anything.

#### Scenario: Escape
- **WHEN** the user presses Esc in the picker
- **THEN** the picker closes and the current session is unchanged
