""" Basic demo of the different widgets available.
"""

import math
from lognplot.qt.qtapi import QtWidgets
from lognplot.tsdb import TsDb
from lognplot.utils import bench_it
from lognplot.qt.widgets import ChartWidget, LogBarWidget
from lognplot.callstackbar import CallStackBar
from lognplot.demo_data import create_demo_samples


def main():
    app = QtWidgets.QApplication([])
    # w1 = demo_graph_widget()
    w2 = demo_log_widget()
    # w = CallStackWidget()
    # w1.show()
    w2.show()
    app.exec()


def demo_log_widget():
    db = TsDb()
    log_widget = LogBarWidget(db)
    from lognplot.tsdb import LogRecord, LogLevel

    log_widget.log_bar.add_track("T1")
    log_widget.log_bar.add_track("T2")
    log_widget.log_bar.add_track("T3")

    for i in range(20):
        db.add_sample("T1", (i * 29, LogRecord(LogLevel.ERROR, f"Woei {i}")))
        db.add_sample("T2", (i * 17, LogRecord(LogLevel.ERROR, f"Blarf {i}")))
        for j in range(100):
            db.add_sample(
                "T3", (i * 5 + j / 1000, LogRecord(LogLevel.ERROR, f"FREQWENT {i}"))
            )

    return log_widget


def demo_graph_widget():
    db = TsDb()
    chart_widget = ChartWidget(db)
    chart_widget.resize(600, 400)

    # num_points = 1_000_000
    num_points = 100_000

    with bench_it(f"create {num_points} demo samples"):
        samples = create_demo_samples(num_points)

    with bench_it(f"create zoom series with {len(samples)} samples"):
        db.add_samples("S1", samples)

    chart_widget.add_curve("S1", "blue")
    db.add_samples("S2", create_demo_samples(5000, 50))
    chart_widget.add_curve("S2", "green")
    print(chart_widget.chart.info())
    return chart_widget


if __name__ == "__main__":
    main()
