# Design

## Context

See `proposal.md`. Names reach kitty in three ways, each with its own parsing (observed in a kitty 0.48.2 test instance, 2026-09-21):

1. **Match expressions** (`kitten @ ls --match-tab`, `focus-tab --match`): tokenized on spaces. `session:^my proj$` fails; `session:"^my proj$"` and `session:^my\sproj$` work. `session:test` also matches `test2` (unanchored), and a snapshot taken with it pulled `test2`'s tab into `test`'s file.
2. **`kitten @ action ...` arguments**: joined and re-split shell-style. An unquoted path with a space is silently ignored; a single-quoted path works. A `\s` in a `--match=` argument survived only as `\\s`.
3. **`boss.save_as_session(*args)` in the watcher**: an argv list, no shell splitting; the match expression still has to be a single space-free token.

The switcher runs as a kitty overlay, and `src/sessions.rs` builds these strings in several places, the watcher in one.

## Goals / Non-Goals

**Goals:**
- One escaping rule for match expressions and one quoting rule for `action` arguments, used everywhere a name is passed to kitty.
- A name policy that keeps filenames inside the sessions directory and identical between Rust and Python.

**Non-Goals:**
- Rename behavior (next change, `safe-session-rename`).
- Non-ASCII or very long names beyond what kitty and the filesystem already accept.

## Decisions

### Escape whitespace as `\s`, other regex characters with a backslash
`\s` is the only space representation that works in match expressions without quotes, so the expression stays one token. Because names may not contain control characters, `\s` matches only a space. Alternative considered: `session:"^…$"` with double quotes. Rejected: quotes then need a second layer of escaping when the expression is placed in an `action` argument.

### Quote every `action` argument with single quotes
For `kitten @ action`, each argument that can contain a name (paths, `--match=…`, session name for `close_session`) is wrapped in single quotes, with `'` excluded by the name policy, so no further escaping is needed. Alternative considered: escape spaces with backslashes. Rejected: harder to reason about after kitty re-splits the string. Verified in a live kitty: single-quoting preserves the `\s` inside `--match=` (the snapshot of a session with a space in its name selected exactly that session).

### Name policy: reject instead of sanitize
Refuse and explain rather than silently rewriting a name (the user typed it, and it becomes a filename). The rejected set is the minimum needed for the three parsing layers above plus path safety: `/`, `\`, quotes, control characters, a leading `.`, the `.kitty-session` suffix (avoids `x.kitty-session.kitty-session` versus `x.kitty-session`), and the reserved `[No Session]`. Alternative considered: also reject spaces (simplest). Rejected: the user asked for whitespace names in `TODO.md`, and they already work in the prompt today.

### Validate at the point of entry only
Validation runs when a name is typed. Existing session files with unusual names (created by hand) are still listed and handled by the same escaping/quoting, but are not renamed by this change.

## Risks / Trade-offs

- [A helper for regex escaping and quoting is subtly wrong for some character] → the name policy shrinks the alphabet; tasks verify names with spaces, dots, brackets, `+`, and a `test`/`test2` pair in a live kitty.
- [Two implementations (Rust and Python) can drift] → both are covered by the same scenarios in the specs and by live checks; the Python side changes in one line.
- [Existing hand-made files with quotes or slashes in the name] → still listed; operations may fail with a visible message instead of silently misbehaving. Not migrated.

## Open Questions

- Maximum name length: left to the filesystem. Can be decided later without changing the specs.
