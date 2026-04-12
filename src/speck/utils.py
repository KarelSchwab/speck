import os
from pathlib import Path


def expanded_path(str_path: str) -> Path:
    return Path(os.path.expanduser(os.path.expandvars(str_path)))
