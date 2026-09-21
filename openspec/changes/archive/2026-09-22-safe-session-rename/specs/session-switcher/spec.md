# Spec Delta

## MODIFIED Requirements

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
