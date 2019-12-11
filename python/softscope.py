""" Soft scope prototype demo.

Generate a 10 kHz samples sine wave and plot it live.
"""

import math
import time
import random
import threading
from lognplot.qt.qtapi import QtWidgets
from lognplot.qt.widgets import SoftScope


def main():
    app = QtWidgets.QApplication([])
    scope = SoftScope()
    t1 = threading.Thread(target=gen_data, args=(scope.add_samples,), daemon=True)
    t1.start()
    scope.show()
    app.exec()


def gen_data(add_samples):
    ts = 0.0001
    t = 0
    dt = 0.02
    F = 400
    F2 = 3
    chunk_size = int(dt / ts)
    while True:
        samples = []
        samples2 = []
        for _ in range(chunk_size):
            v = 4 * math.sin(t * math.pi * 2.0 * F) - 2
            samples.append((t, v))
            v2 = 6 * math.sin(t * math.pi * 2.0 * F2) + 5 + 0.1 * random.random()
            samples2.append((t, v2))

            # Increment time:
            t += ts
        add_samples("C1", samples)
        add_samples("C2", samples2)
        time.sleep(dt)


if __name__ == "__main__":
    main()
