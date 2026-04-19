import subprocess
from typing import Annotated, cast

import typer

from speck.config import Config, Repo


def _filter_repos(
    unfiltered_repos: list[Repo], repo_names: list[str] | None = None
) -> list[Repo] | filter[Repo]:
    if repo_names:
        return filter(lambda repo: repo.name in repo_names, unfiltered_repos)
    return unfiltered_repos


def clone(
    ctx: typer.Context, repos: Annotated[list[str] | None, typer.Argument()] = None
):
    try:
        config = cast(Config, ctx.obj)
        filtered_repos = _filter_repos(config.repos, repos)
        for repo in filtered_repos:
            if repo.destination.exists():
                typer.echo(f"Repo at {repo.destination} already exists. Skipping...")
            else:
                _ = subprocess.run(
                    ["git", "clone", repo.url, repo.destination], check=True
                )
                typer.echo(f"Cloned {repo.name} to {repo.destination}")

    except Exception as e:
        typer.echo(str(e), err=True)
        raise typer.Exit(1)
