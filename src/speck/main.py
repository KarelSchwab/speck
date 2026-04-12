from pathlib import Path
import tomllib

import typer

from speck.config import Config, Dotfile, Repo
from speck.dotfiles import app as dotfiles_app
from speck.repos import app as repos_app
from speck.utils import expanded_path

app = typer.Typer()

app.add_typer(dotfiles_app, name="dotfiles")
app.add_typer(repos_app, name="repos")


@app.callback()
def load_config(ctx: typer.Context):
    config_path = Path(Path.home(), ".config", "speck", "config.toml")
    with open(config_path, "rb") as c:
        config = tomllib.load(c)
        dotfiles: list[Dotfile] = []
        for dotfile in config.get("dotfiles"):
            name: str = dotfile.get("name")
            source: str = dotfile.get("source")
            destination: str = dotfile.get("destination")

            if any([not name, not source, not destination]):
                typer.echo(
                    f"ERROR reading dotfile from config: {name}, {source}, {destination}",
                    err=True,
                )
                raise typer.Exit(1)

            source_path = expanded_path(source)
            if not source_path.exists():
                typer.echo(
                    f"ERROR reading dotfile from config: {source_path.resolve()} does not exist",
                    err=True,
                )
                raise typer.Exit(1)

            destination_path = expanded_path(destination)
            dotfiles.append(
                Dotfile(name=name, source=source_path, destination=destination_path)
            )

        repos: list[Repo] = []
        for repo in config.get("repos"):
            name: str = repo.get("name")
            url: str = repo.get("url")
            destination: str = repo.get("destination")

            if any([not name, not url, not destination]):
                typer.echo(
                    f"ERROR reading repo from config: {name}, {url}, {destination}",
                    err=True,
                )
                raise typer.Exit(1)

            destination_path = expanded_path(destination)

            # TODO: Validate url with urllib
            repos.append(Repo(name=name, url=url, destination=destination_path))

        ctx.obj = Config(dotfiles=dotfiles, repos=repos)


def main():
    app()
