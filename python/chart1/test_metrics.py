import unittest
from chart1.metrics import samples_to_metric


class MatricsTestCase(unittest.TestCase):
    def test_metrics(self):
        samples = [(x + 20, x) for x in range(1, 6)]
        metrics = samples_to_metric(samples)
        self.assertEqual(5, metrics.count)
        self.assertAlmostEqual(3, metrics.mean)
        self.assertAlmostEqual(1.5811388301, metrics.stddev)
        self.assertAlmostEqual(5, metrics.maximum)
        self.assertAlmostEqual(1, metrics.minimum)
        self.assertAlmostEqual(21, metrics.x1)
        self.assertAlmostEqual(25, metrics.x2)
