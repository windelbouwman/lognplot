""" Re-usable chart widget.

Include this widget into your application to plot data.
"""
from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QPainter
from PyQt5.QtCore import Qt

from ...utils import bench_it
from ...chart import Chart
from ..render import render_chart_on_qpainter


class ChartWidget(QWidget):
    """ Charting widget.
    """

    def __init__(self):
        super().__init__()
        self.chart = Chart()

        # Make sure we grab keyboard input:
        self.setFocusPolicy(Qt.StrongFocus)

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QPainter(self)
        # with bench_it("render"):
        render_chart_on_qpainter(self.chart, painter, self.rect())

    # Panning helpers:
    PAN_FACTOR = 0.05

    def pan_left(self):
        self.horizontal_pan(-self.PAN_FACTOR)

    def pan_right(self):
        self.horizontal_pan(self.PAN_FACTOR)

    def pan_up(self):
        self.vertical_pan(self.PAN_FACTOR)

    def pan_down(self):
        self.vertical_pan(-self.PAN_FACTOR)

    # Zooming helpers:
    ZOOM_FACTOR = 0.1

    def zoom_in_horizontal(self):
        self.horizontal_zoom(-self.ZOOM_FACTOR)

    def zoom_out_horizontal(self):
        self.horizontal_zoom(self.ZOOM_FACTOR)

    def zoom_in_vertical(self):
        self.vertical_zoom(self.ZOOM_FACTOR)

    def zoom_out_vertical(self):
        self.vertical_zoom(-self.ZOOM_FACTOR)

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
        self.chart.zoom_fit()
        self.update()

    def keyPressEvent(self, e):
        super().keyPressEvent(e)
        key = e.key()
        if key == Qt.Key_D or key == Qt.Key_Right:
            self.pan_right()
        elif key == Qt.Key_A or key == Qt.Key_Left:
            self.pan_left()
        elif key == Qt.Key_W or key == Qt.Key_Up:
            self.pan_up()
        elif key == Qt.Key_S or key == Qt.Key_Down:
            self.pan_down()
        elif key == Qt.Key_J or key == Qt.Key_Plus:
            self.zoom_in_horizontal()
        elif key == Qt.Key_L or key == Qt.Key_Minus:
            self.zoom_out_horizontal()
        elif key == Qt.Key_K:
            self.zoom_out_vertical()
        elif key == Qt.Key_I:
            self.zoom_in_vertical()
        elif key == Qt.Key_Space:
            # Autoscale all in fit!
            self.zoom_fit()
        else:
            print("press key", e)
