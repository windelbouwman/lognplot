import contextlib
from ..qtapi import QtGui, QtCore, Qt
from ...chart import Chart
from ...utils import bench_it, clip
from ...tsdb import Metrics, Aggregation
from ...time import TimeSpan
from .layout import ChartLayout


class ChartRenderer:
    """ This class can render a chart onto a painter.
    """

    def __init__(
        self, painter: QtGui.QPainter, chart: Chart, layout: ChartLayout, options
    ):
        self.painter = painter
        self.chart = chart
        self.options = options
        self._layout = layout

    def render(self):
        """ Main entry point to start rendering a graph. """

        self._draw_bouding_rect()

        min_ticks = 2
        x_tick_spacing = 100
        y_tick_spacing = 50
        amount_x_ticks = max(min_ticks, int(self._layout.chart_width // x_tick_spacing))
        amount_y_ticks = max(
            min_ticks, int(self._layout.chart_height // y_tick_spacing)
        )
        # print(f'min x ticks {amount_x_ticks} min y ticks {amount_y_ticks}')
        x_ticks = self.chart.x_axis.get_ticks(amount_x_ticks)
        y_ticks = self.chart.y_axis.get_ticks(amount_y_ticks)

        if self.options.show_grid:
            self._draw_grid(x_ticks, y_ticks)

        if self.options.show_axis:
            self._draw_x_axis(x_ticks)
            self._draw_y_axis(y_ticks)

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
        x1 = self._to_x_pixel(region[0])
        y1 = self._to_y_pixel(region[1])
        x2 = self._to_x_pixel(region[2])
        y2 = self._to_y_pixel(region[3])
        width = max(x2 - x1, 5)
        height = max(y1 - y2, 5)
        # print(x1, y1, x2, y2)
        with self.clip_chart_rect():
            self.painter.fillRect(x1, y2, width, height, color)

    def _draw_bouding_rect(self):
        """ Draw a bounding box around the plotting area. """
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        self.painter.setBrush(Qt.NoBrush)

        self.painter.drawRect(
            self._layout.chart_left,
            self._layout.chart_top,
            self._layout.chart_width,
            self._layout.chart_height,
        )

    def _draw_grid(self, x_ticks, y_ticks):
        """ Render a grid on the given x and y tick markers. """
        pen = QtGui.QPen(Qt.gray)
        pen.setWidth(1)
        self.painter.setPen(pen)

        for value, _ in x_ticks:
            x = self._to_x_pixel(value)
            self.painter.drawLine(
                x, self._layout.chart_top, x, self._layout.chart_bottom
            )

        for value, _ in y_ticks:
            y = self._to_y_pixel(value)
            self.painter.drawLine(
                self._layout.chart_left, y, self._layout.chart_right, y
            )

    def _draw_x_axis(self, x_ticks):
        """ Draw the X-axis. """
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        margin = 5
        y = self._layout.chart_bottom + margin
        self.painter.drawLine(self._layout.chart_left, y, self._layout.chart_right, y)
        for value, label in x_ticks:
            x = self._to_x_pixel(value)

            # Tick handle:
            self.painter.drawLine(x, y, x, y + 5)

            # Tick label:
            text_rect = self.painter.fontMetrics().boundingRect(label)
            text_x = x - text_rect.x() - text_rect.width() / 2
            text_y = y + 10 - text_rect.y()
            self.painter.drawText(text_x, text_y, label)

    def _draw_y_axis(self, y_ticks):
        """ Draw the Y-axis. """
        pen = QtGui.QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        x = self._layout.chart_right + 5
        self.painter.drawLine(x, self._layout.chart_top, x, self._layout.chart_bottom)
        for value, label in y_ticks:
            y = self._to_y_pixel(value)

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
            self._layout.chart_left,
            self._layout.chart_top,
            self._layout.chart_width,
            self._layout.chart_height,
        )
        yield
        # Remove the clipping rectangle:
        self.painter.restore()

    def _draw_curves(self):
        """ Draw all data enclosed in the chart. """
        with self.clip_chart_rect():
            for curve in self.chart.curves:
                # with bench_it(f"render {serie}"):
                self._draw_curve(curve)

    def _draw_curve(self, curve):
        """ Draw a single time series. """
        # with bench_it("series query"):
        begin = self.chart.x_axis.minimum
        end = self.chart.x_axis.maximum
        assert begin <= end
        timespan = TimeSpan(begin, end)
        # Determine how many data points we wish to visualize
        # This greatly determines drawing performance.
        min_count = int(self._layout.chart_width / 40)
        data = curve.query(timespan, min_count)
        # print("query result", type(data), len(data))
        curve_color = QtGui.QColor(curve.color)

        if data:
            if isinstance(data[0], Aggregation):
                # self._draw_metrics_as_blocks(data)
                self._draw_aggregations_as_shape(data, curve_color)
            else:
                self._draw_samples_as_lines(data, curve_color)

    def _draw_samples_as_lines(self, samples, curve_color: QtGui.QColor):
        """ Draw raw samples as lines! """
        pen = QtGui.QPen(curve_color)
        pen.setWidth(2)
        self.painter.setPen(pen)
        points = [
            QtCore.QPoint(self._to_x_pixel(x), self._to_y_pixel(y))
            for (x, y) in samples
        ]
        line = QtGui.QPolygon(points)
        self.painter.drawPolyline(line)

        # Draw markers:
        for point in points:
            rect = QtCore.QRect(point.x() - 3, point.y() - 3, 6, 6)
            self.painter.drawEllipse(rect)

    def _draw_metrics_as_blocks(self, metrics):
        """ Draw aggregates as gray blocks.

        This is alternative 1 to draw metrics.
        """
        brush = QtGui.QBrush(Qt.lightGray)
        # Draw a series of min/max rectangles.
        for compaction in metrics:
            x1 = self._to_x_pixel(compaction.x1)
            y1 = self._to_y_pixel(compaction.maximum)
            x2 = self._to_x_pixel(compaction.x2)
            y2 = self._to_y_pixel(compaction.minimum)
            height = max(y2 - y1, 1)  # minimal 1 pixel
            width = max(x2 - x1, 1)  # minimal 1 pixel
            # print(f'{width}x{height}')
            min_max_rect = QtCore.QRect(x1, y1, width, height)
            self.painter.fillRect(min_max_rect, brush)

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
            x1 = self._to_x_pixel(aggregation.timespan.central_timestamp())
            # x2 = self._to_x_pixel(metric.x2)

            # max line:
            y_max = self._to_y_pixel(aggregation.metrics.maximum)
            max_points.append(QtCore.QPoint(x1, y_max))
            # max_points.append(QtCore.QPoint(x2, y_max))

            # min line:
            y_min = self._to_y_pixel(aggregation.metrics.minimum)
            min_points.append(QtCore.QPoint(x1, y_min))
            # min_points.append(QtCore.QPoint(x2, y_min))

            mean = aggregation.metrics.mean
            stddev = aggregation.metrics.stddev

            # Mean line:
            y_mean = self._to_y_pixel(mean)
            mean_points.append(QtCore.QPoint(x1, y_mean))
            # mean_points.append(QtCore.QPoint(x2, y_mean))

            # stddev up line:
            y_stddev_up = self._to_y_pixel(mean + stddev)
            stddev_up_points.append(QtCore.QPoint(x1, y_stddev_up))
            # stddev_up_points.append(QtCore.QPoint(x2, y_stddev_up))

            # stddev down line:
            y_stddev_down = self._to_y_pixel(mean - stddev)
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
        x = self._layout.chart_left + 10
        y = self._layout.chart_top + 10
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

    def _to_x_pixel(self, value):
        """ Transform the given X value to a pixel position.
        """
        axis = self.chart.x_axis
        domain = axis.domain
        a = self._layout.chart_width / domain
        x = self._layout.chart_left + a * (value - axis.minimum)
        return clip(x, self._layout.chart_left, self._layout.chart_right)

    def _to_y_pixel(self, value):
        """ Transform the given Y value to a pixel position.
        """
        axis = self.chart.y_axis
        domain = axis.domain
        a = self._layout.chart_height / domain
        y = self._layout.chart_bottom - a * (value - axis.minimum)
        return clip(y, self._layout.chart_top, self._layout.chart_bottom)

    def x_pixel_to_domain(self, pixel):
        axis = self.chart.x_axis
        domain = axis.domain
        # a = self.
