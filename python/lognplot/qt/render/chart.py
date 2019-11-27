import contextlib
from PyQt5.QtGui import QPainter, QPen, QPolygon, QBrush, QColor
from PyQt5.QtCore import QRect, Qt, QPoint
from ...chart import Chart
from ...utils import bench_it, clip
from ...tsdb.metrics import Metrics
from .layout import ChartLayout


class ChartRenderer:
    """ This class can render a chart onto a painter.
    """

    def __init__(self, painter: QPainter, rect: QRect, chart: Chart, options):
        self.painter = painter
        # self._rect = rect
        self.chart = chart
        self.options = options
        self._layout = ChartLayout(rect, self.options)

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

    def shade_region(self, region):
        """ Draw a shaded box in some region.

        This is handy for the minimap.
        """
        color = QColor(Qt.gray)
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
        pen = QPen(Qt.black)
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
        pen = QPen(Qt.gray)
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
        pen = QPen(Qt.black)
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
        pen = QPen(Qt.black)
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
        timespan = (begin, end)
        # Determine how many data points we wish to visualize
        # This greatly determines drawing performance.
        min_count = int(self._layout.chart_width / 40)
        data = curve.query(timespan, min_count)
        # print("query result", type(data), len(data))
        curve_color = QColor(curve.color)

        if data:
            if isinstance(data[0], Metrics):
                # self._draw_metrics_as_blocks(data)
                self._draw_metrics_as_shape(data, curve_color)
            else:
                self._draw_samples_as_lines(data, curve_color)

    def _draw_samples_as_lines(self, samples, curve_color: QColor):
        """ Draw raw samples as lines! """
        pen = QPen(curve_color)
        pen.setWidth(2)
        self.painter.setPen(pen)
        points = [
            QPoint(self._to_x_pixel(x), self._to_y_pixel(y)) for (x, y) in samples
        ]
        line = QPolygon(points)
        self.painter.drawPolyline(line)

        # Draw markers:
        for point in points:
            rect = QRect(point.x() - 3, point.y() - 3, 6, 6)
            self.painter.drawEllipse(rect)

    def _draw_metrics_as_blocks(self, metrics):
        """ Draw aggregates as gray blocks.

        This is alternative 1 to draw metrics.
        """
        brush = QBrush(Qt.lightGray)
        # Draw a series of min/max rectangles.
        for compaction in metrics:
            x1 = self._to_x_pixel(compaction.x1)
            y1 = self._to_y_pixel(compaction.maximum)
            x2 = self._to_x_pixel(compaction.x2)
            y2 = self._to_y_pixel(compaction.minimum)
            height = max(y2 - y1, 1)  # minimal 1 pixel
            width = max(x2 - x1, 1)  # minimal 1 pixel
            # print(f'{width}x{height}')
            min_max_rect = QRect(x1, y1, width, height)
            self.painter.fillRect(min_max_rect, brush)

    def _draw_metrics_as_shape(self, metrics, curve_color: QColor):
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
        for metric in metrics:
            x1 = self._to_x_pixel(metric.x1)
            # x2 = self._to_x_pixel(metric.x2)

            # max line:
            y_max = self._to_y_pixel(metric.maximum)
            max_points.append(QPoint(x1, y_max))
            # max_points.append(QPoint(x2, y_max))

            # min line:
            y_min = self._to_y_pixel(metric.minimum)
            min_points.append(QPoint(x1, y_min))
            # min_points.append(QPoint(x2, y_min))

            mean = metric.mean
            stddev = metric.stddev

            # Mean line:
            y_mean = self._to_y_pixel(mean)
            mean_points.append(QPoint(x1, y_mean))
            # mean_points.append(QPoint(x2, y_mean))

            # stddev up line:
            y_stddev_up = self._to_y_pixel(mean + stddev)
            stddev_up_points.append(QPoint(x1, y_stddev_up))
            # stddev_up_points.append(QPoint(x2, y_stddev_up))

            # stddev down line:
            y_stddev_down = self._to_y_pixel(mean - stddev)
            stddev_down_points.append(QPoint(x1, y_stddev_down))
            # stddev_down_points.append(QPoint(x2, y_stddev_down))

        # Create contours:
        min_max_points = max_points + list(reversed(min_points))
        stddev_points = stddev_up_points + list(reversed(stddev_down_points))

        # Determine colors:
        stddev_color = QColor(curve_color)
        stddev_color.setAlphaF(0.3)
        min_max_color = QColor(curve_color)
        min_max_color.setAlphaF(0.1)

        self.painter.setPen(Qt.NoPen)

        # Min/max shape:
        min_max_shape = QPolygon(min_max_points)
        brush = QBrush(min_max_color)
        self.painter.setBrush(brush)
        self.painter.drawPolygon(min_max_shape)

        # stddev shape:
        stddev_shape = QPolygon(stddev_points)
        brush = QBrush(stddev_color)
        self.painter.setBrush(brush)
        self.painter.drawPolygon(stddev_shape)

        # Mean line:
        pen = QPen(curve_color)
        pen.setWidth(2)
        self.painter.setPen(pen)
        mean_line = QPolygon(mean_points)
        self.painter.drawPolyline(mean_line)

        # min/max lines:
        if False:
            # This is a bit ugly:
            pen = QPen(curve_color)
            pen.setWidth(1)
            self.painter.setPen(pen)
            max_line = QPolygon(max_points)
            self.painter.drawPolyline(max_line)
            min_line = QPolygon(min_points)
            self.painter.drawPolyline(min_line)

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
