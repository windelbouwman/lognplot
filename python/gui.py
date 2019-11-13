import math
from PyQt5.QtWidgets import QApplication, QWidget
from PyQt5.QtGui import QPainter
from PyQt5.QtCore import Qt, QTimer
from chart1 import Chart, PointSerie, ZoomSerie
from chart1.render import render_chart_on_qpainter
from chart1.utils import bench_it


def main():
    app = QApplication([])
    w = DemoGraphWidget()
    w.show()
    app.exec()


class DemoGraphWidget(QWidget):
    def __init__(self):
        super().__init__()
        self.pan_x_speed = 0
        self.chart = Chart()
        self.chart.y_axis.maximum = 80
        self.chart.y_axis.minimum = -30

        self.chart.x_axis.maximum = 104
        self.chart.x_axis.minimum = -2

        num_points = 1_000_000
        # num_points = 10_000

        with bench_it(f"create {num_points} demo samples"):
            samples = demo_samples(num_points)

        series1 = PointSerie()
        with bench_it("create zoom series"):
            series1 = ZoomSerie()
            series1.add_samples(samples)

        # serie2 = series1.create_compaction_series(244)
        # self.chart.add_serie(serie2)
        self.chart.add_serie(series1)
        print(self.chart.info())

        self._zoom_timer = QTimer()
        # self._zoom_timer.timeout.connect(self._on_timeout)
        # self._zoom_timer.start(100)

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QPainter(self)
        # print("Paint!", self.rect())
        render_chart_on_qpainter(self.chart, painter, self.rect())

    # def _on_timeout(self):
    #     if self.pan_x_speed:
    #         print('TO')

    #     # self.update()

    # Panning helpers:
    PAN_FACTOR = 0.05

    def pan_left(self):
        self.horizontal_pan(-self.PAN_FACTOR)

    def pan_right(self):
        self.horizontal_pan(self.PAN_FACTOR)

    def pan_up(self):
        self.vertical_pan(self.PAN_FACTOR)

    def pan_down(self):
        self.vertical_pan(-self.PAN_FACTOR)

    # Zooming helpers:
    ZOOM_FACTOR = 0.1

    def zoom_in_horizontal(self):
        self.horizontal_zoom(-self.ZOOM_FACTOR)

    def zoom_out_horizontal(self):
        self.horizontal_zoom(self.ZOOM_FACTOR)

    def zoom_in_vertical(self):
        self.vertical_zoom(self.ZOOM_FACTOR)

    def zoom_out_vertical(self):
        self.vertical_zoom(-self.ZOOM_FACTOR)

    def horizontal_zoom(self, amount):
        domain = self.chart.x_axis.domain
        step = domain * amount
        self.chart.x_axis.minimum -= step
        self.chart.x_axis.maximum += step
        self.update()

    def vertical_zoom(self, amount):
        domain = self.chart.y_axis.domain
        step = domain * amount
        self.chart.y_axis.minimum -= step
        self.chart.y_axis.maximum += step
        self.update()

    def horizontal_pan(self, amount):
        domain = self.chart.x_axis.domain
        step = domain * amount
        self.chart.x_axis.minimum += step
        self.chart.x_axis.maximum += step
        self.update()

    def vertical_pan(self, amount):
        domain = self.chart.y_axis.domain
        step = domain * amount
        self.chart.y_axis.minimum += step
        self.chart.y_axis.maximum += step
        self.update()

    def keyPressEvent(self, e):
        super().keyPressEvent(e)
        key = e.key()
        if key == Qt.Key_D or key == Qt.Key_Right:
            self.pan_right()
        elif key == Qt.Key_A or key == Qt.Key_Left:
            self.pan_left()
        elif key == Qt.Key_W or key == Qt.Key_Up:
            self.pan_up()
        elif key == Qt.Key_S or key == Qt.Key_Down:
            self.pan_down()
        elif key == Qt.Key_J or key == Qt.Key_Plus:
            self.zoom_in_horizontal()
        elif key == Qt.Key_L or key == Qt.Key_Minus:
            self.zoom_out_horizontal()
        elif key == Qt.Key_K:
            self.zoom_out_vertical()
        elif key == Qt.Key_I:
            self.zoom_in_vertical()
        else:
            print("press key", e)


def demo_samples(num_points):
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
        y = A * math.sin(omega * x) + A2 * math.cos(omega2 * x) + x
        samples.append((x, y))
    return samples


if __name__ == "__main__":
    main()
