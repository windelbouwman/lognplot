""" Scope on render times of the widget itself!
"""

import time
from lognplot.qt.qtapi import QtWidgets
from lognplot.tsdb import TsDb
from lognplot.qt.widgets import ChartWidget


def main():
    app = QtWidgets.QApplication([])
    w = AutoScopeGraphWidget()
    w.show()
    app.exec()


class BenchmarkedChartWidget(ChartWidget):
    def __init__(self, db, callback):
        super().__init__(db)
        self._callback = callback

    def paintEvent(self, e):
        t1 = time.time()
        super().paintEvent(e)
        t2 = time.time()
        duration = t2 - t1
        self._callback(t1, duration)


class AutoScopeGraphWidget(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        self._db = TsDb()

        def append_measurement(t, duration):
            self._db.add_sample("S1", (t, duration))

        self._chart_widget = BenchmarkedChartWidget(self._db, append_measurement)
        self._chart_widget.add_curve("S1", "red")

        l = QtWidgets.QVBoxLayout()
        l.addWidget(self._chart_widget)
        self.setLayout(l)

        self.resize(600, 400)


if __name__ == "__main__":
    main()
