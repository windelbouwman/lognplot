import logging
from ..qtapi import QtCore, QtWidgets, QtGui, Qt, pyqtSignal


class BaseWidget(QtWidgets.QWidget):
    """ Base class for visualization widgets.

    Features:
    - keyboard focus
    - mouse panning?
    """

    logger = logging.getLogger("base-widget")

    def __init__(self):
        super().__init__()

        # Make sure we grab keyboard input:
        self.setFocusPolicy(Qt.StrongFocus)

        self._mouse_drag_source = None

    def mousePressEvent(self, event):
        super().mousePressEvent(event)
        self.disable_tailing()
        self._mouse_drag_source = event.x(), event.y()
        self.mousePress(event.x(), event.y())
        self.update()

    def mouseMoveEvent(self, event):
        super().mouseMoveEvent(event)
        self._update_mouse_pan(event.x(), event.y())

    def mouseReleaseEvent(self, event):
        super().mouseReleaseEvent(event)
        self._update_mouse_pan(event.x(), event.y())
        self._mouse_drag_source = None
        self.mouseRelease(event.x(), event.y())

    def _update_mouse_pan(self, x, y):
        if self._mouse_drag_source:
            x0, y0 = self._mouse_drag_source
            if x != x0 or y != y0:
                dy = y - y0
                dx = x - x0
                self.mouseDrag(x, y, dx, dy)
                self.pan(dx, dy)
                self._mouse_drag_source = (x, y)
                self.update()

    def mousePress(self, x, y):
        pass

    def mouseRelease(self, x, y):
        pass

    def mouseDrag(self, x, y, dx, dy):
        pass

    def pan(self, dx, dy):
        """ Intended for subclasses to override.
        """
        pass

    def draw_focus_indicator(self, painter, rect):
        """ Draw focus indicator """
        if self.hasFocus():
            pen = QtGui.QPen(Qt.red)
            pen.setWidth(4)
            painter.setPen(pen)
            painter.drawRect(rect)

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

    # Overridable methods:
    def horizontal_zoom(self, amount):
        pass

    def vertical_zoom(self, amount):
        pass

    def horizontal_pan(self, amount):
        pass

    def vertical_pan(self, amount):
        pass

    def zoom_fit(self):
        pass

    def clear_curves(self):
        pass

    def disable_tailing(self):
        pass

    def keyPressEvent(self, e):
        super().keyPressEvent(e)
        self.disable_tailing()
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
        elif key == Qt.Key_Space or key == Qt.Key_Return:
            self.zoom_fit()
        elif key == Qt.Key_Backspace or key == Qt.Key_Delete:
            self.clear_curves()
        else:
            print("press key", e)
