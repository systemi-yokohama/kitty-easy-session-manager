# Tasks

## 1. Verify kitty-dependent requirements in a live kitty

- [x] 1.1 Verify `[No Session]` focuses a session-less tab and that the tab bar then shows only session-less tabs with `tab_bar_filter session:.` (session-switcher, session-tab-bar)
- [x] 1.2 Verify that with `tab_bar_filter session:.` session-less tabs are hidden inside a session, only session-less tabs are shown after `[No Session]`, and record what the session block shows in a session-less tab (design register #5)
- [x] 1.3 Verify `cmd+s` saves every loaded named session (files verified; notification text/count not observable in the test instance)
- [x] 1.4 Verify what kitty does when `startup_session` points to a missing file, then record the result in the design register (#9)
- [x] 1.5 Update the `Verification` notes in the specs from "not yet verified" to the observed result

## 2. Fix documentation that contradicts the specs

- [x] 2.1 README: use `tab_bar_filter session:.` (snippet and uninstall list) with a comment matching the specs
- [x] 2.2 README Usage: remove `[+ New Session]` from the Enter description, document `Ctrl-n`, and document `[No Session]`
- [x] 2.3 README Watcher: state that the save key and quit save all loaded named sessions, and that session-less tabs are not saved
- [x] 2.4 `~/.config/kitty/kitty.conf`: comment already matches the specs (user updated it to `session:.`); no edit needed

## 3. Validate

- [x] 3.1 `openspec validate baseline-specs --strict` passes
- [x] 3.2 Re-read the README and `kitty.conf` against the three specs; no remaining contradictions
