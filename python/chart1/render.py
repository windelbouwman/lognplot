""" Render a graph on a QPainter. """
from PyQt5.QtGui import QPainter, QPen, QPolygon, QBrush
from PyQt5.QtCore import QRect, Qt, QPoint
from .chart import Chart
from .series import PointSerie, CompactedSerie
from .utils import bench_it


def render_chart_on_qpainter(chart: Chart, painter: QPainter, rect: QRect):
    renderer = Renderer(painter, chart)
    with bench_it("render"):
        renderer.render(rect)


class ChartLayout:
    def __init__(self, width, height):
        # Parameters:
        self.padding = 10
        self.axis_width = 40
        self.axis_height = 40

        # Inputs:
        self.width = width
        self.height = height

        # Endless sea of variables :)
        self.right = self.width
        self.bottom = self.height
        self.do_layout()

    def do_layout(self):
        self.chart_top = self.padding
        self.chart_left = self.padding
        self.chart_bottom = self.height - self.padding - self.axis_height
        self.chart_right = self.right - self.padding - self.axis_width
        self.chart_width = self.chart_right - self.chart_left
        self.chart_height = self.chart_bottom - self.chart_top


class Renderer:
    def __init__(self, painter: QPainter, chart: Chart):
        self.painter = painter
        self.chart = chart

    def render(self, rect: QRect):
        self._layout = ChartLayout(rect.width(), rect.height())

        self._draw_bouding_rect()

        min_ticks = 2
        x_tick_spacing = 80
        y_tick_spacing = 40
        amount_x_ticks = max(min_ticks, int(self._layout.chart_width // x_tick_spacing))
        amount_y_ticks = max(
            min_ticks, int(self._layout.chart_height // y_tick_spacing)
        )
        # print(f'min x ticks {amount_x_ticks} min y ticks {amount_y_ticks}')
        x_ticks = self.chart.x_axis.get_ticks(amount_x_ticks)
        y_ticks = self.chart.y_axis.get_ticks(amount_y_ticks)
        self._draw_grid(x_ticks, y_ticks)
        self._draw_x_axis(x_ticks)
        self._draw_y_axis(y_ticks)
        self._draw_series()

    def _draw_bouding_rect(self):
        pen = QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)

        self.painter.drawRect(
            self._layout.chart_left,
            self._layout.chart_top,
            self._layout.chart_width,
            self._layout.chart_height,
        )

    def _draw_grid(self, x_ticks, y_ticks):
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
        pen = QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        y = self._layout.chart_bottom + 5
        self.painter.drawLine(self._layout.chart_left, y, self._layout.chart_right, y)
        for value, label in x_ticks:
            x = self._to_x_pixel(value)
            self.painter.drawLine(x, y, x, y + 5)
            self.painter.drawText(x, y + 20, label)

    def _draw_y_axis(self, y_ticks):
        pen = QPen(Qt.black)
        pen.setWidth(2)
        self.painter.setPen(pen)
        x = self._layout.chart_right + 5
        self.painter.drawLine(x, self._layout.chart_top, x, self._layout.chart_bottom)
        for value, label in y_ticks:
            y = self._to_y_pixel(value)
            self.painter.drawLine(x, y, x + 5, y)
            self.painter.drawText(x + 10, y, label)

    def _draw_series(self):
        self.painter.setClipRect(
            self._layout.chart_left,
            self._layout.chart_top,
            self._layout.chart_width,
            self._layout.chart_height,
        )

        for serie in self.chart.series:
            with bench_it(f"render {serie}"):
                self._draw_serie(serie)

    def _draw_serie(self, serie):
        if isinstance(serie, PointSerie):
            self._draw_point_serie(serie)
        elif isinstance(serie, CompactedSerie):
            self._draw_compacted_serie(serie)
        else:  # pragma: no cover
            raise NotImplementedError(str(serie))

    def _draw_point_serie(self, serie):
        pen = QPen(Qt.red)
        pen.setWidth(2)
        self.painter.setPen(pen)
        points = [QPoint(self._to_x_pixel(x), self._to_y_pixel(y)) for (x, y) in serie]
        line = QPolygon(points)
        self.painter.drawPolyline(line)

    def _draw_compacted_serie(self, serie):
        """ Render a series which contain aggregates

        This means drawing the min/max values during a specific period.
        Also, mean/stddev/median might be involved here!
        """
        brush = QBrush(Qt.lightGray)
        # Draw a series of min/max rectangles.
        for compaction in serie:
            x1 = self._to_x_pixel(compaction.x1)
            y1 = self._to_y_pixel(compaction.maximum)
            x2 = self._to_x_pixel(compaction.x2)
            y2 = self._to_y_pixel(compaction.minimum)
            height = max(y2 - y1, 1)  # minimal 1 pixel
            width = max(x2 - x1, 1)  # minimal 1 pixel
            # print(f'{width}x{height}')
            min_max_rect = QRect(x1, y1, width, height)
            self.painter.fillRect(min_max_rect, brush)

    def _to_x_pixel(self, value):
        axis = self.chart.x_axis
        domain = axis.domain
        a = self._layout.chart_width / domain
        return self._layout.chart_left + a * (value - axis.minimum)

    def _to_y_pixel(self, value):
        axis = self.chart.y_axis
        domain = axis.domain
        a = self._layout.chart_height / domain
        return self._layout.chart_bottom - a * (value - axis.minimum)
