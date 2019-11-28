import unittest
from lognplot.tsdb.timespan import TimeSpan


class OverlapTestCase(unittest.TestCase):
    def test_no_overlap1(self):
        self.assertFalse(TimeSpan(1, 3).overlaps(TimeSpan(6, 7)))

    def test_no_overlap2(self):
        self.assertFalse(TimeSpan(10, 13).overlaps(TimeSpan(6, 7)))

    def test_overlap1(self):
        self.assertTrue(TimeSpan(1, 8).overlaps(TimeSpan(6, 17)))

    def test_overlap2(self):
        self.assertTrue(TimeSpan(1, 19).overlaps(TimeSpan(6, 17)))

    def test_overlap3(self):
        self.assertTrue(TimeSpan(8, 19).overlaps(TimeSpan(6, 17)))

    def test_overlap1_reversed(self):
        self.assertTrue(TimeSpan(6, 17).overlaps(TimeSpan(1, 8)))

    def test_overlap2_reversed(self):
        self.assertTrue(TimeSpan(6, 17).overlaps(TimeSpan(1, 19)))

    def test_overlap3_reversed(self):
        self.assertTrue(TimeSpan(6, 17).overlaps(TimeSpan(8, 19)))
