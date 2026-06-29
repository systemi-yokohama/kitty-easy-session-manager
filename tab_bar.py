# ~/.config/kitty/tab_bar.py
"""
Custom kitty tab bar.

Layout:
  [ <session>] [tab1] [tab2] ...
   ↑ always inactive styling, never colored when tab1 is active

Root cause of the "session block gets active color" bug:
  TabBar.update() sets screen.cursor.bg/fg to active colors BEFORE calling
  draw_tab.  draw_tab_with_powerline reads screen.cursor.bg as tab_bg, so
  it ignores session_tab.is_active=False and uses whatever color is already
  on the cursor.

Fix: manually reset cursor colors to inactive before drawing the session
block, then restore the correct colors before drawing the real first tab.

Required kitty.conf:
  tab_bar_style       custom
  tab_powerline_style round   # or slanted
  tab_bar_min_tabs    1
"""

from kitty.fast_data_types import Screen, get_options, wcswidth
from kitty.tab_bar import DrawData, ExtraData, TabBarData, as_rgb, draw_tab_with_powerline

SESSION_ICON = ""  # Nerd Fonts kitty icon (U+F39B)


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
    if index == 1:
        session_name = (
            tab.active_session_name
            or tab.session_name
            or "no session"
        )
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
            draw_data, screen, session_tab,
            before, session_max, index, False, session_ed,
        )

        # ── Restore correct cursor colors for the actual first tab ────────────
        screen.cursor.bg = as_rgb(draw_data.tab_bg(tab))
        screen.cursor.fg = as_rgb(draw_data.tab_fg(tab))
        opts = get_options()
        screen.cursor.bold, screen.cursor.italic = (
            opts.active_tab_font_style if tab.is_active else opts.inactive_tab_font_style
        )

        actual_ed = ExtraData()
        actual_ed.prev_tab = session_tab
        actual_ed.next_tab = extra_data.next_tab
        actual_ed.for_layout = extra_data.for_layout

        remaining = max(1, before + max_tab_length - session_end)
        return draw_tab_with_powerline(
            draw_data, screen, tab,
            session_end, remaining, index, is_last, actual_ed,
        )

    return draw_tab_with_powerline(
        draw_data, screen, tab,
        before, max_tab_length, index, is_last, extra_data,
    )
