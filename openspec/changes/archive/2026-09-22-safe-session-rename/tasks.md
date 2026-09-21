# Tasks

Prerequisite: `safe-session-names` is applied (validation, escaping and quoting helpers).

## 1. Rename flow

- [x] 1.1 Validate the new name (rules from `safe-session-names`) and show the reason for refused or taken names
- [x] 1.2 Not-open session: rename the file only; keep the picker
- [x] 1.3 Open session: warning prompt (`[y/N]`, default No)
- [x] 1.4 On `y`: snapshot with an anchored, escaped match; rename the file; open the new session; verify it is open, else roll back; then `close_session <old>`
- [x] 1.5 Make `main.rs` exit the picker after an open-session rename
- [x] 1.6 Replace the `expect` on `fs::rename` with handled errors

## 2. Docs

- [x] 2.1 README Usage: describe the new `Ctrl-r` behavior and that a not-open session is not opened

## 3. Verify in a live kitty (throwaway instance; one batch to limit focus stealing)

- [x] 3.1 Rename a not-open session: file renamed, nothing opened, picker shown again
- [x] 3.2 Rename an open session: warning shown, snapshot in the new file, lands in the renamed session, old tabs gone, picker exited
- [x] 3.3 `test` and `test2` open, rename `test`: `test2` untouched, new file has only `test`'s tab
- [x] 3.4 Decline the warning: nothing changes
- [x] 3.5 Rename to an existing or invalid name: nothing changes, reason shown (existing name verified live; invalid name covered by unit tests and the create path)
- [x] 3.6 Simulated failure to open the renamed session: file renamed back, old session open
- [x] 3.7 Rename the session the user is in
- [x] 3.8 `openspec validate safe-session-rename --strict` passes
