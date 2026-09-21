# Design

## Context

See `proposal.md`. Today `delete_session` (`src/sessions.rs`) only calls `fs::remove_file`. The picker runs as an overlay inside the tab of the session the user is in. The overlay window belongs to no session.

Observed in a kitty 0.48.2 test instance (2026-09-21):
- `goto_session -1` moves to the previously active session; with no session history (only session-less tabs before) it does nothing and exits 0. The startup session-less tab is not part of that history.
- `close_session <name>` closes exactly that session's windows; for an unknown name it does nothing and exits 0.
- After `close_session`, `save-active-session-with-notification` no longer writes the deleted session's file.
- `close_session <name>` does **not** close the picker's overlay window (it belongs to no session), so after deleting the current session the picker survives in an otherwise orphaned tab unless it exits itself. Found during verification of the first implementation.
- `kitten @ ls --match-tab '<expr>'` exits 0 when some tab matches and 1 otherwise, so state can be queried without parsing JSON. `state:focused and session:^name$` tells whether the user is in a session; whether a tab outside it exists needs `not state:focused and not session:^name$`: plain `not session:^name$` also matches the focused tab, because its picker overlay is session-less (found during verification).

## Goals / Non-Goals

**Goals:**
- Delete ends the session: file gone, tabs gone, nothing re-saved.
- Never leave the user in a dead session, and never kill kitty by closing its last tab.

**Non-Goals:**
- Fixing the unanchored/unescaped session match used by rename (`safe-session-rename`); this change only uses an anchored, escaped match for its own queries.
- Showing error messages properly across the picker (`fix-switcher-flow-and-errors`); this change only needs a message the user can read before the picker exits.
- Names containing whitespace (TODO.md); see Risks.

## Decisions

### Order: check, confirm, remove file, move away, close tabs last
1. Query (`ls --match-tab`) whether the session has open tabs, whether the user is in it, and whether any tab exists outside it. If the user is in it and nothing exists outside it, refuse before anything is touched.
2. Show the confirmation (stronger wording when the session is open/current).
3. On `y`: remove the file; if the user is in the session, move away; then `close_session <name>`; and when the user was in the session, the picker exits (its overlay would otherwise stay in an orphaned tab).

Removing the file first means an interrupted run never leaves an open session with a stale file to be re-saved. Alternative considered: close first, then remove the file. Rejected: the file would outlive a failed close and could be re-saved.

### Moving away: `goto_session -1`, then a session-less tab, then any other tab
`goto_session -1` is the "previous session" the user asked for. Because it silently does nothing without history, the switcher checks afterwards whether the user is still in the session (`ls --match-tab 'state:focused and session:^name$'`) and falls back to `focus-tab --match session:^$` (the existing `[No Session]` behavior), then to any other tab (`focus-tab --match 'not state:focused and not session:^name$'`), which the first check guarantees exists. Alternative considered: choose the destination ourselves from the list of loaded sessions. Rejected: needs JSON parsing and reimplements kitty's history.

### `close_session`, not `close-tab`
`close_session` is kitty's own action for "close every window of this session" and is what removes the session from kitty's loaded set, which is what stops the watcher from re-saving it. Alternative considered: `kitten @ close-tab --match session:^name$` (used by rename today). Not chosen: closes tabs but was not verified to drop the session from the loaded set, and `close_session` was.

### Stronger prompt, no tab count
The prompt says the tabs "will be closed" and where the user lands, without a number, so no JSON parsing is needed. Alternative considered: count tabs with `serde_json`. Rejected as a new dependency for wording only.

### Refuse when there is nowhere to go
If every tab belongs to the session being deleted, closing them would quit kitty (and the quit hook would save sessions). The delete is refused instead. Alternative considered: allow it and let kitty quit. Rejected as a surprising side effect of "delete". This is an assumption made without the user's input; it is cheap to reverse.

## Risks / Trade-offs

- [Names with whitespace or regex characters break `--match` and `close_session <name>`] → build the match with a regex-escaping helper and pass it as a single argument; quote the name for `close_session`. Whitespace names remain a known limitation (TODO.md) and are verified only for plain names.
- [After deleting the current session there is nothing left to print to] → all checks and refusal messages happen before any destructive step; afterwards the picker only exits.
- [Refusal message visibility] → shown in the same prompt flow and waits for Enter, since the generic error display is a separate change.

## Migration Plan

None. No stored data changes. Rollback is reverting the commit.

## Open Questions

- Should deleting a non-current open session also warn when it holds a running foreground program? Can be decided later without changing the specs.
