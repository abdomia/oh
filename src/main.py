from pathlib import Path
import click

def get_fd(p: Path) -> list[Path]:
    conts = p.iterdir()
    return sorted(conts, key=lambda x: x.is_dir(), reverse=True)

@click.command(help="oh is an ls command but with super power")
@click.argument("path", required=False)
def oh(path: str):
    p = path or './'
    fds = get_fd(Path(p))
    for item in fds:
        if item.is_dir():
            click.echo(click.style(f"{str(item)}/", fg="green"))
        else:
            click.echo(str(item))

if __name__ == "__main__":
    oh()
