import unittest
from lognplot.utils import chunk, clip


class RoundTestCase(unittest.TestCase):
    def test_chunking_with_left_over(self):
        self.assertEqual([[0, 1], [2, 3], [4]], list(chunk(range(5), 2)))

    def test_chunking_precisely(self):
        self.assertEqual([[0, 1, 2], [3, 4, 5]], list(chunk(range(6), 3)))

    def test_chunking_empty_list(self):
        self.assertEqual([], list(chunk([], 3)))


class ClipTestCase(unittest.TestCase):
    def test_clipping(self):
        self.assertEqual(clip(1, 0, 10), 1)
        self.assertEqual(clip(1, 2, 10), 2)
        self.assertEqual(clip(12, 2, 10), 10)
