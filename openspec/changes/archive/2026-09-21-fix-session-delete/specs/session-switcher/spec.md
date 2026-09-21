# Spec Delta

## MODIFIED Requirements

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
