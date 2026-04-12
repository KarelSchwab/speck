from dataclasses import dataclass
from pathlib import Path


@dataclass
class Dotfile:
    name: str
    source: Path
    destination: Path


@dataclass
class Repo:
    name: str
    url: str
    destination: Path


@dataclass
class Config:
    dotfiles: list[Dotfile]
    repos: list[Repo]
