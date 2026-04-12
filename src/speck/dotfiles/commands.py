from pathlib import Path
from typing import Annotated, cast

import typer

from speck.config import Config, Dotfile


def _remove_path(destination: Path):
    if destination.is_symlink() or destination.is_file(follow_symlinks=True):
        destination.unlink(missing_ok=True)
    elif destination.is_dir(follow_symlinks=True):
        destination.rmdir()


def _filter_dotfiles(
    unfiltered_dotfiles: list[Dotfile], dotfile_names: list[str] | None = None
) -> list[Dotfile] | filter[Dotfile]:
    if dotfile_names:
        return filter(
            lambda dotfile: dotfile.name in dotfile_names, unfiltered_dotfiles
        )
    return unfiltered_dotfiles


def ls(ctx: typer.Context, linked: Annotated[bool | None, typer.Option()] = None):
    try:
        config = cast(Config, ctx.obj)
        for dotfile in config.dotfiles:
            name = dotfile.name
            destination = dotfile.destination
            source = dotfile.source
            if linked is None:
                typer.echo(f"{name}: {source} => {destination}")
                continue
            if linked:
                if source.resolve() == destination.resolve():
                    typer.echo(f"{name}: {source} => {destination}")
            else:
                if source.resolve() != destination.resolve():
                    typer.echo(f"{name}: {source} => {destination}")
    except Exception as e:
        typer.echo(str(e), err=True)
        raise typer.Exit(1)


def link(
    ctx: typer.Context, dotfiles: Annotated[list[str] | None, typer.Argument()] = None
):
    try:
        config = cast(Config, ctx.obj)
        filtered_dotfiles = _filter_dotfiles(config.dotfiles, dotfiles)
        for dotfile in filtered_dotfiles:
            destination = dotfile.destination
            source = dotfile.source
            destination.parent.mkdir(parents=True, exist_ok=True)
            _remove_path(destination)
            destination.symlink_to(source)
            typer.echo(f"Linked {source} => {destination}")
    except Exception as e:
        typer.echo(str(e), err=True)
        raise typer.Exit(1)


def unlink(
    ctx: typer.Context, dotfiles: Annotated[list[str] | None, typer.Argument()] = None
):
    try:
        config = cast(Config, ctx.obj)
        filtered_dotfiles = _filter_dotfiles(config.dotfiles, dotfiles)

        for dotfile in filtered_dotfiles:
            destination = dotfile.destination
            _remove_path(destination)
            typer.echo(f"Unlinked {destination}")
    except Exception as e:
        typer.echo(str(e), err=True)
        raise typer.Exit(1)
