import re
from pathlib import Path
from filters import sorted_fd
import click

def display_matched(results: list[Path], match_key: str):
    for res in results:
        key = str(res)
        matched = re.match(
            str(key),
            str(res),
            re.IGNORECASE
        )

        g = matched.group()
        s = re.search(
            match_key,
            g,
            re.IGNORECASE
        ).span()

        click.echo(
            f"{key[:s[0]]}".split('/')[-1]+
            f"{click.style(
                f"{key[s[0]:s[1]]}",
                fg="red",
                underline=True,
                bold=True
            )}"
            f"{key[s[1]:]}"

        )

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
    click.echo("-" * 50)

