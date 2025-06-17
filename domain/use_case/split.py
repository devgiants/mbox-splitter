"""Use case for splitting mail data."""


class Split:  # pylint: disable=too-few-public-methods
    """Split use case class"""

    def split(self, filename: str) -> str:
        """Split input data in several parts"""

        with open(filename, encoding="utf-8") as file:
            return file.read()
