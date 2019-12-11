""" Render event observations on a QPainter.

Ideas:
- render each event with it's own attributes.
"""

from ..qtapi import QtGui, QtCore, Qt
from .layout import ChartLayout
from .options import ChartOptions
from .base import BaseRenderer
from . import transform
from ...tsdb import Aggregation


def render_events_on_qpainter(
    event_tracker, painter: QtGui.QPainter, rect: QtCore.QRect
):
    options1 = ChartOptions()
    layout = ChartLayout(rect, options1)
    log_renderer = EventRenderer(painter, event_tracker, layout)
    log_renderer.render()


class EventRenderer(BaseRenderer):
    def __init__(self, painter: QtGui.QPainter, event_tracker, layout):
        super().__init__(painter, layout)
        self.event_tracker = event_tracker
        self.padding = 8

    def render(self):
        self.draw_bouding_rect()
        self.draw_axis()
        self.draw_events()

    def draw_axis(self):
        x_ticks = self.calc_x_ticks(self.event_tracker.x_axis)
        self.draw_x_axis(x_ticks)

    def draw_events(self):
        font = QtGui.QFont("courier", 6)
        # Font color:
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        self.painter.setFont(font)

        y = self.layout.chart_top + self.padding
        dy = self.painter.fontMetrics().height() + 3 * self.padding

        for event_track in self.event_tracker.tracks:
            with self.clip_chart_rect():
                self.draw_event_track(y, event_track)
                y += dy
            # Draw label on y axis:
            legend_x = self.layout.chart_right + 5
            legend_y = y
            self.painter.drawText(legend_x, legend_y, event_track.name)

    def draw_event_track(self, y, event_track):
        timespan = self.event_tracker.x_axis.get_timespan()
        # Determine how many data points we wish to visualize
        # This greatly determines drawing performance.
        min_count = int(self.layout.chart_width / 100)
        data = event_track.query(timespan, min_count)
        if data:
            if isinstance(data[0], Aggregation):
                self.draw_event_bundles(y, data)
            else:
                self.draw_single_events(y, data)

    def draw_single_events(self, y, stamped_records):
        """ Draw individual events.
        """
        for timestamp, event in stamped_records:
            x = self.to_x_pixel(timestamp)
            event_text = str(event)

            text_rect = self.painter.fontMetrics().boundingRect(event_text)

            width = text_rect.width() + self.padding * 2
            height = text_rect.height() + self.padding * 2
            event_rect = QtCore.QRect(x, y, width, height)

            # Draw background:
            fill_color = Qt.red
            self.painter.fillRect(event_rect, fill_color)
            self.painter.drawRect(event_rect)

            # Draw text:
            text_x = x - text_rect.x() + self.padding
            text_y = y - text_rect.y() + self.padding
            self.painter.drawText(text_x, text_y, event_text)

    def draw_event_bundles(self, y, aggregates):
        """ Draw event aggregations.
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
        return transform.to_x_pixel(value, self.event_tracker.x_axis, self.layout)
