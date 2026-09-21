# session-tab-bar Specification

## Purpose
The custom tab bar shows which session the user is in and only the tabs of that session, with short, readable tab titles.

## Requirements

### Requirement: Tabs are filtered by session
With `tab_bar_filter session:.`, the tab bar SHALL show only the tabs that belong to the same session as the active tab. When the active tab belongs to no session, the tab bar SHALL show only the tabs that belong to no session. The recommended configuration SHALL NOT use `session:~`, because it also shows the tabs of the most recently loaded session.

_Verification: verified by screenshot in a kitty 0.48.2 test instance (2026-09-21) with `session:.`: inside `config` only the `config` tabs were shown; after `[No Session]` only the session-less tab was shown. The problem with `session:~` was observed by the user in their own kitty (recorded in their `kitty.conf` comment); not re-run._

#### Scenario: Switching sessions
- **WHEN** the user has tabs in `config` and `docker` and switches to `docker`
- **THEN** the tab bar shows only the `docker` tabs

#### Scenario: Session-less tabs are hidden inside a session
- **WHEN** the user started with a session-less tab and switches to `docker`
- **THEN** the session-less tab is not shown in the tab bar

#### Scenario: Back to session-less tabs
- **WHEN** the user picks `[No Session]` in the switcher
- **THEN** the tab bar shows only the session-less tabs and no tab of `docker`

### Requirement: Session block
The first item of the tab bar SHALL be a block with a session icon and a session name. The name SHALL be the active session name, or else the session of the first tab, or else `no session`.

#### Scenario: Inside a session
- **WHEN** the active session is `config`
- **THEN** the block reads `config` and is followed by the session's tabs

#### Scenario: Session-less tab
- **WHEN** the active tab belongs to no session, including right after picking `[No Session]` from a session
- **THEN** the block reads `no session`

### Requirement: Tab titles
Each tab title SHALL be, in order of preference: the title set explicitly for the tab; the name of the program running in the foreground of the tab's active window; `new tab` when only the shell is running. Titles wider than 20 cells SHALL be truncated with a trailing `…`.

#### Scenario: Explicit title wins
- **WHEN** a tab was given the title `logs` and runs `htop`
- **THEN** the tab shows `logs`

#### Scenario: Foreground program
- **WHEN** a tab without an explicit title runs `nvim`
- **THEN** the tab shows `nvim`

#### Scenario: Idle shell
- **WHEN** a tab without an explicit title only runs `zsh`
- **THEN** the tab shows `new tab`

#### Scenario: Long title
- **WHEN** the resolved title is wider than 20 cells
- **THEN** it is cut and ends with `…`
