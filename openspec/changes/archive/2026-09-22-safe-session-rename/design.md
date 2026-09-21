# Design

## Context

See `proposal.md`. Today `rename_session` (`src/sessions.rs`) saves a snapshot with `--match session:<old>`, renames the file, opens the new session and closes the old tabs with `close-tab --match session:<old>`. Observed in a kitty 0.48.2 test instance (2026-09-21): the unanchored match also selects `test2`; anchored `session:^test$` writes exactly `test`'s tab; `goto_session <new file>` opens the renamed session; `close_session <old name>` closes only the old one and stops it being re-saved (see `fix-session-delete`). The picker's overlay window belongs to no session, so it is not closed by `close_session` and must exit by itself when the user's tab was the old session.

## Goals / Non-Goals

**Goals:**
- Never lose or mix sessions during a rename; make the one destructive case (open session) explicit and confirmed.
- Every failure before the old tabs are closed leaves the old session intact.

**Non-Goals:**
- Renaming a live session without restarting its programs. kitty binds a tab to the session name it was created with; no way to change that was found, so a restart is unavoidable.
- Preserving scrollback or running programs across the restart.

## Decisions

### Order: snapshot, rename file, open new, verify, close old
1. Snapshot the old session to its own file with `save_as_session` (anchored, escaped match).
2. Rename the file.
3. `goto_session` the new file.
4. Check the new session is open (`ls --match-tab` on its name); if not, rename the file back and stop.
5. `close_session <old>`.
6. Exit the picker.

The old tabs are closed last, only after the replacement exists, so a failure at any earlier step leaves the user with their old session. Alternative considered: close old first (today's order is open-then-close, but with no check). Rejected: no way back if opening fails.

### Not-open sessions: rename the file only
A session with no tabs has nothing to restart, so no warning and no opening. This changes today's behavior, where renaming also opened the session; the user agreed to "rename the file only" for this case.

### Warning without listing programs
The warning is generic ("open tabs will be closed and re-opened; running programs are restarted; scrollback is lost") and does not enumerate programs, which would need process inspection and JSON parsing. Alternative considered: list foreground programs. Rejected as more machinery for the same decision.

### Exit the picker after renaming an open session
After the switch, the picker's overlay lives in a tab the user left (or in the closed session's orphaned tab). It exits, as in `fix-session-delete`. Alternative considered: stay in the picker. Rejected: it would sit in the wrong tab.

### Validation and quoting come from `safe-session-names`
This change adds no escaping or naming logic of its own; it depends on that change's helpers.

## Risks / Trade-offs

- [Snapshot misses state that `save_as_session` cannot capture (unsaved editor buffers, shell state)] → the warning says programs are restarted; the confirmation defaults to No.
- [`goto_session` on the new file appears to succeed but opens nothing] → step 4 checks that the new session is open before anything is closed.
- [A crash between steps 2 and 5 leaves both an old open session and a renamed file] → the old session is still open with its tabs; a later save would recreate the old file. Accepted: the window is short and the state is recoverable by hand. If this proves real, add a marker.

## Migration Plan

None. Rollback is reverting the commit.

## Open Questions

- After renaming a not-open session, should the picker keep the cursor on the renamed entry? A cosmetic detail that can be decided later without changing the specs.
