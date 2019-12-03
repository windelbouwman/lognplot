import unittest
from lognplot.tsdb import Metrics, Aggregation


class MatricsTestCase(unittest.TestCase):
    def test_metrics(self):
        samples = [(x + 20, x) for x in range(1, 6)]
        values = [s[1] for s in samples]
        metrics = Metrics.from_values(values)
        self.assertEqual(5, metrics.count)
        self.assertAlmostEqual(3, metrics.mean)

        # Check population variance (statistics.pstddev)
        self.assertAlmostEqual(1.4142135623730951, metrics.stddev)

        self.assertAlmostEqual(5, metrics.maximum)
        self.assertAlmostEqual(1, metrics.minimum)

        aggregation = Aggregation.from_samples(samples)
        self.assertAlmostEqual(21, aggregation.timespan.begin)
        self.assertAlmostEqual(25, aggregation.timespan.end)

    def test_metric_combination(self):
        values1 = list(range(1, 6))
        metrics1 = Metrics.from_values(values1)
        self.assertEqual(5, metrics1.count)
        self.assertAlmostEqual(3, metrics1.mean)

        # Check population variance (statistics.pstddev)
        self.assertAlmostEqual(1.4142135623730951, metrics1.stddev)

        self.assertAlmostEqual(5, metrics1.maximum)
        self.assertAlmostEqual(1, metrics1.minimum)

        values2 = list(range(5, 11))
        metrics2 = Metrics.from_values(values2)
        self.assertEqual(6, metrics2.count)
        self.assertAlmostEqual(7.5, metrics2.mean)

        # Check population variance (statistics.pstddev)
        self.assertAlmostEqual(1.707825127659933, metrics2.stddev)

        self.assertAlmostEqual(10, metrics2.maximum)
        self.assertAlmostEqual(5, metrics2.minimum)

        metrics3 = Metrics.from_metrics([metrics1, metrics2])
        self.assertEqual(11, metrics3.count)
        self.assertAlmostEqual(5.454545454545454, metrics3.mean)

        # Check population variance (statistics.pstddev)
        self.assertAlmostEqual(2.7423823870906103, metrics3.stddev)

        self.assertAlmostEqual(10, metrics3.maximum)
        self.assertAlmostEqual(1, metrics3.minimum)
