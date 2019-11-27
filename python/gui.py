import math
from PyQt5.QtWidgets import QApplication, QWidget, QVBoxLayout
from PyQt5.QtCore import Qt, QTimer
from PyQt5.QtGui import QPainter

from lognplot.tsdb import TsDb
from lognplot.utils import bench_it
from lognplot.qt.widgets import ChartWidget
from lognplot.logbar import LogBar
from lognplot.callstackbar import CallStackBar


def main():
    app = QApplication([])
    w = DemoGraphWidget()
    # w = LogWidget()
    # w = CallStackWidget()
    w.show()
    app.exec()


class DemoGraphWidget(QWidget):
    def __init__(self):
        super().__init__()
        self._db = TsDb()
        self._chart_widget = ChartWidget(self._db)
        l = QVBoxLayout()
        l.addWidget(self._chart_widget)
        self.setLayout(l)
        self.resize(600, 400)

        # num_points = 1_000_000
        num_points = 100_000

        with bench_it(f"create {num_points} demo samples"):
            samples = demo_samples(num_points)

        with bench_it("create zoom series"):
            self._db.add_samples("S1", samples)

        self._chart_widget.add_curve("S1", "blue")
        self._db.add_samples("S2", demo_samples(5000, 50))
        self._chart_widget.add_curve("S2", "green")
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
