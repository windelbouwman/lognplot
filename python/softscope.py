""" Soft scope prototype demo.

Generate a 10 kHz samples sine wave and plot it live.
"""

import math
import time
import queue
import threading
from PyQt5.QtWidgets import QApplication, QWidget, QVBoxLayout
from PyQt5.QtCore import QTimer
from chart1 import ZoomSerie
from chart1.qt.widgets.chartwidget import ChartWidget


def main():
    app = QApplication([])
    w = DemoGraphWidget()
    t1 = threading.Thread(target=gen_data, args=(w._rx_queue,), daemon=True)
    t1.start()
    w.show()
    app.exec()


def gen_data(tx_queue):
    print("enter thread")
    ts = 0.0001
    t = 0
    dt = 0.2
    F = 400
    chunk_size = int(dt / ts)
    while True:
        samples = []
        for _ in range(chunk_size):
            v = math.sin(t * math.pi * 2.0 * F) + 0.1 * t
            samples.append((t, v))

            # Increment time:
            t += ts
        tx_queue.put(samples)
        time.sleep(dt)
    print("rip thread")


class DemoGraphWidget(QWidget):
    def __init__(self):
        super().__init__()

        self.series1 = ZoomSerie()

        self._chart_widget = ChartWidget()
        self._chart_widget.chart.add_serie(self.series1)

        l = QVBoxLayout()
        l.addWidget(self._chart_widget)
        self.setLayout(l)

        self.resize(600, 400)

        self._rx_queue = queue.Queue()
        self._timer = QTimer()
        self._timer.timeout.connect(self.on_timeout)
        self._timer.start(50)

    def on_timeout(self):
        if not self._rx_queue.empty():
            while not self._rx_queue.empty():
                samples = self._rx_queue.get()
                self.series1.add_samples(samples)
                self._rx_queue.task_done()
            self.update()


if __name__ == "__main__":
    main()
