""" Render a graph on a QPainter. """
import contextlib
from PyQt5.QtGui import QPainter
from PyQt5.QtCore import QRect
from ...chart import Chart
from .render import Renderer


def render_chart_on_qpainter(chart: Chart, painter: QPainter, rect: QRect):
    """ Call this function to paint a chart onto the given painter within the rectangle specified.
    """
    renderer = Renderer(painter, chart)
    # with bench_it("render"):
    renderer.render(rect)
