from pathlib import Path

def sorted_fd(fds: list[Path]) -> list[Path]:
    return sorted(
        [fd for fd in fds],
        key=lambda x: x.is_dir(),
        reverse=True
    )




