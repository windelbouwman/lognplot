from ..qtapi import QtGui, QtCore, Qt
from ...chart import Axis, Chart
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

        x_ticks = self.calc_x_ticks(self.chart.x_axis)
        y_ticks = self.calc_y_ticks(self.chart.y_axis)

        if self.options.show_grid:
            self.draw_grid(self.chart.y_axis, x_ticks, y_ticks)

        self.draw_bouding_rect()

        if self.options.show_axis:
            self.draw_x_axis(x_ticks)
            self.draw_y_axis(self.chart.y_axis, y_ticks)

        if self.options.show_bar:
            self._draw_bar()

        if self.options.show_handles:
            self._draw_handles()

        self._draw_curves()

        if self.options.show_legend:
            self._draw_legend()
        self._draw_cursor()

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
                self._draw_aggregations_as_shape(curve.axis, data, curve_color)
            else:
                self._draw_samples_as_lines(curve.axis, data, curve_color)

    def _draw_samples_as_lines(self, y_axis: Axis, samples, curve_color: QtGui.QColor):
        """ Draw raw samples as lines! """
        pen = QtGui.QPen(curve_color)
        pen.setWidth(2)
        self.painter.setPen(pen)
        points = [
            QtCore.QPoint(self.to_x_pixel(x), self.to_y_pixel(y_axis, y)) for (x, y) in samples
        ]
        line = QtGui.QPolygon(points)
        self.painter.drawPolyline(line)

        # Draw markers:
        for point in points:
            rect = QtCore.QRect(point.x() - 3, point.y() - 3, 6, 6)
            self.painter.drawEllipse(rect)

    def _draw_aggregations_as_shape(
        self, y_axis: Axis, aggregations: Aggregation, curve_color: QtGui.QColor
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
            y_max = self.to_y_pixel(y_axis, aggregation.metrics.maximum)
            max_points.append(QtCore.QPoint(x1, y_max))
            # max_points.append(QtCore.QPoint(x2, y_max))

            # min line:
            y_min = self.to_y_pixel(y_axis, aggregation.metrics.minimum)
            min_points.append(QtCore.QPoint(x1, y_min))
            # min_points.append(QtCore.QPoint(x2, y_min))

            mean = aggregation.metrics.mean
            stddev = aggregation.metrics.stddev

            # Mean line:
            y_mean = self.to_y_pixel(y_axis, mean)
            mean_points.append(QtCore.QPoint(x1, y_mean))
            # mean_points.append(QtCore.QPoint(x2, y_mean))

            # stddev up line:
            y_stddev_up = self.to_y_pixel(y_axis, mean + stddev)
            stddev_up_points.append(QtCore.QPoint(x1, y_stddev_up))
            # stddev_up_points.append(QtCore.QPoint(x2, y_stddev_up))

            # stddev down line:
            y_stddev_down = self.to_y_pixel(y_axis, mean - stddev)
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

    def _draw_cursor(self):
        if self.chart.cursor:
            # Draw cursor line:
            x = self.to_x_pixel(self.chart.cursor)
            pen = QtGui.QPen(Qt.black)
            pen.setWidth(1)
            self.painter.setPen(pen)
            self.painter.drawLine(x, self.layout.chart_top, x, self.layout.chart_bottom)

            # Draw values of signals at position:
            font_metrics = self.painter.fontMetrics()
            legend_x = x + 10
            y = self.layout.chart_top + 10
            text_height = font_metrics.height()
            color_block_size = text_height * 0.8
            for index, curve in enumerate(self.chart.curves):
                color = QtGui.QColor(curve.color)
                curve_point = curve.query_value(self.chart.cursor)
                if not curve_point:
                    continue
                curve_point_timestamp, curve_point_value = curve_point

                # Draw circle indicator around selected point:
                pen = QtGui.QPen(color)
                pen.setWidth(2)
                self.painter.setPen(pen)
                marker_x = self.to_x_pixel(curve_point_timestamp)
                marker_y = self.to_y_pixel(curve.axis, curve_point_value)
                marker_size = 10
                indicator_rect = QtCore.QRect(
                    marker_x - marker_size // 2,
                    marker_y - marker_size // 2,
                    marker_size,
                    marker_size,
                )
                self.painter.drawEllipse(indicator_rect)

                # Legend:
                if self.options.show_cursor_legend:
                    text = "{} = {}".format(curve.name, curve_point_value)
                    text_rect = font_metrics.boundingRect(text)
                    # legend_y = y + index * text_height
                    legend_x = marker_x + 10
                    legend_y = marker_y
                    text_x = legend_x + color_block_size + 3 - text_rect.x()
                    text_y = legend_y - text_rect.y() - text_rect.height() / 2
                    self.painter.drawText(text_x, text_y, text)
                    self.painter.fillRect(
                        legend_x,
                        legend_y - color_block_size / 2,
                        color_block_size,
                        color_block_size,
                        color,
                    )

    def _draw_handles(self):
        x = self.layout.handles.left()

        for _, curve in enumerate(self.chart.curves):
            handle_y = self.to_y_pixel(curve.axis, 0)
            x_full = self.options.handle_width
            x_half = x_full / 2
            y_half = self.options.handle_height / 2

            curve.handle = [
                QtCore.QPointF(x, handle_y - y_half),
                QtCore.QPointF(x, handle_y - y_half),
                QtCore.QPointF(x + x_half, handle_y - y_half),
                QtCore.QPointF(x + x_full, handle_y),
                QtCore.QPointF(x + x_half, handle_y + y_half),
                QtCore.QPointF(x, handle_y + y_half)
            ] 

            polygon = QtGui.QPainterPath(curve.handle[0])
            for p in curve.handle[1:]:
                polygon.lineTo(p)

            color = QtGui.QColor(curve.color)
            self.painter.fillPath(polygon, QtGui.QBrush(color))

    def _draw_bar(self):
        bar = self.layout.bar
        curves = self.chart.curves
        font_metrics = self.painter.fontMetrics()

        segment_width = bar.width() / len(curves)

        x = bar.left()
        for curve in curves:
            indicator = QtGui.QPainterPath(QtCore.QPointF(x, bar.top()))
            indicator.lineTo(QtCore.QPointF(x + bar.height(), bar.top()))
            indicator.lineTo(QtCore.QPointF(x + bar.height(), bar.bottom()))
            indicator.lineTo(QtCore.QPointF(x, bar.bottom()))

            self.painter.fillPath(indicator, QtGui.QBrush(QtGui.QColor(curve.color)))

            if curve != self.chart.activeCurve:
                legend = QtGui.QPainterPath(QtCore.QPointF(x + bar.height(), bar.top()))
                legend.lineTo(QtCore.QPointF(x + segment_width, bar.top()))
                legend.lineTo(QtCore.QPointF(x + segment_width, bar.bottom()))
                legend.lineTo(QtCore.QPointF(x + bar.height(), bar.bottom()))
                self.painter.fillPath(legend, QtGui.QBrush(Qt.lightGray))

            curve.bar_segment = [
                QtCore.QPointF(x, bar.top()),
                QtCore.QPointF(x + segment_width, bar.top()),
                QtCore.QPointF(x + segment_width, bar.bottom()),
                QtCore.QPointF(x, bar.bottom()),
            ]

            polygon = QtGui.QPainterPath(curve.bar_segment[0])
            for p in curve.bar_segment[1:]:
                polygon.lineTo(p)
            polygon.lineTo(curve.bar_segment[0])
            polygon.lineTo(QtCore.QPointF(x + bar.height(), bar.top()))
            polygon.lineTo(QtCore.QPointF(x + bar.height(), bar.bottom()))

            pen = QtGui.QPen(Qt.black)
            pen.setWidth(2)
            self.painter.strokePath(polygon, pen)
 
            # Legend:
            if self.chart.cursor:
                curve_point = curve.query_value(self.chart.cursor)
                if not curve_point:
                    continue
                _, curve_point_value = curve_point

                text = format(curve_point_value, '.08g')
                text_rect = font_metrics.boundingRect(text)
                # legend_y = y + index * text_height
                text_x = curve.bar_segment[0].x() + bar.height() + 3 - text_rect.x()
                text_y = curve.bar_segment[0].y() + bar.height() / 2 - text_rect.y() - text_rect.height() / 2
                self.painter.drawText(text_x, text_y, text)
            
            x += segment_width

    def to_x_pixel(self, value):
        return transform.to_x_pixel(value, self.chart.x_axis, self.layout)

    def to_y_pixel(self, y_axis, value):
        return transform.to_y_pixel(value, y_axis, self.layout)

    def x_pixel_to_domain(self, pixel):
        axis = self.chart.x_axis
        domain = axis.domain
        # a = self.
