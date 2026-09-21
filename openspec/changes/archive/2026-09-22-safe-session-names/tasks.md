# Tasks

## 1. Switcher

- [x] 1.1 Add name validation with a human-readable reason for each rejected rule
- [x] 1.2 Use it in `create_session` and show the reason (waiting for Enter) instead of silently returning
- [x] 1.3 Make `regex_escape` map whitespace to `\s`; keep a single `session_match` helper for every match expression
- [x] 1.4 Add a helper that single-quotes `kitten @ action` arguments; use it for `goto_session`, `close_session` and every other action that carries a name or path
- [x] 1.5 Remove the `expect` panics touched by these code paths

## 2. Watcher

- [x] 2.1 Build the `--match` expression in `save_all_sessions` with the same escaping (space to `\s`), keeping it anchored
- [x] 2.2 Confirm the destination filename with a space is passed as one argument

## 3. Docs

- [x] 3.1 README Usage: state the name rules and that spaces are allowed
- [x] 3.2 `TODO.md`: remove the whitespace item

## 4. Verify in a live kitty (throwaway instance; minimise GUI launches, one batch)

- [x] 4.1 Create, open and delete `my project`; tabs closed on delete, file removed
- [x] 4.2 `cmd+s` with `my project`, `test` and `test2` loaded: three correct files, `test` has only its own tab
- [x] 4.3 Names with `.`, `+`, `[` and `(` behave the same
- [x] 4.4 Refused names (`../x`, `a/b`, `[No Session]`, `x.kitty-session`, empty, a quote) create nothing and show a reason
- [x] 4.5 `openspec validate safe-session-names --strict` passes
