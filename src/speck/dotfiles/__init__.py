import typer

from speck.dotfiles.commands import link
from speck.dotfiles.commands import ls
from speck.dotfiles.commands import unlink

app = typer.Typer()

_ = app.command()(link)
_ = app.command()(unlink)
_ = app.command(name="list")(ls)
