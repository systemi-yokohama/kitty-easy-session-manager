# Spec Delta

## ADDED Requirements

### Requirement: Errors are visible
When an operation of the switcher fails, the switcher SHALL show the reason in a message that stays until the user presses Enter, and SHALL NOT close the picker silently or panic. Errors that prevent the picker from working at all (for example fzf cannot be started, or the sessions directory cannot be read) SHALL be shown the same way before the picker exits.

#### Scenario: fzf is missing
- **WHEN** the picker is started and fzf cannot be run
- **THEN** a message says fzf could not be started, waits for Enter, and the picker exits

#### Scenario: Sessions directory is unreadable
- **WHEN** the sessions directory exists but cannot be read
- **THEN** a message with the reason is shown, waits for Enter, and the picker exits

### Requirement: Missing sessions directory
If the sessions directory does not exist, the picker SHALL list only `[No Session]` and work normally, and SHALL create the directory when the first session file is created.

#### Scenario: First use
- **WHEN** `~/.config/kitty/sessions` does not exist and the user opens the picker
- **THEN** the picker lists only `[No Session]`, and creating a session named `blog` creates the directory and `blog.kitty-session`

## MODIFIED Requirements

### Requirement: Open a session
Pressing Enter on a session name SHALL switch kitty to that session, using the session file in the sessions directory, and close the picker. The switcher SHALL confirm from kitty's state that the user is in that session. If not, it SHALL show why and keep the picker open.

#### Scenario: Open a saved session
- **WHEN** the user presses Enter on `docker`
- **THEN** kitty switches to the `docker` session and the picker closes

#### Scenario: The switch does not happen
- **WHEN** the user presses Enter on `docker` and kitty does not end up in that session
- **THEN** the switcher shows that the session could not be opened, waits for Enter and keeps the picker open

### Requirement: Create a session
`Ctrl-n` SHALL prompt for a session name and, if the name is valid, non-empty and no session file with that name exists, create `<name>.kitty-session` containing one tab with one shell and switch kitty to it. After a successful switch the picker SHALL exit. An empty name SHALL create nothing silently; an invalid or existing name SHALL create nothing and overwrite nothing and SHALL say why. In all those cases the picker SHALL be shown again. If the session file is created but the switch does not happen, the switcher SHALL say so and keep the picker open.

#### Scenario: Create a new session
- **WHEN** the user presses `Ctrl-n` and enters `blog`
- **THEN** `blog.kitty-session` is created with one tab running the default shell, kitty switches to `blog`, and the picker exits, leaving no picker behind in the previous tab

#### Scenario: Name already taken
- **WHEN** the user presses `Ctrl-n` and enters the name of an existing session
- **THEN** the existing file is left unchanged, the switcher says the name is taken and the picker is shown again

#### Scenario: Empty name
- **WHEN** the user presses `Ctrl-n` and submits an empty name
- **THEN** nothing is created and the picker is shown again

#### Scenario: Created but not opened
- **WHEN** the file is created but kitty does not end up in the new session
- **THEN** the switcher says the session was created but could not be opened, waits for Enter and keeps the picker open
