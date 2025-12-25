from pathlib import Path

def sorted_fd(p: Path) -> list[Path]:
    conts = p.iterdir()
    return sorted(conts, key=lambda x: x.is_dir(), reverse=True)


