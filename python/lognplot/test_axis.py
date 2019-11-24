import unittest
from lognplot.axis import ceil_to_multiple_of, Axis


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
        # TODO: how about changing "-40.0" to "-40" (without decimal .0?)
        self.assertEqual(
            [
                (-40.0, "-40.0"),
                (-30.0, "-30.0"),
                (-20.0, "-20.0"),
                (-10.0, "-10.0"),
                (0.0, "0.0"),
                (10.0, "10.0"),
                (20.0, "20.0"),
                (30.0, "30.0"),
                (40.0, "40.0"),
            ],
            ticks,
        )
