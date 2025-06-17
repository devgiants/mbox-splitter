"""Use case for splitting mail data."""


def split(filename: str) -> str:
    """Split input data in several parts"""

    with open(filename, encoding="utf-8") as file:
        return file.read()
