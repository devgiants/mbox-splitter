"""Use case for splitting mail data."""


def split(filename: str) -> list:
    """Split input file content in several parts"""

    with open(filename, encoding="utf-8") as file:
        mails = []
        data = file.read()
        if len(data) > 0:
            mails.append(data)
        return mails
