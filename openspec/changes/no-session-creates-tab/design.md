# Design

## Context

See `proposal.md`. `goto_no_session` (`src/sessions.rs`) runs `kitten @ focus-tab --match session:^$` and prints to stderr on failure, which the closing overlay hides.

Observed in a kitty 0.48.2 test instance (2026-09-22): from an overlay running inside session `test`, `kitten @ launch --type=tab` created a plain zsh tab whose window has an empty session name, i.e. a session-less tab, when no session-less tab existed. `focus-tab --match session:^$` then focused it, and `ls --match-tab session:^$` exits 1 again after it is closed. By contrast the `new_tab_with_cwd` action inside a session creates a tab in that session, so the remote-control `launch` is what makes a session-less tab. The new tab's working directory was the directory of the tab it was launched from.

## Goals / Non-Goals

**Goals:**
- `[No Session]` always works.

**Non-Goals:**
- A separate key or menu entry to create additional session-less tabs.
- Saving session-less tabs (they stay unsaved by design).
- Changing the tab bar; the `session:.` filter and the `no session` block already handle session-less tabs.

## Decisions

### Check, create if missing, then focus
1. `ls --match-tab session:^$`; exit 0 means one exists.
2. If none: `launch --type=tab --cwd=current` (kitty focuses the new tab).
3. `focus-tab --match session:^$` (no-op after a fresh launch, needed when it already existed).

Checking first avoids piling up session-less tabs on repeated use. Alternative considered: always create a fresh tab. Rejected: the entry means "go back to the place outside sessions", not "open another".

### Reuse the exact match already in use
The `session:^$` match is the one kitty documents for "not created in a session" and is already what the entry uses; no new escaping is needed.

### Report failure in the picker
On failure use the same prompt-and-wait message as the other operations, and keep the picker open. Alternative considered: exit anyway. Rejected: it hides the failure, which is what this change removes for this path.

## Risks / Trade-offs

- [`--cwd=current` may resolve to the overlay's cwd rather than the previous tab's] → they matched in the experiment; the apply-time check confirms it.
- [The created tab is unsaved scratch space and is lost when closed] → by design and unchanged; documented in the README.

## Migration Plan

None. Rollback is reverting the commit.

## Open Questions

- Should the new tab start in the home directory instead? A cosmetic default; can change without touching the specs' behavior contract.
