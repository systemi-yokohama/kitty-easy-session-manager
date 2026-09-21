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
`Ctrl-r` on a session SHALL prompt for a new name. An empty name, the current name, a name that already exists, or a name that is not valid under the session name rules SHALL change nothing, and the reason SHALL be shown for the last two cases. Sessions SHALL be selected by their exact name: renaming `test` SHALL NOT affect `test2`.

If the session has no open tabs, only its file SHALL be renamed to `<new>.kitty-session`; nothing SHALL be opened and the picker SHALL be shown again.

If the session has open tabs, the switcher SHALL first warn that its tabs will be closed and re-opened from a saved snapshot, that running programs are started again and scrollback is lost, and ask for confirmation with a `[y/N]` prompt. On `y` or `Y` it SHALL: save a snapshot of that session to its file; rename the file; open the session under the new name and switch to it; and only when the renamed session is open, close the tabs of the old session. Afterwards the picker SHALL exit. If saving the snapshot, renaming the file or opening the renamed session fails, the switcher SHALL keep the old session and its file, undo any file rename, and tell the user. Any other answer SHALL change nothing.

_Verification: every scenario except "Invalid new name" was run against the implementation in a kitty 0.48.2 test instance on 2026-09-22, including renaming the session the user is in and a session with a space in its name; "Reopening fails" was simulated with a `kitten` wrapper that drops `goto_session`. "Invalid new name" is covered by the shared name-validation unit tests and by the same check on create; its live run was skipped because of a mistake in the test script._

#### Scenario: Rename to a free name
- **WHEN** the user presses `Ctrl-r` on `test`, which has no open tabs, and enters `scratch`
- **THEN** `test.kitty-session` no longer exists, `scratch.kitty-session` exists, kitty stays where it was and the picker is shown again

#### Scenario: Rename to an existing name
- **WHEN** the user presses `Ctrl-r` on `test` and enters `docker`, which exists
- **THEN** both files are unchanged and the switcher says the name is taken

#### Scenario: Rename an open session
- **WHEN** the user presses `Ctrl-r` on `test`, which has open tabs, enters `scratch` and answers `y` to the warning
- **THEN** `test.kitty-session` no longer exists, `scratch.kitty-session` holds the snapshot, kitty is in the `scratch` session, `test`'s original tabs are closed and the picker has exited

#### Scenario: Similar names are not mixed up
- **WHEN** sessions `test` and `test2` are both open and the user renames `test` to `scratch`
- **THEN** `scratch.kitty-session` contains only `test`'s tabs and `test2`'s tabs stay open and untouched

#### Scenario: Warning is declined
- **WHEN** the user presses `Ctrl-r` on an open session, enters a free name and answers `n`
- **THEN** nothing changes and the session's tabs stay open

#### Scenario: Reopening fails
- **WHEN** the renamed session cannot be opened after the file was renamed
- **THEN** the file is renamed back, the old session's tabs are still open, and the user is told the rename failed

#### Scenario: Invalid new name
- **WHEN** the user presses `Ctrl-r` on `test` and enters `a/b`
- **THEN** nothing changes and the switcher explains that `/` is not allowed

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

### Requirement: Session names
A session name SHALL be accepted when, after trimming surrounding whitespace, it is not empty, does not start with `.`, does not contain `/`, `\`, `'`, `"` or control characters, does not end with `.kitty-session`, and is not `[No Session]`. Spaces inside a name SHALL be allowed. When a name is refused, the switcher SHALL say why and create nothing. Every operation of the switcher (open, create, delete) SHALL work for any accepted name, including names with spaces.

_Verification: run against the implementation in a kitty 0.48.2 test instance on 2026-09-22: create, reopen and delete of `my project`; names with `.`, `+`, `[`, `(`; refused names (`../x`, `a/b`, `[No Session]`, `x.kitty-session`, `it's`, `.hid`) each showed a reason and created nothing. Trimming and the full rule set are covered by unit tests._

#### Scenario: Name with spaces
- **WHEN** the user creates a session named `my project`, opens it later from the picker, and deletes it
- **THEN** `my project.kitty-session` is created, opening switches to that session, and deleting removes the file and closes that session's tabs

#### Scenario: Name that would leave the sessions directory
- **WHEN** the user enters `../evil` or `a/b` as a new session name
- **THEN** the switcher explains that `/` is not allowed and nothing is created

#### Scenario: Name that collides with the pseudo entry or the file extension
- **WHEN** the user enters `[No Session]` or `notes.kitty-session`
- **THEN** the switcher refuses the name and nothing is created

#### Scenario: Surrounding whitespace
- **WHEN** the user enters `  blog  `
- **THEN** the session is created as `blog.kitty-session`
