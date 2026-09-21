# Tasks

## 1. Kitty queries

- [x] 1.1 Add a helper to run `kitten @ ls --match-tab <expr>` and return whether it matched (exit code), with a regex-escaping helper for session names
- [x] 1.2 Add helpers: `session_is_open(name)`, `in_session(name)`, `other_tabs_exist(name)`

## 2. Delete flow

- [x] 2.1 Reorder `delete_session`: refuse when the user is in the session and no other tab exists (with a message that waits for Enter)
- [x] 2.2 Build the confirmation prompt: name the session; say tabs will be closed when it is open; say where the user lands when it is the current session
- [x] 2.3 On `y`: remove the file, move away if current (`goto_session -1`, then session-less tab, then any other tab), then `close_session <name>` last
- [x] 2.4 Make `main.rs` exit the picker after deleting the current session (its overlay is not closed by `close_session`) and keep re-showing it otherwise
- [x] 2.5 Replace the `expect` on `remove_file` with handled errors

## 3. Docs

- [x] 3.1 README Usage: describe the new `Ctrl-d` behavior

## 4. Verify in a live kitty (throwaway instance, own HOME and socket)

- [x] 4.1 Delete a session that is not open: file removed, nothing else changes
- [x] 4.2 Delete an open session while in another: tabs closed, user stays, picker lists it no more
- [x] 4.3 Delete the current session with history: lands in the previous session; save does not recreate the file
- [x] 4.4 Delete the current session without history: lands in the session-less tab
- [x] 4.5 Delete when nothing else exists: refused, file and tabs unchanged
- [x] 4.6 Decline: nothing changes
- [x] 4.7 `openspec validate fix-session-delete --strict` passes
