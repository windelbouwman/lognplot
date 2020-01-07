from ..qtapi import QtGui, QtCore, Qt
from ...chart import Axis, Chart, LegendMode, Curve
from ...tsdb import Aggregation
from .layout import ChartLayout
from .base import BaseRenderer

class LegendRenderer(BaseRenderer):
    """ Not sure this should derive from BaseRenderer,
        or BaseRenderer is doing way too much...
    """

    def __init__(
        self, painter: QtGui.QPainter, chart: Chart, layout: ChartLayout, options
    ):
        super().__init__(painter, layout)
        self.chart = chart
        self.options = options

    def render(self):
        if self.options.show_legend:
            self._draw_legend()

    def _draw_legend(self):
        legend = self.layout.legend
        curves = self.chart.curves
 
        segment_width = legend.width() / len(curves)

        x = legend.left()
        for curve in curves:
            indicator = QtGui.QPainterPath(QtCore.QPointF(x, legend.top()))
            indicator.lineTo(QtCore.QPointF(x + legend.height(), legend.top()))
            indicator.lineTo(QtCore.QPointF(x + legend.height(), legend.bottom()))
            indicator.lineTo(QtCore.QPointF(x, legend.bottom()))

            self.painter.fillPath(indicator, QtGui.QBrush(QtGui.QColor(curve.color)))

            if curve != self.chart.activeCurve:
                labelArea = QtGui.QPainterPath(QtCore.QPointF(x + legend.height(), legend.top()))
                labelArea.lineTo(QtCore.QPointF(x + segment_width, legend.top()))
                labelArea.lineTo(QtCore.QPointF(x + segment_width, legend.bottom()))
                labelArea.lineTo(QtCore.QPointF(x + legend.height(), legend.bottom()))
                self.painter.fillPath(labelArea, QtGui.QBrush(Qt.lightGray))

            curve.legend_segment = [
                QtCore.QPointF(x, legend.top()),
                QtCore.QPointF(x + segment_width, legend.top()),
                QtCore.QPointF(x + segment_width, legend.bottom()),
                QtCore.QPointF(x, legend.bottom()),
            ]

            polygon = QtGui.QPainterPath(curve.legend_segment[0])
            for p in curve.legend_segment[1:]:
                polygon.lineTo(p)
            polygon.lineTo(curve.legend_segment[0])
            polygon.lineTo(QtCore.QPointF(x + legend.height(), legend.top()))
            polygon.lineTo(QtCore.QPointF(x + legend.height(), legend.bottom()))

            pen = QtGui.QPen(Qt.black)
            pen.setWidth(2)
            self.painter.strokePath(polygon, pen)

            if self.chart.legend.mode == LegendMode.SIGNAL_NAMES:
                self._draw_signal_names(x, curve)
            elif self.chart.legend.mode == LegendMode.CURSOR_VALUES:
                self._draw_cursor_values(x, curve)
            elif self.chart.legend.mode == LegendMode.Y_AXIS_SCALE:
                self._draw_y_axis_scale(x, curve)

            x += segment_width

    def _draw_text(self, x, curve: Curve, text):
        legend = self.layout.legend
        font_metrics = self.painter.fontMetrics()

        text_rect = font_metrics.boundingRect(text)
        text_x = curve.legend_segment[0].x() + legend.height() + 5 - text_rect.x()
        text_y = curve.legend_segment[0].y() + legend.height() / 2 - text_rect.y() - text_rect.height() / 2
        self.painter.setPen(Qt.black)
        self.painter.drawText(text_x, text_y, text)

    def _draw_cursor_values(self, x, curve: Curve):
        if self.chart.cursor:
            curve_point = curve.query_value(self.chart.cursor)
            if not curve_point:
                return
            _, curve_point_value = curve_point

            text = format(curve_point_value, '.08g')
            self._draw_text(x, curve, text)

    def _draw_signal_names(self, x, curve: Curve):
        self._draw_text(x, curve, curve.name)

    def _draw_y_axis_scale(self, x, curve: Curve):
        ticks = self.calc_y_ticks(curve.axis)
        valperdiv = ticks[1][0] - ticks[0][0]

        self._draw_text(x, curve, '{} / div'.format(valperdiv))
