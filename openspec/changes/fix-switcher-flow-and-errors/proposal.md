# Proposal

## Why

The picker misbehaves in basic ways (register items #4, #6, #8, #10 in the archived `baseline-specs`):

- After `Ctrl-n` creates a session and switches to it, the picker loops back to its list, so a stale picker overlay stays in the tab the user just left. It was reproduced in a live kitty and reappears when the user returns to that tab.
- Failures are printed to stderr and immediately hidden when the overlay closes or fzf redraws. `kitten @ action goto_session` also exits 0 when it silently did nothing, so the exit code cannot be trusted.
- `expect(...)` panics (missing `HOME`, missing or unreadable sessions directory, fzf failing to start) close the overlay with no explanation.
- `[+ New Session]` is dead code: the entry is never listed.

## What Changes

- After a session is created and switched to, the picker exits.
- Opening or creating a session verifies that the user really ended up in it; otherwise it shows the reason and keeps the picker open. Success is decided from kitty's state, not from the exit code.
- No more panics: a missing sessions directory is an empty list (created when the first session is created); other errors (unreadable directory, missing `HOME`, fzf not runnable) are shown in a message that waits for Enter.
- Remove the dead `[+ New Session]` code.

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `session-switcher`: "Open a session" and "Create a session" change (verify the switch, exit after create, report failures); a new "Errors are visible" requirement and a "Missing sessions directory" requirement are added.

## Impact

- `src/main.rs`, `src/sessions.rs`, `src/ui.rs`.
- README: the sessions directory no longer has to be created by hand.
- Depends on the message helper introduced in earlier changes; sequenced after `no-session-creates-tab`.
