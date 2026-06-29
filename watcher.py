import re
import subprocess
from pathlib import Path
from typing import Any

from kittens.tui.handler import result_handler
from kitty.boss import Boss
from kitty.window import Window

sessions_dir = Path.home() / "dotfiles" / "kitty" / "sessions"


def session_file_name(session_name: str) -> str:
    path = Path(session_name)
    if path.suffix == ".kitty-session":
        return session_name
    return f"{session_name}.kitty-session"


def applescript_string(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def notify(message: str) -> None:
    script = (
        f"display notification {applescript_string(message)} "
        f"with title {applescript_string('Kitty')}"
    )
    subprocess.Popen(["osascript", "-e", script])


def save_all_sessions(boss: Boss) -> int:
    saved = 0

    for session_name in sorted(set(boss.all_loaded_session_names)):
        if not session_name:
            continue

        boss.save_as_session(
            "--base-dir",
            str(sessions_dir),
            f"--match=session:^{re.escape(session_name)}$",
            "--save-only",
            "--use-foreground-process",
            session_file_name(session_name),
        )
        saved += 1

    return saved


def save_all_sessions_with_notification(boss: Boss) -> None:
    saved = save_all_sessions(boss)
    notify(f"Saved {saved} kitty session{'s' if saved != 1 else ''}")


def on_quit(boss: Boss, window: Window, data: dict[str, Any]) -> None:
    # if before window close confirmation, return
    # https://sw.kovidgoyal.net/kitty/launch/#watching-launched-windows
    if data.get("confirmed") is False:
        return

    try:
        save_all_sessions_with_notification(boss)
    except Exception as err:
        data["aborted"] = True
        notify(f"Failed to save kitty sessions: {err}")


def main(args: list[str]) -> str:
    return ""


@result_handler(no_ui=True)
def handle_result(
    args: list[str], answer: str, target_window_id: int, boss: Boss
) -> None:
    try:
        save_all_sessions_with_notification(boss)
    except Exception as err:
        notify(f"Failed to save kitty sessions: {err}")
