import unittest
from lognplot.chart import Chart, Curve
from lognplot.tsdb import TsDb


class ChartTestCase(unittest.TestCase):
    def test_autoscale(self):
        db = TsDb()
        chart = Chart(db)
        chart.add_curve("S1", "green")

        # Zoom fit without points
        chart.zoom_fit()
        chart.autoscale_y()

        # Zoom fit with a single point:
        db.add_sample("S1", (10, 12))
        chart.zoom_fit()
        chart.autoscale_y()

        # Add an additional sample:
        db.add_sample("S1", (12, 13))
        chart.zoom_fit()
        chart.autoscale_y()
