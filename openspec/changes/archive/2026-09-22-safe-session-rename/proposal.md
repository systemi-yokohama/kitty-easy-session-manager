# Proposal

## Why

Rename has three problems (register items #2 and #3 in the archived `baseline-specs`):

- It selects the old session with an unanchored, unescaped match. Renaming `test` also matches `test2`: in a live kitty the snapshot taken this way pulled `test2`'s tab into `test`'s file, and the final `close-tab` would close both sessions.
- For an open session it silently restarts everything: the tabs are re-launched from a snapshot (running programs are started again, scrollback is lost) and the old tabs are closed, with no warning.
- After renaming, the picker overlay stays behind in the old session's tab, and errors (`expect` panics) are invisible.

The behavior for open sessions was decided with the user: warn, then close and re-open the session under the new name.

## What Changes

- Rename selects the old session by an anchored, escaped match (using the helpers from `safe-session-names`, which this change depends on).
- Renaming a session that is not open only renames its file; nothing is opened and the picker is shown again.
- Renaming an open session shows a warning that its tabs will be closed and re-opened from a snapshot, asks for confirmation, then: saves a snapshot of the old session, renames the file, opens the session under the new name, and only if that succeeded closes the old session's tabs. The user ends in the renamed session and the picker exits.
- If any step fails before the old session is closed, the old session and its file are kept (the rename is rolled back) and the user is told.
- The new name follows the name rules of `safe-session-names`.

**BREAKING**: renaming a session that is not open no longer opens it afterwards.

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `session-switcher`: the "Rename a session" requirement is replaced by separate behavior for open and not-open sessions, with warning, safe ordering and rollback.

## Impact

- `src/sessions.rs` (`rename_session`), `src/main.rs` (picker exits after renaming an open session).
- README Usage: `Ctrl-r` description.
- Depends on `safe-session-names` (match/quoting helpers and name validation); apply that change first.
