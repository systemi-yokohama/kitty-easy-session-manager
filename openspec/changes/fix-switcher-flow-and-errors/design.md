# Design

## Context

See `proposal.md`. `main.rs` loops: it lists sessions, runs fzf, and for `Ctrl-n` calls `create_session` then `continue`. `goto_session` builds `kitten @ action goto_session '<path>'` and only prints a warning on a non-zero exit. In a live kitty (2026-09-21/22) `kitten @ action goto_session` with a bad argument exited 0 without doing anything, while verifying kitty's state after the call worked reliably (`ls --match-tab` on the session, used by rename and delete).

`sessions.rs` and `ui.rs` still contain `expect(...)` on `HOME`, `read_dir`, fzf spawn and fzf wait.

## Goals / Non-Goals

**Goals:**
- No stale picker, no silent failure, no panic in the switcher.

**Non-Goals:**
- Changes to `watcher.py` errors (notifications already report save failures).
- A general logging facility.
- Changing keys or the picker's look.

## Decisions

### Decide success from kitty's state
After `goto_session`, poll (up to about one second, in short steps) until `state:focused and session:^<name>$` matches. Exit codes are not used, because the action reports success when it did nothing. Alternative considered: trust the exit code. Rejected by the observation above. A short poll is needed because the action is applied by kitty asynchronously; the rename check worked without polling in tests, but the poll costs nothing when it succeeds immediately.

### Functions return results, one place shows errors
Helpers return `Result<_, String>` instead of printing or panicking; the caller (`main`) shows the message with the existing "message and wait for Enter" helper. Fatal start-up problems (no `HOME`, unreadable directory, fzf cannot start, fzf exits with an error code) use the same helper and then exit. Alternative considered: keep `eprintln!` and pause the overlay. Rejected: output ordering with fzf's inline rendering is unreliable.

### Missing sessions directory is empty, not an error
`NotFound` from `read_dir` gives an empty list, and `create_session` calls `create_dir_all` before writing the file. Other I/O errors are real errors and are shown. Alternative considered: treat all errors as empty. Rejected: it would hide permission problems.

### Exit the picker after a successful create
Same reason as delete and rename: the picker's overlay lives in the tab the user left. It exits only after the state check succeeded, so a failed create keeps the picker (and the user's context).

### Remove `[+ New Session]`
Never listed, its branches are unreachable. Remove the constant, the branches, and the name reservation for it (the pseudo entry `[No Session]` stays reserved).

## Risks / Trade-offs

- [The poll adds up to about a second of waiting on a real failure] → acceptable; success returns immediately.
- [fzf exit codes: 130 for Esc, 1 for no match, 2 for an error] → only 2 (and spawn failures) are treated as errors; 130 and 1 keep meaning "nothing selected".
- [A file is left behind when create succeeds but the switch fails] → intended and stated in the message: the session exists and can be opened from the picker.

## Migration Plan

None. Rollback is reverting the commit.

## Open Questions

- Should the poll timeout be configurable? Can be decided later without changing the specs.
