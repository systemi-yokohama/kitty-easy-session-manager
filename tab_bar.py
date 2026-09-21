import os

from kitty.fast_data_types import Screen, get_boss, get_options, wcswidth
from kitty.tab_bar import (
    DrawData,
    ExtraData,
    TabBarData,
    as_rgb,
    draw_tab_with_powerline,
)

SESSION_ICON = ""

# Tab titles: "new tab" while only the shell is running, the app name otherwise.
DEFAULT_TITLE = "new tab"
MAX_TITLE_CELLS = 20  # max display width of a tab title
ELLIPSIS = "…"

_SHELLS = {"sh", "bash", "zsh", "fish", "dash", "ksh", "csh", "tcsh", "nu", "elvish"}
_SHELLS.add(os.path.basename(os.environ.get("SHELL", "")))


def _truncate(text: str, max_cells: int) -> str:
    if wcswidth(text) <= max_cells:
        return text
    out = ""
    for ch in text:
        if wcswidth(out + ch + ELLIPSIS) > max_cells:
            break
        out += ch
    return out + ELLIPSIS


def _foreground_app(tab_id: int) -> str:
    """Name of the program running in the tab's active window ("" if it is the shell)."""
    try:
        child = get_boss().tab_for_id(tab_id).active_window.child
        # Helpers an app spawns (e.g. claude -> `caffeinate`) share its foreground
        # process group, so pick the group leader (pid == pgid) rather than any member.
        procs = child.foreground_processes
        try:
            pgid = os.tcgetpgrp(child.child_fd)
        except OSError:
            pgid = None
        # Fall back to the oldest process (children get higher pids than their parent).
        proc = next((p for p in procs if p["pid"] == pgid), None) or min(
            procs, key=lambda p: p["pid"]
        )
        cmdline = proc["cmdline"]
        # Login shells show up as "-zsh".
        name = os.path.basename(cmdline[0]).lstrip("-") if cmdline else ""
    except Exception:
        return ""
    return "" if name in _SHELLS else name


def display_title(tab: TabBarData) -> str:
    # A title set explicitly (set_tab_title, `new_tab <name>` in a session) wins.
    try:
        explicit = get_boss().tab_for_id(tab.tab_id).name
    except Exception:
        explicit = ""
    title = explicit or _foreground_app(tab.tab_id) or DEFAULT_TITLE
    return _truncate(title, MAX_TITLE_CELLS)


def draw_tab(
    draw_data: DrawData,
    screen: Screen,
    tab: TabBarData,
    before: int,
    max_tab_length: int,
    index: int,
    is_last: bool,
    extra_data: ExtraData,
) -> int:
    tab = tab._replace(title=display_title(tab))

    if index == 1:
        session_name = tab.active_session_name or tab.session_name or "no session"
        session_title = f"{SESSION_ICON} {session_name}"

        session_tab = tab._replace(
            title=session_title,
            is_active=False,
            tab_id=-1,
        )

        # ── Force inactive cursor colors for the session block ────────────────
        # TabBar.update() already set screen.cursor.bg/fg to active colors
        # before calling us (when the first tab is focused).
        # draw_tab_with_powerline reads screen.cursor.bg as its tab_bg, so we
        # must override it here — session_tab.is_active=False alone is not enough.
        # screen.cursor.bg = as_rgb(draw_data.tab_bg(session_tab))
        # screen.cursor.fg = as_rgb(draw_data.tab_fg(session_tab))
        # screen.cursor.bold = screen.cursor.italic = False
        screen.cursor.bg = as_rgb(draw_data.tab_fg(session_tab))
        screen.cursor.fg = as_rgb(draw_data.tab_bg(session_tab))
        screen.cursor.bold = screen.cursor.italic = False

        session_ed = ExtraData()
        session_ed.prev_tab = None
        session_ed.next_tab = tab
        session_ed.for_layout = extra_data.for_layout

        session_max = wcswidth(session_title) + 4
        session_end = draw_tab_with_powerline(
            draw_data,
            screen,
            session_tab,
            before,
            session_max,
            index,
            False,
            session_ed,
        )

        # ── Restore correct cursor colors for the actual first tab ────────────
        screen.cursor.bg = as_rgb(draw_data.tab_bg(tab))
        screen.cursor.fg = as_rgb(draw_data.tab_fg(tab))
        opts = get_options()
        screen.cursor.bold, screen.cursor.italic = (
            opts.active_tab_font_style
            if tab.is_active
            else opts.inactive_tab_font_style
        )

        actual_ed = ExtraData()
        actual_ed.prev_tab = session_tab
        actual_ed.next_tab = extra_data.next_tab
        actual_ed.for_layout = extra_data.for_layout

        remaining = max(1, before + max_tab_length - session_end)
        return draw_tab_with_powerline(
            draw_data,
            screen,
            tab,
            session_end,
            remaining,
            index,
            is_last,
            actual_ed,
        )

    return draw_tab_with_powerline(
        draw_data,
        screen,
        tab,
        before,
        max_tab_length,
        index,
        is_last,
        extra_data,
    )
