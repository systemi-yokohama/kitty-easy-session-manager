# Spec Delta

## MODIFIED Requirements

### Requirement: Return to session-less tabs
Pressing Enter on `[No Session]` SHALL focus a tab that does not belong to any session and close the picker. If no such tab exists, the switcher SHALL first create one, a new tab running the default shell that belongs to no session and starts in the working directory of the tab the user is in, and focus it. It SHALL NOT create another one when a session-less tab already exists. If the tab cannot be created or focused, the switcher SHALL tell the user why and keep the picker open. `Ctrl-r` and `Ctrl-d` on `[No Session]` SHALL do nothing and the picker SHALL stay open.

_Verification: creating a session-less tab from an overlay inside a session (`kitten @ launch --type=tab`) and focusing it with `focus-tab --match session:^$` were observed in a kitty 0.48.2 test instance on 2026-09-22, including when zero session-less tabs existed. The full behavior is verified when this change is applied._

#### Scenario: Go back to the startup tab
- **WHEN** the user started kitty without a session, switched to `docker`, and presses Enter on `[No Session]`
- **THEN** kitty focuses a tab that belongs to no session and the picker closes

#### Scenario: The session-less tab was closed
- **WHEN** the user closed the startup tab, is in `docker`, and presses Enter on `[No Session]`
- **THEN** a new shell tab that belongs to no session is created in the current working directory, focused, and the picker closes

#### Scenario: A session-less tab already exists
- **WHEN** a session-less tab exists and the user presses Enter on `[No Session]` twice, from different sessions
- **THEN** both times the same session-less tab is focused and no additional tab is created

#### Scenario: Creation fails
- **WHEN** kitty refuses to create the tab
- **THEN** the switcher shows the reason, waits for Enter and keeps the picker open

#### Scenario: Rename is ignored on the pseudo entry
- **WHEN** the user presses `Ctrl-r` on `[No Session]`
- **THEN** no prompt appears, nothing changes and the picker is shown again
