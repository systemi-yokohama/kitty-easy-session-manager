# Tasks

## 1. Implementation

- [ ] 1.1 In `goto_no_session`: check for an existing session-less tab, create one with `launch --type=tab --cwd=current` if none, then focus it
- [ ] 1.2 Return the outcome so `main.rs` closes the picker on success and keeps it open (with the message) on failure
- [ ] 1.3 Show failures with the shared message helper instead of `eprintln!`

## 2. Docs

- [ ] 2.1 README Usage: `[No Session]` goes to a session-less tab and creates one if none exists

## 3. Verify in a live kitty (throwaway instance; bundle with the next change's checks to limit focus stealing)

- [ ] 3.1 Zero session-less tabs: Enter on `[No Session]` creates one, it is focused, the picker exits, `ls --match-tab session:^$` succeeds
- [ ] 3.2 The new tab's working directory is the directory of the tab the user came from
- [ ] 3.3 A session-less tab exists: no extra tab is created, from two different sessions
- [ ] 3.4 Failure is reported (simulate with a `kitten` wrapper that fails `launch`) and the picker stays open
- [ ] 3.5 `openspec validate no-session-creates-tab --strict` passes
