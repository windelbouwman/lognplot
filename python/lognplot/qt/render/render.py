""" Render a graph on a QPainter. """
import contextlib
from PyQt5.QtGui import QPainter, QPen, QPolygon, QBrush, QColor
from PyQt5.QtCore import QRect, Qt, QPoint
from ...chart import Chart
from ...utils import bench_it, clip
from ...tsdb.metrics import Metrics
from .layout import ChartLayout
from .chart import ChartRenderer


class ChartOptions:
    def __init__(self):
        self.show_axis = True
        self.show_grid = True
        self.padding = 10


class Renderer:
    """ Render a chart.

    Optionally include a minimap?
    """

    def __init__(self, painter: QPainter, chart: Chart):
        self.painter = painter
        self.chart = chart

    def render(self, rect: QRect):
        options1 = ChartOptions()
        chart_renderer = ChartRenderer(self.painter, rect, self.chart, options1)
        chart_renderer.render()

        # Create a new chart with the whole thing zoomed
        minimap_chart = Chart(self.chart.db)
        for curve in self.chart.curves:
            minimap_chart.add_curve(curve.name, curve.color)
        minimap_chart.zoom_fit()

        # Now render minimap in top left corner.
        minimap_options = ChartOptions()
        minimap_options.padding = 2
        minimap_options.show_axis = False
        minimap_rect = QRect(rect.x() + 40, rect.y() + 40, 120, 80)
        self.painter.fillRect(minimap_rect, Qt.yellow)
        minimap_chart_renderer = ChartRenderer(
            self.painter, minimap_rect, minimap_chart, minimap_options
        )
        minimap_chart_renderer.render()
        region = self.chart.get_region()
        minimap_chart_renderer.shade_region(region)
