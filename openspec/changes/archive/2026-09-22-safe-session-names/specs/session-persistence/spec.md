# Spec Delta

## ADDED Requirements

### Requirement: Sessions with spaces in the name are saved
Saving SHALL write each loaded session to `<name>.kitty-session` using exactly that session's tabs, also when the name contains spaces or characters that are special in regular expressions.

_Verification: verified in a kitty 0.48.2 test instance on 2026-09-22 with `my project`, `test` and `test2` loaded: each file held exactly its own session's tab._

#### Scenario: Name with a space
- **WHEN** the session `my project` is loaded and the user presses `cmd+s`
- **THEN** `my project.kitty-session` is rewritten with the tabs of `my project` only

#### Scenario: Similar names
- **WHEN** sessions `test` and `test2` are loaded and the user presses `cmd+s`
- **THEN** `test.kitty-session` contains only the tabs of `test`, and `test2.kitty-session` only those of `test2`
