import math
from PyQt5.QtWidgets import QApplication, QWidget
from PyQt5.QtGui import QPainter
from chart1 import Chart, PointSerie
from chart1.render import render_chart_on_qpainter


def main():
    app = QApplication([])
    w = Wid()
    w.show()
    app.exec()


class Wid(QWidget):
    def __init__(self):
        super().__init__()
        self.chart = Chart()
        self.chart.y_axis.maximum = 80
        self.chart.y_axis.minimum = -30

        self.chart.x_axis.maximum = 104
        self.chart.x_axis.minimum = -2
        series1 = PointSerie()
        F = 1
        A = 25.0
        omega = math.pi * 2 * F
        F2 = 50
        A2 = 3.14
        omega2 = math.pi * 2 * F2
        for t in range(100_000):
            x = t * 0.001
            y = A * math.sin(omega * x) + A2 * math.cos(omega2 * x) + x
            series1.samples.append((x, y))
        serie2 = series1.create_compaction_series(44)
        self.chart.add_serie(serie2)
        # self.chart.add_serie(series1)
        print(self.chart.info())

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QPainter(self)
        # print("Paint!", self.rect())
        render_chart_on_qpainter(self.chart, painter, self.rect())


if __name__ == "__main__":
    main()
