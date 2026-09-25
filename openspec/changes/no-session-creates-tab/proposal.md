# Proposal

## Why

`[No Session]` takes the user back to a tab that belongs to no session, but such a tab exists only because kitty opens one at startup. Once the user closes it, `[No Session]` silently does nothing (verified live), and there is no way to get a session-less tab again: `cmd+t` inside a session creates a tab in that session (register item #5 in the archived `baseline-specs`).

`[No Session]` is not a session but the always-available "scratch place" outside all sessions, so it should always work. Discussed and agreed with the user: if no session-less tab exists, create one.

## What Changes

- Pressing Enter on `[No Session]` focuses a session-less tab if one exists; otherwise it first creates one (a new tab with the default shell that belongs to no session) and focuses it.
- If the tab cannot be created or focused, the user is told why instead of the picker closing silently.

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `session-switcher`: the "Return to session-less tabs" requirement gains the create-on-demand behavior and error reporting.

## Impact

- `src/sessions.rs` (`goto_no_session`), `src/main.rs` (call site).
- README Usage: the `[No Session]` line.
- Closes the remaining part of register item #5. The tab-bar part was already resolved by the `session:.` filter, and the session block already reads `no session` in a session-less tab.
