import math
from lognplot.qt.qtapi import QtWidgets
from lognplot.tsdb import TsDb
from lognplot.utils import bench_it
from lognplot.qt.widgets import ChartWidget
from lognplot.logbar import LogBar
from lognplot.callstackbar import CallStackBar
from lognplot.demo_data import create_demo_samples


def main():
    app = QtWidgets.QApplication([])
    w = DemoGraphWidget()
    # w = LogWidget()
    # w = CallStackWidget()
    w.show()
    app.exec()


class DemoGraphWidget(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        self._db = TsDb()
        self._chart_widget = ChartWidget(self._db)
        l = QtWidgets.QVBoxLayout()
        l.addWidget(self._chart_widget)
        self.setLayout(l)
        self.resize(600, 400)

        # num_points = 1_000_000
        num_points = 100_000

        with bench_it(f"create {num_points} demo samples"):
            samples = create_demo_samples(num_points)

        with bench_it(f"create zoom series with {len(samples)} samples"):
            self._db.add_samples("S1", samples)

        self._chart_widget.add_curve("S1", "blue")
        self._db.add_samples("S2", create_demo_samples(5000, 50))
        self._chart_widget.add_curve("S2", "green")
        print(self._chart_widget.chart.info())


if __name__ == "__main__":
    main()
