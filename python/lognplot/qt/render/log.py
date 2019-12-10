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
from ...tsdb import LogLevel, Aggregation


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

        for log_track in self.logbar.log_tracks:
            with self.clip_chart_rect():
                self.draw_log_track(y, log_track)
                y += dy
            # Draw label on y axis:
            legend_x = self.layout.chart_right + 5
            legend_y = y
            self.painter.drawText(legend_x, legend_y, log_track.name)

    def draw_log_track(self, y, log_track):
        timespan = self.logbar.x_axis.get_timespan()
        # Determine how many data points we wish to visualize
        # This greatly determines drawing performance.
        min_count = int(self.layout.chart_width / 100)
        data = log_track.query(timespan, min_count)
        if data:
            if isinstance(data[0], Aggregation):
                self.draw_message_bundles(y, data)
            else:
                self.draw_single_records(y, data)

    def draw_single_records(self, y, stamped_records):
        """ Draw individual log messages.
        """
        color_map = {
            LogLevel.INFO: Qt.green,
            LogLevel.WARNING: Qt.yellow,
            LogLevel.ERROR: Qt.red,
        }
        for timestamp, log_record in stamped_records:
            x = self.to_x_pixel(timestamp)
            log_text = log_record.message

            text_rect = self.painter.fontMetrics().boundingRect(log_text)

            width = text_rect.width() + self.padding * 2
            height = text_rect.height() + self.padding * 2
            log_record_rect = QtCore.QRect(x, y, width, height,)

            # Draw background:
            fill_color = color_map[log_record.level]
            self.painter.fillRect(log_record_rect, fill_color)
            self.painter.drawRect(log_record_rect)

            # Draw text:
            text_x = x - text_rect.x() + self.padding
            text_y = y - text_rect.y() + self.padding
            self.painter.drawText(text_x, text_y, log_text)

    def draw_message_bundles(self, y, aggregates):
        """ Draw log message aggregations.
        """
        for aggregate in aggregates:
            x1 = self.to_x_pixel(aggregate.timespan.begin)
            x2 = self.to_x_pixel(aggregate.timespan.end)
            width = x2 - x1
            height = self.painter.fontMetrics().height() + self.padding * 2
            log_record_rect = QtCore.QRect(x1, y, width, height,)
            # TODO: do something smart with the color?
            # Draw a color flag? Draw stats bar?
            fill_color = Qt.cyan
            self.painter.fillRect(log_record_rect, fill_color)
            self.painter.drawRect(log_record_rect)

    def to_x_pixel(self, value):
        return transform.to_x_pixel(value, self.logbar.x_axis, self.layout)
