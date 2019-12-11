""" Re-usable chart widget.

Include this widget into your application to plot data.
"""
from itertools import cycle
import logging

from ..qtapi import QtCore, QtWidgets, QtGui, Qt, pyqtSignal
from ...utils import bench_it
from ...chart import Chart
from ..render import render_chart_on_qpainter, ChartLayout, ChartOptions
from . import mime
from .basewidget import BaseWidget

color_wheel = ["blue", "red", "green", "black", "yellow"]


class ChartWidget(BaseWidget):
    """ Charting widget.
    """

    logger = logging.getLogger("chart-widget")

    def __init__(self, db):
        super().__init__()
        self.chart = Chart(db)
        self._colors = cycle(color_wheel)

        # Accept drop of signal names
        self.setAcceptDrops(True)

        # Tailing mode, select last t seconds
        self._last_span = None
        self._tailing_timer = QtCore.QTimer()
        self._tailing_timer.timeout.connect(self._on_tailing_timeout)
        self._tailing_timer.start(50)

    # Drag drop events:
    def dragEnterEvent(self, event):
        if event.mimeData().hasFormat(mime.signal_names_mime_type):
            event.acceptProposedAction()

    def dropEvent(self, event):
        names = bytes(event.mimeData().data(mime.signal_names_mime_type)).decode(
            "ascii"
        )
        for name in names.split(":"):
            self.logger.debug(f"Add curve {name}")
            self.add_curve(name)

    # Mouse interactions:
    def pan(self, dx, dy):
        print("pan", dx, dy)
        options1 = ChartOptions()
        layout = ChartLayout(self.rect(), options1)

    def add_curve(self, name, color=None):
        if not self.chart.has_curve(name):
            color = color or next(self._colors)
            self.chart.add_curve(name, color)

            # When adding a curve, autozoom is the first thing:
            self.zoom_fit()

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QtGui.QPainter(self)
        # with bench_it("render"):
        render_chart_on_qpainter(self.chart, painter, self.rect())

        self.draw_focus_indicator(painter, self.rect())

    def horizontal_zoom(self, amount):
        self.chart.horizontal_zoom(amount)
        # Autoscale Y for a nice effect?
        self.chart.autoscale_y()
        self.update()

    def vertical_zoom(self, amount):
        self.chart.vertical_zoom(amount)
        self.update()

    def horizontal_pan(self, amount):
        self.chart.horizontal_pan(amount)
        # Autoscale Y for a nice effect?
        self.chart.autoscale_y()
        self.update()

    def vertical_pan(self, amount):
        self.chart.vertical_pan(amount)
        self.update()

    def zoom_fit(self):
        """ Autoscale all in fit! """
        self.chart.zoom_fit()
        self.update()

    def zoom_to_last(self, span):
        """ Zoom to fit the last x time in view.
        """
        self.chart.zoom_to_last(span)
        self.update()

    def enable_tailing(self, timespan):
        """ Slot to enable tailing the last timespan of the signals. """
        self._last_span = timespan

    def disable_tailing(self):
        """ Stop tailing the signals in view. """
        self._last_span = None

    def _on_tailing_timeout(self):
        # Follow last x seconds:
        if self._last_span:
            self.zoom_to_last(self._last_span)

    def clear_curves(self):
        """ Clear all curves """
        self.chart.clear_curves()
        self.update()
