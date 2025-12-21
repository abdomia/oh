
from pathlib import Path
from rich import print as p

def get_fd(path: Path) -> None:
    for entrey in path.iterdir():
        if entrey.is_dir():
            p(f'[yellow]{entrey}[yellow]')
        else:
            print(entrey)


def main() -> None:
    fd = Path('.')
    get_fd(fd)


if __name__ == "__main__":
    main()
