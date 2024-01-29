#!/usr/bin/env python

import click
from oalib.solutions import create_code

# create a click group
@click.group()
def cli():
    """Generate code examples from comments in multiple languages"""


# create a click command
@cli.command("generate")
@click.option("--language", "-l", default="python", help="The language to use")
@click.argument("text")
def mkcode(text, language):
    """Convert a comment into code in any language

    Example:
        ./mkcode generate -l python "Calculate the mean distance between an array of points"

    """

    print(create_code(text, language))


if __name__ == "__main__":
    cli()
