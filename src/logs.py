from pathlib import Path
from filters import sorted_fd
import click

def display_matched(results: list[Path], match_key: str):
    for res in results:
        key = str(res)
        indx = key.find(match_key)



def display(
    contents: list[Path],
    require_sort: bool
):
    # TODO add sortintg flag
    if require_sort:
        contents = sorted_fd(contents)
    for entry in contents:
        fd = str(entry).split('/')[-1]
        if entry.is_dir():
            click.echo(click.style(f"{fd}/", fg="green"))
        else:
            click.echo(fd)
    click.echo("-" * 100)

