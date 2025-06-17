"""Data splitter test cases"""

from domain.use_case.split import Split


def test_empty_file_should_return_empty_string():
    """Split empty file should return empty string"""
    splitter = Split()
    assert splitter.split("tests/data/empty.mbox") == ""
