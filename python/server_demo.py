""" Demo on how to receive data via TCP/IP. """

import threading
from lognplot.qt.qtapi import QtWidgets
from lognplot.qt.widgets import SoftScope
from lognplot.server import run_server


def main():
    app = QtWidgets.QApplication([])
    scope = SoftScope()
    t1 = threading.Thread(
        target=run_server, args=(DataSink(scope.add_samples),), daemon=True
    )
    t1.start()
    scope.show()
    app.exec()


class DataSink:
    def __init__(self, add_samples):
        self.add_samples = add_samples


if __name__ == "__main__":
    main()
