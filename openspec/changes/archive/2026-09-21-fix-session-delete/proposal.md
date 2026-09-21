# Proposal

## Why

Deleting a session only removes its file. The session's tabs stay open, so the user can keep working in a session that no longer exists, and the next save (`cmd+s` or quit) writes the file back. This was reproduced in a live kitty (see the archived `baseline-specs`, register item #1). Deleting must actually end the session.

## What Changes

- Deleting a session closes all of its open tabs.
- Deleting the session the user is currently in first moves them to the previously active session, or to a session-less tab (`[No Session]`) when there is no previous session, and only then closes the tabs.
- If there is nowhere to move to, the delete is refused and nothing changes.
- The confirmation prompt says that open tabs will be closed, and for the current session says where the user will land. The wording is stronger for the current session, as agreed.
- A deleted session's file does not come back on later saves (a consequence of closing its tabs; nothing changes in the watcher).

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `session-switcher`: the "Delete a session" requirement now covers open tabs, moving away from the current session, the refusal case and the stronger confirmation prompt.

## Impact

- `src/sessions.rs` (`delete_session`) and `src/main.rs` (flow after delete).
- README Usage: `Ctrl-d` description.
- No change to `watcher.py`: closing a session's tabs removes it from the loaded sessions, so saves no longer recreate its file (verified in a kitty 0.48.2 test instance).
