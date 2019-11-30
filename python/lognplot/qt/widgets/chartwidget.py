""" Re-usable chart widget.

Include this widget into your application to plot data.
"""
from itertools import cycle
from ..qtapi import QtCore, QtWidgets, QtGui, Qt, pyqtSignal
from ...utils import bench_it
from ...chart import Chart
from ..render import render_chart_on_qpainter

color_wheel = ["blue", "red", "green", "black", "yellow"]


class ChartWidget(QtWidgets.QWidget):
    """ Charting widget.
    """

    manually_zoomed = pyqtSignal()

    def __init__(self, db):
        super().__init__()
        self.chart = Chart(db)
        self._colors = cycle(color_wheel)

        # Make sure we grab keyboard input:
        self.setFocusPolicy(Qt.StrongFocus)

        # Accept drop of signal names
        self.setAcceptDrops(True)

        self._mouse_drag_source = None

    # Drag drop events:
    def dragEnterEvent(self, event):
        # print("drag enter!")
        if event.mimeData().hasFormat("text/plain"):
            # print("accept drag")
            event.acceptProposedAction()

    def dragMoveEvent(self, event):
        # print("drag move event!")
        pass

    def dragLeaveEvent(self, event):
        # print("drag leave!")
        pass

    def dropEvent(self, event):
        names = event.mimeData().text()
        # print("Mime data text", names, type(names))
        for name in names.split(":"):
            self.add_curve(name)

    # Mouse interactions:
    def mousePressEvent(self, event):
        super().mousePressEvent(event)
        self.manually_zoomed.emit()
        self._mouse_drag_source = event.x(), event.y()
        self.update()

    def mouseMoveEvent(self, event):
        super().mouseMoveEvent(event)
        self._update_mouse_pan(event.x(), event.y())

    def mouseReleaseEvent(self, event):
        super().mouseReleaseEvent(event)
        self._update_mouse_pan(event.x(), event.y())
        self._mouse_drag_source = None

    def _update_mouse_pan(self, x, y):
        if self._mouse_drag_source:
            x0, y0 = self._mouse_drag_source
            if x != x0 or y != y0:
                dy = y - y0
                dx = x - x0
                self.pan(dx, dy)
                self._mouse_drag_source = (x, y)
                self.update()

    def pan(self, dx, dy):
        print("pan", dx, dy)

    def add_curve(self, name, color=None):
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

        # Draw focus indicator:
        if self.hasFocus():
            pen = QtGui.QPen(Qt.red)
            pen.setWidth(4)
            painter.setPen(pen)
            painter.drawRect(self.rect())

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

    def zoom_to_last(self, span):
        """ Zoom to fit the last x time in view.
        """
        self.chart.zoom_to_last(span)
        self.update()

    def keyPressEvent(self, e):
        super().keyPressEvent(e)
        self.manually_zoomed.emit()
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
