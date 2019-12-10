from ..qtapi import QtGui, QtCore, Qt
from ...chart import Chart
from ...utils import bench_it
from ...tsdb import Aggregation
from .layout import ChartLayout
from . import transform
from .base import BaseRenderer


class ChartRenderer(BaseRenderer):
    """ This class can render a chart onto a painter.
    """

    def __init__(
        self, painter: QtGui.QPainter, chart: Chart, layout: ChartLayout, options
    ):
        super().__init__(painter, layout)
        self.chart = chart
        self.options = options

    def render(self):
        """ Main entry point to start rendering a graph. """

        self.draw_bouding_rect()

        x_ticks = self.calc_x_ticks(self.chart.x_axis)
        y_ticks = self.calc_y_ticks(self.chart.y_axis)

        if self.options.show_grid:
            self.draw_grid(x_ticks, y_ticks)

        if self.options.show_axis:
            self.draw_x_axis(x_ticks)
            self.draw_y_axis(y_ticks)

        self._draw_curves()
        self._draw_legend()

    def shade_region(self, region):
        """ Draw a shaded box in some region.

        This is handy for the minimap.
        """
        color = QtGui.QColor(Qt.gray)
        color.setAlphaF(0.7)
        # shade_brush = QBrush(color)
        # self.painter.setBrush(shade_brush)
        # print(region)
        x1 = self.to_x_pixel(region[0])
        y1 = self.to_y_pixel(region[1])
        x2 = self.to_x_pixel(region[2])
        y2 = self.to_y_pixel(region[3])
        width = max(x2 - x1, 5)
        height = max(y1 - y2, 5)
        # print(x1, y1, x2, y2)
        with self.clip_chart_rect():
            self.painter.fillRect(x1, y2, width, height, color)

    def _draw_curves(self):
        """ Draw all data enclosed in the chart. """
        with self.clip_chart_rect():
            for curve in self.chart.curves:
                # with bench_it(f"render {serie}"):
                self._draw_curve(curve)

    def _draw_curve(self, curve):
        """ Draw a single time series. """
        # with bench_it("series query"):
        timespan = self.chart.x_axis.get_timespan()
        # Determine how many data points we wish to visualize
        # This greatly determines drawing performance.
        min_count = int(self.layout.chart_width / 40)
        data = curve.query(timespan, min_count)
        # print("query result", type(data), len(data))
        curve_color = QtGui.QColor(curve.color)

        if data:
            if isinstance(data[0], Aggregation):
                self._draw_aggregations_as_shape(data, curve_color)
            else:
                self._draw_samples_as_lines(data, curve_color)

    def _draw_samples_as_lines(self, samples, curve_color: QtGui.QColor):
        """ Draw raw samples as lines! """
        pen = QtGui.QPen(curve_color)
        pen.setWidth(2)
        self.painter.setPen(pen)
        points = [
            QtCore.QPoint(self.to_x_pixel(x), self.to_y_pixel(y)) for (x, y) in samples
        ]
        line = QtGui.QPolygon(points)
        self.painter.drawPolyline(line)

        # Draw markers:
        for point in points:
            rect = QtCore.QRect(point.x() - 3, point.y() - 3, 6, 6)
            self.painter.drawEllipse(rect)

    def _draw_aggregations_as_shape(
        self, aggregations: Aggregation, curve_color: QtGui.QColor
    ):
        """ Draw aggregates as polygon shapes.

        This works by creating a contour around the min / max values.

        This is alternative 2 to draw metrics.
        """
        # Draw a series of min/max rectangles.
        mean_points = []
        min_points = []
        max_points = []
        stddev_up_points = []
        stddev_down_points = []

        # Forward sweep:
        for aggregation in aggregations:
            x1 = self.to_x_pixel(aggregation.timespan.central_timestamp())
            # x2 = self.to_x_pixel(metric.x2)

            # max line:
            y_max = self.to_y_pixel(aggregation.metrics.maximum)
            max_points.append(QtCore.QPoint(x1, y_max))
            # max_points.append(QtCore.QPoint(x2, y_max))

            # min line:
            y_min = self.to_y_pixel(aggregation.metrics.minimum)
            min_points.append(QtCore.QPoint(x1, y_min))
            # min_points.append(QtCore.QPoint(x2, y_min))

            mean = aggregation.metrics.mean
            stddev = aggregation.metrics.stddev

            # Mean line:
            y_mean = self.to_y_pixel(mean)
            mean_points.append(QtCore.QPoint(x1, y_mean))
            # mean_points.append(QtCore.QPoint(x2, y_mean))

            # stddev up line:
            y_stddev_up = self.to_y_pixel(mean + stddev)
            stddev_up_points.append(QtCore.QPoint(x1, y_stddev_up))
            # stddev_up_points.append(QtCore.QPoint(x2, y_stddev_up))

            # stddev down line:
            y_stddev_down = self.to_y_pixel(mean - stddev)
            stddev_down_points.append(QtCore.QPoint(x1, y_stddev_down))
            # stddev_down_points.append(QtCore.QPoint(x2, y_stddev_down))

        # Create contours:
        min_max_points = max_points + list(reversed(min_points))
        stddev_points = stddev_up_points + list(reversed(stddev_down_points))

        # Determine colors:
        stddev_color = QtGui.QColor(curve_color)
        stddev_color.setAlphaF(0.3)
        min_max_color = QtGui.QColor(curve_color)
        min_max_color.setAlphaF(0.1)

        self.painter.setPen(Qt.NoPen)

        # Min/max shape:
        min_max_shape = QtGui.QPolygon(min_max_points)
        brush = QtGui.QBrush(min_max_color)
        self.painter.setBrush(brush)
        self.painter.drawPolygon(min_max_shape)

        # stddev shape:
        stddev_shape = QtGui.QPolygon(stddev_points)
        brush = QtGui.QBrush(stddev_color)
        self.painter.setBrush(brush)
        self.painter.drawPolygon(stddev_shape)

        # Mean line:
        pen = QtGui.QPen(curve_color)
        pen.setWidth(2)
        self.painter.setPen(pen)
        mean_line = QtGui.QPolygon(mean_points)
        self.painter.drawPolyline(mean_line)

        # min/max lines:
        if False:
            # This is a bit ugly:
            pen = QtGui.QPen(curve_color)
            pen.setWidth(1)
            self.painter.setPen(pen)
            max_line = QtGui.QPolygon(max_points)
            self.painter.drawPolyline(max_line)
            min_line = QtGui.QPolygon(min_points)
            self.painter.drawPolyline(min_line)

    def _draw_legend(self):
        """ Draw names / color of the curve next to eachother.
        """
        font_metrics = self.painter.fontMetrics()
        x = self.layout.chart_left + 10
        y = self.layout.chart_top + 10
        text_height = font_metrics.height()
        color_block_size = text_height * 0.8
        for index, curve in enumerate(self.chart.curves):
            color = QtGui.QColor(curve.color)
            text = curve.name
            text_rect = font_metrics.boundingRect(text)
            legend_x = x
            legend_y = y + index * text_height
            text_x = legend_x + color_block_size + 3 - text_rect.x()
            text_y = legend_y - text_rect.y() - text_rect.height() / 2
            self.painter.drawText(text_x, text_y, text)
            self.painter.fillRect(
                x,
                legend_y - color_block_size / 2,
                color_block_size,
                color_block_size,
                color,
            )

    def to_x_pixel(self, value):
        return transform.to_x_pixel(value, self.chart.x_axis, self.layout)

    def to_y_pixel(self, value):
        return transform.to_y_pixel(value, self.chart.y_axis, self.layout)

    def x_pixel_to_domain(self, pixel):
        axis = self.chart.x_axis
        domain = axis.domain
        # a = self.
