# Spec Delta

## ADDED Requirements

### Requirement: Session names
A session name SHALL be accepted when, after trimming surrounding whitespace, it is not empty, does not start with `.`, does not contain `/`, `\`, `'`, `"` or control characters, does not end with `.kitty-session`, and is not `[No Session]`. Spaces inside a name SHALL be allowed. When a name is refused, the switcher SHALL say why and create nothing. Every operation of the switcher (open, create, delete) SHALL work for any accepted name, including names with spaces.

_Verification: run against the implementation in a kitty 0.48.2 test instance on 2026-09-22: create, reopen and delete of `my project`; names with `.`, `+`, `[`, `(`; refused names (`../x`, `a/b`, `[No Session]`, `x.kitty-session`, `it's`, `.hid`) each showed a reason and created nothing. Trimming and the full rule set are covered by unit tests._

#### Scenario: Name with spaces
- **WHEN** the user creates a session named `my project`, opens it later from the picker, and deletes it
- **THEN** `my project.kitty-session` is created, opening switches to that session, and deleting removes the file and closes that session's tabs

#### Scenario: Name that would leave the sessions directory
- **WHEN** the user enters `../evil` or `a/b` as a new session name
- **THEN** the switcher explains that `/` is not allowed and nothing is created

#### Scenario: Name that collides with the pseudo entry or the file extension
- **WHEN** the user enters `[No Session]` or `notes.kitty-session`
- **THEN** the switcher refuses the name and nothing is created

#### Scenario: Surrounding whitespace
- **WHEN** the user enters `  blog  `
- **THEN** the session is created as `blog.kitty-session`
