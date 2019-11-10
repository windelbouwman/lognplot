import unittest
from chart1.axis import ceil_to_multiple_of


class RoundTestCase(unittest.TestCase):
    def test_ceil_to_multiple_of(self):
        self.assertAlmostEqual(0, ceil_to_multiple_of(-4, 10))
        self.assertAlmostEqual(-5, ceil_to_multiple_of(-6, 5))
        self.assertAlmostEqual(5, ceil_to_multiple_of(1, 5))
        self.assertAlmostEqual(5, ceil_to_multiple_of(5, 5))
        self.assertAlmostEqual(14, ceil_to_multiple_of(13, 7))
