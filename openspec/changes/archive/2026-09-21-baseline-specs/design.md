# Design

## Context

See `proposal.md` for motivation. ESM has three parts: the Rust picker (`src/`), `watcher.py` and `tab_bar.py`, wired into the user's `kitty.conf`. All specs in this change were written by reading that code and the kitty 0.48 documentation. Behavior was then checked in a throwaway kitty 0.48.2 instance (own `HOME`, remote-control socket, screenshots) on 2026-09-21; each spec carries a `Verification` note saying what was and was not observed.

## Goals / Non-Goals

**Goals:**
- One spec per component that a reader can trust to match the code today.
- A visible register of known defects, each assigned to a follow-up change, so nothing found in the audit is lost.

**Non-Goals:**
- Fixing any defect here. This change edits docs only.
- Specifying behavior we already know is wrong.

## Decisions

### Specs describe sound behavior only; defects live in this register
Writing a defect into a baseline spec would make the spec "true" but wrong, and every fix would then have to remove a requirement instead of adding one. Instead, requirements are silent where behavior is a known defect, and the follow-up change ADDs or MODIFIES the requirement. Alternative considered: spec the current behavior verbatim and mark bugs inline. Rejected because the specs would be the thing people copy from.

### Known-gap register
| # | Gap (observed in code) | Follow-up change |
|---|---|---|
| 1 | **Verified.** Deleting a session only removes the file; its tabs stay open and the next save recreated the deleted file (reproduced with a throwaway session). Agreed direction: after delete, go to the previous session, or `[No Session]` if that is hard; close the deleted session's tabs. | `fix-session-delete` |
| 2 | Rename matches the old session with an unanchored, unescaped regex (`session:test` also matches `test2`), for both saving and closing tabs. | `safe-session-rename` |
| 3 | Rename re-launches the session from its file, so running processes restart and scrollback is lost. Agreed direction: warn, then close and re-open the renamed session (or refuse for an open session). | `safe-session-rename` |
| 4 | `Ctrl-n` creates and switches, then loops back to the picker, leaving a picker overlay in the previous session's tab. | `fix-switcher-flow-and-errors` |
| 5 | Filter part resolved (`session:~` pulled in another session's tabs; `kitty.conf` now uses `session:.`). Session block is fine: it reads `no session` in a session-less tab (verified). **Verified, still open:** `[No Session]` silently does nothing when no session-less tab exists, and session-less tabs cannot be created after startup (a new tab opened inside a session joins that session), so closing them makes the entry useless. | `no-session-tab-bar` |
| 6 | Errors (`goto_session` failure, name taken) are printed to stderr and immediately hidden by the overlay closing or fzf redrawing. | `fix-switcher-flow-and-errors` |
| 7 | Names are not validated: whitespace breaks `--match`, `/` and `..` escape the sessions directory, a `.kitty-session` suffix is handled differently in Rust and Python. | `safe-session-rename` (and create) |
| 8 | `expect(...)` panics on missing sessions directory or filesystem errors, closing the overlay with no explanation. | `fix-switcher-flow-and-errors` |
| 9 | `startup_session` in `kitty.conf` points to a file that does not exist. **Verified harmless:** kitty logs `Failed to read from session file, ignoring` and starts normally. Keep as is, or remove the line to silence the log. | `baseline-specs` (closed) |
| 10 | Dead `CREATE_NEW` (`[+ New Session]`) branches in the picker. | `fix-switcher-flow-and-errors` |

### Documentation fixes belong to this change
The README contradicts the specs in three places (see tasks); the `kitty.conf` comment was already corrected by the user. They are fixed here so the specs become the reference immediately.

## Risks / Trade-offs

- [Specs rest on reading, not running] → Each kitty-dependent requirement carries a `Verification` note and tasks include a manual check in a live kitty before archive.
- [Gap register drifts from reality] → Each follow-up change removes its row when archived.

## Open Questions

- Should the register stay in this design document after archive, or move to a tracked list (for example `TODO.md`)? It can be decided at archive time without changing the specs.
