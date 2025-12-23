from pathlib import Path
from rich import print as p

def get_fd(path: Path) -> tuple[list[Path], list[Path]]:
    l_f: list[Path] = []
    l_d: list[Path] = []
    for entry in path.iterdir():
        if entry.is_dir():
            l_d.append(entry)
        else:
            l_f.append(entry)
    return (l_f, l_d)

def main() -> None:
    fd = Path('.')
    f, d = get_fd(fd)

    for ds in sorted(d):
        p(f'[yellow]{str(ds)}/[yellow]')
    for fs in sorted(f):
        print(str(fs))


if __name__ == "__main__":
    main()
