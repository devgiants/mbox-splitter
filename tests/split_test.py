"""Data splitter test cases"""

from domain.use_case.split import Split


def test_empty_data_should_return_empty_string():
    """Split empty data should return empty string"""
    splitter = Split()
    assert splitter.split("") == ""
