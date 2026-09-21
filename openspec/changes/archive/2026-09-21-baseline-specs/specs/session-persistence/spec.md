# Spec Delta

## Purpose

The watcher saves the state of loaded kitty sessions to session files so they can be restored later, either on demand or when kitty quits.

## ADDED Requirements

### Requirement: Save on demand
Triggering the save key (`cmd+s` in the recommended config) SHALL save every loaded session that has a name to `~/.config/kitty/sessions/<name>.kitty-session`, recording working directories and foreground processes, and SHALL show a notification `Saved N kitty session(s)` where N is the number of sessions saved. Despite the action's name (`save-active-session-with-notification`), all loaded sessions are saved, not only the active one.

_Verification: verified in a kitty 0.48.2 test instance (2026-09-21) by invoking the save action: both loaded sessions' files were rewritten. The notification text and count were not observable and remain unverified._

#### Scenario: Two sessions loaded
- **WHEN** the `config` and `docker` sessions are loaded and the user presses `cmd+s`
- **THEN** `config.kitty-session` and `docker.kitty-session` are rewritten and the notification reads `Saved 2 kitty sessions`

#### Scenario: Only session-less tabs
- **WHEN** no named session is loaded and the user presses `cmd+s`
- **THEN** no session file is written and the notification reads `Saved 0 kitty sessions`

### Requirement: Save on quit
When kitty quits and the quit has been confirmed, the watcher SHALL save all loaded named sessions the same way as the save key. If saving fails, the quit SHALL be aborted and a notification `Failed to save kitty sessions: <reason>` SHALL be shown. Before the quit is confirmed, nothing SHALL be saved.

#### Scenario: Quit with loaded sessions
- **WHEN** the user quits kitty with `docker` loaded and confirms the quit
- **THEN** `docker.kitty-session` is rewritten and kitty quits

#### Scenario: Quit not yet confirmed
- **WHEN** kitty shows its quit confirmation and the user has not answered yet
- **THEN** no session file is written

### Requirement: Session-less tabs are not saved
Tabs that belong to no session SHALL NOT be written to any session file.

#### Scenario: Startup tabs on quit
- **WHEN** the user quits kitty with only the tabs kitty opened at startup
- **THEN** no session file is created for them

### Requirement: Notifications are macOS-only
Save notifications SHALL be delivered through macOS notifications. On other platforms the save itself SHALL still happen, but no notification is guaranteed.

#### Scenario: Save on macOS
- **WHEN** a save completes on macOS
- **THEN** a notification titled `Kitty` is shown
