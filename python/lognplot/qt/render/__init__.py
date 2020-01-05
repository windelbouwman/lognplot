""" Render a graph on a QPainter. """
import contextlib
from ..qtapi import QtCore, QtGui
from ...chart import Chart
from .render import Renderer
from .layout import ChartLayout
from .options import ChartOptions
from .transform import *

def render_chart_on_qpainter(chart: Chart, painter: QtGui.QPainter, rect: QtCore.QRect):
    """ Call this function to paint a chart onto the given painter within the rectangle specified.
    """
    renderer = Renderer(painter, chart)
    # with bench_it("render"):
    renderer.render(rect)
