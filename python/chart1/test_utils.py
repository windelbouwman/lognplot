import unittest
from chart1.utils import chunk


class RoundTestCase(unittest.TestCase):
    def test_chunking_with_left_over(self):
        self.assertEqual([[0, 1], [2, 3], [4]], list(chunk(range(5), 2)))

    def test_chunking_precisely(self):
        self.assertEqual([[0, 1, 2], [3, 4, 5]], list(chunk(range(6), 3)))

    def test_chunking_empty_list(self):
        self.assertEqual([], list(chunk([], 3)))
