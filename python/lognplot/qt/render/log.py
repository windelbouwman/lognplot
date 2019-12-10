""" Render logged messages on a QPainter.

Ideas:
- Render levels as colors.
- Render log message as a block containing the log message as text.
- Render aggregates as grouping of logs or highest level as color?
"""

from ..qtapi import QtGui, QtCore, Qt
from .layout import ChartLayout
from .options import ChartOptions
from .base import BaseRenderer
from . import transform
from ...logbar import LogLevel


def render_logs_on_qpainter(logbar, painter: QtGui.QPainter, rect: QtCore.QRect):
    options1 = ChartOptions()
    layout = ChartLayout(rect, options1)
    log_renderer = LogBarRenderer(painter, logbar, layout)
    log_renderer.render()


class LogBarRenderer(BaseRenderer):
    def __init__(self, painter: QtGui.QPainter, logbar, layout):
        super().__init__(painter, layout)
        self.logbar = logbar
        self.padding = 8

    def render(self):
        self.draw_bouding_rect()
        self.draw_axis()
        self.draw_log_messages()

    def draw_axis(self):
        x_ticks = self.calc_x_ticks(self.logbar.x_axis)
        self.draw_x_axis(x_ticks)

    def draw_log_messages(self):
        font = QtGui.QFont("courier", 20)
        # Font color:
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        self.painter.setFont(font)

        y = self.layout.chart_top + self.padding
        dy = self.painter.fontMetrics().height() + 3 * self.padding

        with self.clip_chart_rect():
            for log_track in self.logbar.log_tracks:
                self.draw_log_track(y, log_track)
                y += dy

    def draw_log_track(self, y, stamped_records):
        color_map = {
            LogLevel.INFO: Qt.green,
            LogLevel.WARNING: Qt.yellow,
            LogLevel.ERROR: Qt.red,
        }
        for timestamp, log_record in stamped_records:
            x = self.to_x_pixel(timestamp)
            log_text = log_record.message

            text_rect = self.painter.fontMetrics().boundingRect(log_text)

            # Draw background:
            fill_color = color_map[log_record.level]
            log_record_rect = QtCore.QRect(
                x,
                y,
                text_rect.width() + self.padding * 2,
                text_rect.height() + self.padding * 2,
            )
            self.painter.fillRect(log_record_rect, fill_color)
            self.painter.drawRect(log_record_rect)

            # Draw text:
            text_x = x - text_rect.x() + self.padding
            text_y = y - text_rect.y() + self.padding
            self.painter.drawText(text_x, text_y, log_text)

    def to_x_pixel(self, value):
        return transform.to_x_pixel(value, self.logbar.x_axis, self.layout)
