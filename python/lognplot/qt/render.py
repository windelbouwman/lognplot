""" Render a graph on a QPainter. """
import contextlib
from PyQt5.QtGui import QPainter, QPen, QPolygon, QBrush, QColor
from PyQt5.QtCore import QRect, Qt, QPoint
from ..chart import Chart
from ..series import PointSerie, CompactedSerie, ZoomSerie
from ..utils import bench_it, clip
from ..metrics import Metrics


def render_chart_on_qpainter(chart: Chart, painter: QPainter, rect: QRect):
    """ Call this function to paint a chart onto the given painter within the rectangle specified.
    """
    renderer = Renderer(painter, chart)
    # with bench_it("render"):
    renderer.render(rect)


class ChartLayout:
    def __init__(self, rect: QRect, options):
        # Parameters:
        self.axis_width = 40
        self.axis_height = 40

        # Inputs:
        self.options = options
        # print(rect, type(rect))
        self.rect = rect

        # Endless sea of variables :)
        self.do_layout()

    def do_layout(self):
        # self.right = self.rect.right()
        # self.bottom = self.rect.bottom()
        self.chart_top = self.rect.top() + self.options.padding
        self.chart_left = self.rect.left() + self.options.padding
        if self.options.show_axis:
            axis_height = self.axis_height
            axis_width = self.axis_width
        else:
            axis_height = 0
            axis_width = 0

        self.chart_bottom = self.rect.bottom() - self.options.padding - axis_height
        self.chart_right = self.rect.right() - self.options.padding - axis_width
        self.chart_width = self.chart_right - self.chart_left
        self.chart_height = self.chart_bottom - self.chart_top


class ChartOptions:
    def __init__(self):
        self.show_axis = True
        self.show_grid = True
        self.padding = 10


class Renderer:
    """ Render a chart.

    Optionally include a minimap?
    """

    def __init__(self, painter: QPainter, chart: Chart):
        self.painter = painter
        self.chart = chart

    def render(self, rect: QRect):
        options1 = ChartOptions()
        chart_renderer = ChartRenderer(self.painter, rect, self.chart, options1)
        chart_renderer.render()

        # Create a new chart with the whole thing zoomed
        minimap_chart = Chart()
        for serie in self.chart.series:
            minimap_chart.add_serie(serie)
        minimap_chart.zoom_fit()

        # Now render minimap in top left corner.
        minimap_options = ChartOptions()
        minimap_options.padding = 2
        minimap_options.show_axis = False
        minimap_rect = QRect(rect.x() + 40, rect.y() + 40, 120, 80)
        self.painter.fillRect(minimap_rect, Qt.yellow)
        minimap_chart_renderer = ChartRenderer(
            self.painter, minimap_rect, minimap_chart, minimap_options
        )
        minimap_chart_renderer.render()
        region = self.chart.get_region()
        minimap_chart_renderer.shade_region(region)


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

        self._draw_series()

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

    def _draw_series(self):
        """ Draw all data enclosed in the chart. """
        with self.clip_chart_rect():
            for serie in self.chart.series:
                # with bench_it(f"render {serie}"):
                self._draw_serie(serie)

    def _draw_serie(self, serie):
        """ Draw a single time series. """
        if isinstance(serie, PointSerie):
            self._draw_point_serie(serie)
        elif isinstance(serie, CompactedSerie):
            self._draw_compacted_serie(serie)
        elif isinstance(serie, ZoomSerie):
            self._draw_zoomed_serie(serie)
        else:  # pragma: no cover
            raise NotImplementedError(str(serie))

    def _draw_point_serie(self, serie):
        self._draw_samples_as_lines(serie)

    def _draw_compacted_serie(self, serie):
        """ Render a series which contain aggregates

        This means drawing the min/max values during a specific period.
        Also, mean/stddev/median might be involved here!
        """
        self._draw_metrics_as_blocks(serie)

    def _draw_zoomed_serie(self, serie):
        # with bench_it("series query"):
        begin = self.chart.x_axis.minimum
        end = self.chart.x_axis.maximum
        # Determine how many data points we wish to visualize
        # This greatly determines drawing performance.
        min_count = int(self._layout.chart_width / 40)
        data = serie.query(begin, end, min_count)
        # print("query result", type(data), len(data))

        if data:
            if isinstance(data[0], Metrics):
                # self._draw_metrics_as_blocks(data)
                self._draw_metrics_as_shape(data)
            else:
                self._draw_samples_as_lines(data)

    def _draw_samples_as_lines(self, samples):
        """ Draw raw samples as lines! """
        pen = QPen(Qt.red)
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

    def _draw_metrics_as_shape(self, metrics):
        """ Draw aggregates as polygon shapes.

        This works by creating a contour around the min / max values.

        This is alternative 2 to draw metrics.
        """
        # Draw a series of min/max rectangles.
        mean_points = []
        min_max_points = []
        stddev_points = []

        # Forward sweep:
        for metric in metrics:
            x1 = self._to_x_pixel(metric.x1)
            x2 = self._to_x_pixel(metric.x2)

            # min max contour:
            y_max = self._to_y_pixel(metric.maximum)
            min_max_points.append(QPoint(x1, y_max))
            min_max_points.append(QPoint(x2, y_max))

            # Mean line:
            y_mean = self._to_y_pixel(metric.mean)
            mean_points.append(QPoint(x1, y_mean))
            mean_points.append(QPoint(x2, y_mean))

            # stddev contour:
            y_stddev_up = self._to_y_pixel(metric.mean + metric.stddev)
            stddev_points.append(QPoint(x1, y_stddev_up))
            stddev_points.append(QPoint(x2, y_stddev_up))

        # Backwards sweep:
        for metric in reversed(metrics):
            x1 = self._to_x_pixel(metric.x1)
            x2 = self._to_x_pixel(metric.x2)

            # min max contour:
            y_min = self._to_y_pixel(metric.minimum)
            min_max_points.append(QPoint(x2, y_min))
            min_max_points.append(QPoint(x1, y_min))

            # stddev contour:
            y_stddev_down = self._to_y_pixel(metric.mean - metric.stddev)
            stddev_points.append(QPoint(x2, y_stddev_down))
            stddev_points.append(QPoint(x1, y_stddev_down))

        self.painter.setPen(Qt.NoPen)
        color = QColor(Qt.red)
        stddev_color = color.lighter(140)
        min_max_color = color.lighter(170)

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
        pen = QPen(color)
        pen.setWidth(2)
        self.painter.setPen(pen)
        mean_line = QPolygon(mean_points)
        self.painter.drawPolyline(mean_line)

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
