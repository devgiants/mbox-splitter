"""Data splitter test cases"""

from domain.use_case.split_use_case import split


class TestSplitter:
    """Gather all splitter tests"""

    def test_empty_file_should_return_empty_list(self):
        """Split empty file should return empty string"""

        assert len(split("tests/data/empty.mbox")) == 0

    def test_should_return_one_mail_list_with_complete_content(self):
        """
        Split file with one mail inside should return
        one list with entire file content
        """
        filename = "tests/data/one_mail.mbox"
        mails = split(filename)
        assert len(mails) == 1
        with open(filename, encoding="utf-8") as file:
            assert mails.pop() == file.read()
