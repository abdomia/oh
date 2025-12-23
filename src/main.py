from pathlib import Path
import click

def get_fd(p: Path) -> list[Path]:
    conts = p.iterdir()
    return sorted(conts, key=lambda x: x.is_dir(), reverse=True)

@click.command(help="oh is an ls command but with super power")
@click.argument(
    "dest",
    required=False,
    nargs=-1
)
# TODO make this function pretty print of any number of args/dest
def oh(dest: tuple[str]):
    d = dest or '.'
    for entrys in d:
        fds = get_fd(Path(entrys))
        for item in fds:
            if item.is_dir():
                click.echo(click.style(f"{str(item)}/", fg="green"))
            else:
                click.echo(str(item))


# TODO think also of another implementations for this project
if __name__ == "__main__":
    oh()
