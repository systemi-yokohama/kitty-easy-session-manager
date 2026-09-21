# Proposal

## Why

ESM has no written contract. The README and the `kitty.conf` comments describe behavior the code does not have (for example, the README still says the tab bar also shows session-less tabs and recommends `tab_bar_filter session:~`, which the user has since replaced with `session:.` after seeing it pull in another session's tabs), and several basic behaviors are wrong or undefined. Before fixing those, we need one source of truth for what ESM does today, so each later fix is a reviewable delta against it instead of another undocumented tweak.

## What Changes

- Capture the current, sound behavior of the switcher, the watcher and the tab bar as baseline specs.
- Specs only state behavior we intend to keep. Known defects (delete leaving the session alive and re-saved, unanchored rename matching, picker surviving `ctrl-n`, silent errors, panics) are deliberately **not** written down as requirements; they are listed in `design.md` and get their own changes that ADD or MODIFY requirements.
- Fix documentation that contradicts the specs: the README's tab-filter claim and recommended `session:~`, and the `[+ New Session]` / `Ctrl-n` usage. The user's `kitty.conf` comment was already corrected by them.
- Document the `[No Session]` picker entry, which is already implemented.

No code behavior changes.

## Capabilities

### New Capabilities
- `session-switcher`: the `kitty-esm-switcher` fzf picker: listing saved sessions, opening, creating, renaming, deleting, and the `[No Session]` entry.
- `session-persistence`: `watcher.py`: saving loaded sessions to `~/.config/kitty/sessions` on demand (`cmd+s`) and on quit.
- `session-tab-bar`: `tab_bar.py` and the `tab_bar_filter`: which tabs are shown, the session block, and tab titles.

### Modified Capabilities
<!-- None: there are no existing specs. -->

## Impact

- New files under `openspec/specs/` (after archive).
- `README.md`: Usage and tab-filter wording corrected.
- `~/.config/kitty/kitty.conf` (outside this repo): no change needed; already uses `session:.`.
- No changes to `src/`, `watcher.py` or `tab_bar.py`.
