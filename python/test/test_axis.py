import unittest
from lognplot.chart.axis import ceil_to_multiple_of, Axis


class RoundTestCase(unittest.TestCase):
    def test_ceil_to_multiple_of(self):
        self.assertAlmostEqual(0, ceil_to_multiple_of(-4, 10))
        self.assertAlmostEqual(-5, ceil_to_multiple_of(-6, 5))
        self.assertAlmostEqual(5, ceil_to_multiple_of(1, 5))
        self.assertAlmostEqual(5, ceil_to_multiple_of(5, 5))
        self.assertAlmostEqual(14, ceil_to_multiple_of(13, 7))


class AxisTestCase(unittest.TestCase):
    def test_ticks(self):
        ax1 = Axis()
        ax1.minimum = -44
        ax1.maximum = 46

        ticks = ax1.get_ticks(7)
        expected_ticks = [
            (-40.0, "-40"),
            (-30.0, "-30"),
            (-20.0, "-20"),
            (-10.0, "-10"),
            (0.0, "0"),
            (10.0, "10"),
            (20.0, "20"),
            (30.0, "30"),
            (40.0, "40"),
        ]

        self.assertTicksEqual(ticks, expected_ticks)

    def test_small_ticks(self):
        ax1 = Axis()
        ax1.minimum = 1.5
        ax1.maximum = 3.3

        ticks = ax1.get_ticks(7)
        expected_ticks = [
            (1.6, "1.6"),
            (1.8, "1.8"),
            (2.0, "2.0"),
            (2.2, "2.2"),
            (2.4, "2.4"),
            (2.6, "2.6"),
            (2.8, "2.8"),
            (3.0, "3.0"),
            (3.2, "3.2"),
        ]
        self.assertTicksEqual(ticks, expected_ticks)

    def assertTicksEqual(self, ticks1, ticks2):
        self.assertEqual(len(ticks1), len(ticks2))
        for t1, t2 in zip(ticks1, ticks2):
            self.assertEqual(t1[1], t2[1])
            self.assertAlmostEqual(t1[0], t2[0])
