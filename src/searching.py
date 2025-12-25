import re
from pathlib import Path

# from filters import sorted_fd

def search_fds(dest: Path) -> list[Path]:
    return [fds for fds in dest.iterdir()]

def search_match(match_input: str, dest_to_match: Path) -> list[Path]:
    mi = re.escape(match_input)
    all_matched = []
    fds = search_fds(dest_to_match)
    for fd in fds:
        a = re.findall(
            r'(?:^|\s)[^\s]*' + mi + r'[^\s]*',
            str(fd),
            re.IGNORECASE
        )
        for i in a:
            all_matched.append(Path(i))
    # TODO handle if all_matched is empty
    return all_matched


