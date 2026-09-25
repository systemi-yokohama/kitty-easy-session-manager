# Tasks

Prerequisite: `no-session-creates-tab` is applied.

## 1. Results instead of panics

- [ ] 1.1 `session_dir` returns a `Result` (missing `HOME`); `list_sessions` returns `Ok(empty)` for a missing directory and `Err(reason)` otherwise
- [ ] 1.2 `run_fzf` returns a `Result`: spawn failure, wait failure, and exit code 2 are errors with a readable message
- [ ] 1.3 `main` shows start-up errors with the shared message helper and exits
- [ ] 1.4 `create_session` calls `create_dir_all` before writing the file and handles I/O errors

## 2. Verified switching

- [ ] 2.1 Add a bounded poll that waits for `state:focused and session:^<name>$`
- [ ] 2.2 `goto_session` returns `Result` from that check; `main` shows the failure and keeps the picker
- [ ] 2.3 `create_session` reports "created but not opened" and returns whether the picker should exit
- [ ] 2.4 `main` exits the picker after a successful create instead of `continue`

## 3. Cleanup and docs

- [ ] 3.1 Remove `CREATE_NEW`, its branches, its name reservation and its unit-test entry
- [ ] 3.2 README: the sessions directory is created on first use

## 4. Verify in a live kitty (throwaway instance; same launch as `no-session-creates-tab`)

- [ ] 4.1 `Ctrl-n` create: the picker exits and no picker overlay remains in the previous tab
- [ ] 4.2 Open a session that cannot be opened (a `kitten` wrapper that drops `goto_session`): message shown, picker stays
- [ ] 4.3 Start with no sessions directory: only `[No Session]`, and creating a session creates the directory
- [ ] 4.4 Sessions directory unreadable (permissions removed): message shown, no panic
- [ ] 4.5 fzf missing from `PATH`: message shown, no panic
- [ ] 4.6 `cargo test` and `openspec validate fix-switcher-flow-and-errors --strict` pass
