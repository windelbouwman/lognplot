import contextlib
from ..qtapi import QtGui, QtCore, Qt


class BaseRenderer:
    def __init__(self, painter: QtGui.QPainter, layout):
        self.painter = painter
        self.layout = layout

    def draw_bouding_rect(self):
        """ Draw a bounding box around the plotting area. """
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        self.painter.setBrush(Qt.NoBrush)

        self.painter.drawRect(
            self.layout.chart_left,
            self.layout.chart_top,
            self.layout.chart_width,
            self.layout.chart_height,
        )

    def calc_x_ticks(self, axis):
        min_ticks = 2
        x_tick_spacing = 100
        amount_x_ticks = max(min_ticks, int(self.layout.chart_width // x_tick_spacing))
        x_ticks = axis.get_ticks(amount_x_ticks)
        return x_ticks

    def calc_y_ticks(self, axis):
        min_ticks = 2
        y_tick_spacing = 50
        amount_y_ticks = max(min_ticks, int(self.layout.chart_height // y_tick_spacing))
        y_ticks = axis.get_ticks(amount_y_ticks)
        return y_ticks

    def draw_grid(self, y_axis, x_ticks, y_ticks):
        """ Render a grid on the given x and y tick markers. """
        pen = QtGui.QPen(Qt.gray)
        pen.setWidth(1)
        self.painter.setPen(pen)

        for value, _ in x_ticks:
            x = self.to_x_pixel(value)
            self.painter.drawLine(x, self.layout.chart_top, x, self.layout.chart_bottom)

        for value, _ in y_ticks:
            y = self.to_y_pixel(y_axis, value)
            self.painter.drawLine(self.layout.chart_left, y, self.layout.chart_right, y)

    def draw_x_axis(self, x_ticks):
        """ Draw the X-axis. """
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        margin = 5
        y = self.layout.chart_bottom + margin
        self.painter.drawLine(self.layout.chart_left, y, self.layout.chart_right, y)
        for value, label in x_ticks:
            x = self.to_x_pixel(value)

            # Tick handle:
            self.painter.drawLine(x, y, x, y + 5)

            # Tick label:
            text_rect = self.painter.fontMetrics().boundingRect(label)
            text_x = x - text_rect.x() - text_rect.width() / 2
            text_y = y + 10 - text_rect.y()
            self.painter.drawText(text_x, text_y, label)

    def draw_y_axis(self, y_axis, y_ticks):
        """ Draw the Y-axis. """
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        x = self.layout.chart_right + 5
        self.painter.drawLine(x, self.layout.chart_top, x, self.layout.chart_bottom)
        for value, label in y_ticks:
            y = self.to_y_pixel(y_axis, value)

            # Tick handle:
            self.painter.drawLine(x, y, x + 5, y)

            # Tick label:
            text_rect = self.painter.fontMetrics().boundingRect(label)
            text_x = x + 10 - text_rect.x()
            text_y = y - text_rect.y() - text_rect.height() / 2
            self.painter.drawText(text_x, text_y, label)

    @contextlib.contextmanager
    def clip_chart_rect(self):
        self.painter.save()
        self.painter.setClipRect(
            self.layout.chart_left,
            self.layout.chart_top,
            self.layout.chart_width,
            self.layout.chart_height,
        )
        yield
        # Remove the clipping rectangle:
        self.painter.restore()

    def to_x_pixel(self, value):
        raise NotImplementedError()

    def to_y_pixel(self, value):
        raise NotImplementedError()
