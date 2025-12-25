from pathlib import Path
import click
from searching import search_fds, search_match
from logs import display, display_matched


# TODO make this function pretty print of any number of args/dest
@click.command(help="""
    oh is an ls command but with super power
""")
@click.option(
    "-m", "--match",
    help="""
    If specified then it will search for the given word that match any file/dir (name) in [DESTS...]
        """,
    type=str,
    required=False
)
@click.option(
    "-r", "--rec",
    help="""
    Used for show sub directories in the specified [DESTS...]. if used with -m it will search
    recursively for the word has given
        """,
    is_flag=True
)
@click.argument(
    "dests",
    type=str,
    required=False,
    default=['.'],
    nargs=-1
)
def oh(
    match: str,
    rec,
    dests: tuple[str]
) -> None:
    for dest in dests:
        if match:
            res = search_match(match, Path(dest))
            display_matched(res, match)
        # TODO implement these functions
            if rec:
                # res = search_match_rec()
                ...
        elif rec:
            # res = search_rec()
            ...
        else:
            res = search_fds(Path(dest))
            display(res, True)


# TODO think also of another implementations for this project
if __name__ == "__main__":
    oh()

