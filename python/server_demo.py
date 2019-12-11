""" Demo on how to receive data via TCP/IP. """

import threading
from lognplot.qt.qtapi import QtWidgets
from lognplot.qt.widgets import SoftScope
from lognplot.server import run_server
import asyncio

try:
    from lognplot.web.rest import serve_db_via_rest
except ImportError:
    serve_db_via_rest = None


def main():
    app = QtWidgets.QApplication([])
    scope = SoftScope()

    t1 = threading.Thread(
        target=run_server, args=(DataSink(scope.add_samples),), daemon=True
    )
    t1.start()

    if serve_db_via_rest:
        t2 = threading.Thread(target=rest, args=(scope.db,), daemon=True)
        t2.start()

    scope.show()
    app.exec()


def rest(db):
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    serve_db_via_rest(db)


class DataSink:
    def __init__(self, add_samples):
        self.add_samples = add_samples


if __name__ == "__main__":
    main()
