import math
from PyQt5.QtWidgets import QApplication, QWidget, QVBoxLayout
from PyQt5.QtCore import Qt, QTimer
from PyQt5.QtGui import QPainter
from chart1 import Chart, PointSerie, ZoomSerie

from chart1.qt.render_log import render_logs_on_qpainter
from chart1.utils import bench_it
from chart1.qt.widgets.chartwidget import ChartWidget
from chart1.logbar import LogBar
from chart1.callstackbar import CallStackBar


def main():
    app = QApplication([])
    w = DemoGraphWidget()
    # w = LogWidget()
    # w = CallStackWidget()
    w.show()
    app.exec()


class CallStackWidget(QWidget):
    """ Visualize a program callstack. """

    def __init__(self):
        super().__init__()
        self.call_stack = CallStackBar()

    def paintEvent(self, e):
        super().paintEvent(e)


class LogWidget(QWidget):
    """ Visualize log records in chronological order.
    """

    def __init__(self):
        super().__init__()
        self.logs = LogBar()

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QPainter(self)
        render_logs_on_qpainter(self.logs, painter, self.rect())


class DemoGraphWidget(QWidget):
    def __init__(self):
        super().__init__()
        self._chart_widget = ChartWidget()
        l = QVBoxLayout()
        l.addWidget(self._chart_widget)
        self.setLayout(l)
        self.resize(600, 400)

        # num_points = 1_000_000
        num_points = 10_000

        with bench_it(f"create {num_points} demo samples"):
            samples = demo_samples(num_points)

        series1 = PointSerie()
        with bench_it("create zoom series"):
            series1 = ZoomSerie()
            series1.add_samples(samples)

        self._chart_widget.chart.add_serie(series1)
        series4 = ZoomSerie()
        series4.add_samples(demo_samples(5000, 50))
        self._chart_widget.chart.add_serie(series4)
        print(self._chart_widget.chart.info())


def demo_samples(num_points, offset=0):
    """ Create sin wave with superposed cosine wave """
    # Parameters:
    F = 1
    A = 25.0
    omega = math.pi * 2 * F
    F2 = 50
    A2 = 3.14
    omega2 = math.pi * 2 * F2

    samples = []
    for t in range(num_points):
        x = t * 0.001
        y = offset + A * math.sin(omega * x) + A2 * math.cos(omega2 * x)
        samples.append((x, y))
    return samples


if __name__ == "__main__":
    main()
