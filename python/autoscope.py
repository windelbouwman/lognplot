""" Scope on render times of the widget itself!
"""

import time
from PyQt5.QtWidgets import QApplication, QWidget, QVBoxLayout
from lognplot import Chart, ZoomSerie
from lognplot.qt.widgets.chartwidget import ChartWidget


def main():
    app = QApplication([])
    w = AutoScopeGraphWidget()
    w.show()
    app.exec()


class BenchmarkedChartWidget(ChartWidget):
    def __init__(self, callback):
        super().__init__()
        self._callback = callback

    def paintEvent(self, e):
        t1 = time.time()
        super().paintEvent(e)
        t2 = time.time()
        duration = t2 - t1
        self._callback(t1, duration)


class AutoScopeGraphWidget(QWidget):
    def __init__(self):
        super().__init__()

        self.series1 = ZoomSerie()

        def append_measurement(t, duration):
            self.series1.add_sample((t, duration))

        self._chart_widget = BenchmarkedChartWidget(append_measurement)
        self._chart_widget.chart.add_serie(self.series1)

        l = QVBoxLayout()
        l.addWidget(self._chart_widget)
        self.setLayout(l)

        self.resize(600, 400)


if __name__ == "__main__":
    main()
