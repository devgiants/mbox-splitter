"""Data splitter test cases"""

from domain.use_case.split_use_case import split


def test_empty_file_should_return_empty_string():
    """Split empty file should return empty string"""

    assert split("tests/data/empty.mbox") == ""
