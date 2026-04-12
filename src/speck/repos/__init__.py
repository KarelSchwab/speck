import typer

from speck.repos.commands import clone

app = typer.Typer()

_ = app.command()(clone)
