import re
import click
from pathlib import Path
from filters import sorted_fd

def search_match(match_input: str, paths_to_match: tuple[str]) -> list[Path]:
    mi = re.escape(match_input)
    for fds in sorted_fd(paths_to_match):
        all_matched = [
            Path(p) for p in re.findall(
                r'(?:^|\s)[^\s]*' + mi + r'[^\s]*',
                fds,
                re.IGNORECASE
            )]
    return all_matched


def display(
    dest: str,
    is_matched_specif: bool,
    default_opts: bool
):
    # TODO make sort/unsorted/dir_only/files_only/.... impls
    fds = sorted_fd(Path(dest))
    for entry in fds:
        fd = str(entry).split('/')[-1]
        if entry.is_dir():
            click.echo(click.style(f"{fd}/", fg="green"))
        else:
            click.echo(fd)
    click.echo("-" * 100)

