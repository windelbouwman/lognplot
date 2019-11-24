import unittest
from lognplot import Chart, ZoomSerie


class ChartTestCase(unittest.TestCase):
    def test_autoscale(self):
        chart = Chart()
        serie1 = ZoomSerie()
        chart.add_serie(serie1)

        # Zoom fit without points
        chart.zoom_fit()
        chart.autoscale_y()

        # Zoom fit with a single point:
        serie1.add_sample((10, 12))
        chart.zoom_fit()
        chart.autoscale_y()

        # Add an additional sample:
        serie1.add_sample((12, 13))
        chart.zoom_fit()
        chart.autoscale_y()
