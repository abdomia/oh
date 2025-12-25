import re
from pathlib import Path

# from filters import sorted_fd

def search_fds(dest: Path) -> list[Path]:
    return [Path(fds) for fds in dest.iterdir()]

def search_match(match_input: str, dest_to_match: Path) -> list[Path]:
    mi = re.escape(match_input)
    all_matched = []
    for fds in dest_to_match.iterdir():
        a = re.findall(
            r'(?:^|\s)[^\s]*' + mi + r'[^\s]*',
            str(fds),
            re.IGNORECASE
        )
        for i in a:
            all_matched.append(Path(i))
    return all_matched


