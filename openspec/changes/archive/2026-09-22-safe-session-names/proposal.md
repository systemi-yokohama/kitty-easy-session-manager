# Proposal

## Why

Session names flow into kitty commands and match expressions without validation or quoting (register items #2 and #7 in the archived `baseline-specs`). Today the prompt accepts a name with spaces, but then `--match` breaks: delete leaves such a session's tabs open, and `watcher.py` cannot save it. Names containing `/` or `..` can write outside the sessions directory, and a name ending in `.kitty-session` produces different filenames in the Rust switcher and the Python watcher. Rename (next change) needs safe names first. `TODO.md` already asks for whitespace in session names.

## What Changes

- Define what a valid session name is, and check it when a session is created (and, in the next change, renamed). Invalid names are refused with a reason.
- Allow spaces in names, and make every place that hands a session name to kitty (switcher and watcher) escape and quote it correctly, so open, save, delete and later rename all work for such names.
- Match sessions by an anchored, escaped expression everywhere.

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `session-switcher`: adds a "Session names" requirement (valid names, spaces allowed, invalid names refused) and whitespace-safe open and delete.
- `session-persistence`: adds a requirement that sessions with spaces in their name are saved to the right file.

## Impact

- `src/sessions.rs` (name validation, quoting helpers, all `kitten @` calls that carry a name), `src/main.rs` (create flow shows the refusal).
- `watcher.py` (`save_all_sessions` match expression).
- README Usage: name rules.
- `TODO.md`: the whitespace item is done.
