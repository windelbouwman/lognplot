import unittest
from lognplot.tsdb.aggregation import Aggregation


class MatricsTestCase(unittest.TestCase):
    def test_metrics(self):
        samples = [(x + 20, x) for x in range(1, 6)]
        aggregation = Aggregation.from_samples(samples)
        metrics = aggregation.metrics
        self.assertEqual(5, metrics.count)
        self.assertAlmostEqual(3, metrics.mean)
        self.assertAlmostEqual(1.5811388301, metrics.stddev)
        self.assertAlmostEqual(5, metrics.maximum)
        self.assertAlmostEqual(1, metrics.minimum)
        self.assertAlmostEqual(21, aggregation.timespan.begin)
        self.assertAlmostEqual(25, aggregation.timespan.end)
