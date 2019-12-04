""" A GUI which will listen on port 12345 to incoming connections
and enable the plotting of this data.
"""
import threading
import base64

from ..qtapi import QtWidgets, Qt, QtGui
from ..widgets import SoftScope, Dashboard, SignalListWidget
from ..widgets.timespan_toolbutton import DurationToolButton
from ...server import run_server
from ...tsdb import TsDb
from ...demo_data import create_demo_samples


def run_server_gui():
    app = QtWidgets.QApplication([])
    gui = ServerGuiMainWindow()
    gui.showMaximized()
    app.exec()


class ServerGuiMainWindow(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()
        self.db = TsDb()
        self.db.add_samples("C1", create_demo_samples(1000))
        self.db.add_samples("C2", create_demo_samples(1000, offset=60))
        self.db.add_samples("C3", create_demo_samples(2000, offset=20))
        self.db.add_samples("C5", create_demo_samples(5000, offset=-20))

        t1 = threading.Thread(
            target=run_server, args=(DatabaseSink(self.db),), daemon=True
        )
        t1.start()

        self.setWindowTitle("lognplot")
        icon_data = base64.decodebytes(icon_png_base64.encode("ascii"))
        icon_pixmap = QtGui.QPixmap()
        icon_pixmap.loadFromData(icon_data)
        icon = QtGui.QIcon(icon_pixmap)
        self.setWindowIcon(icon)

        # Central widget:
        self._dashboard = Dashboard(self.db)
        self.setCentralWidget(self._dashboard)

        # Data trace view side panel:
        self.signal_selector = SignalListWidget(self.db)
        self.signal_dock_widget = QtWidgets.QDockWidget("Signals")
        self.signal_dock_widget.setWidget(self.signal_selector)
        self.addDockWidget(Qt.LeftDockWidgetArea, self.signal_dock_widget)

        # Toolbar:
        toolbar = self.addToolBar("Actions")
        zoom_quick_select = DurationToolButton()
        toolbar.addWidget(zoom_quick_select)
        zoom_quick_select.duration_selected.connect(self._dashboard.enable_tailing)


class DatabaseSink:
    def __init__(self, db):
        self.db = db

    def add_samples(self, name, samples):
        self.db.add_samples(name, samples)


# Icon as base64 text:
icon_png_base64 = """
iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAABGdBTUEAALGPC/xhBQAAAAlwSFlz
AAAe4AAAHuABgwaXIwAAAFBJREFUaN7t17ENACAMA0GH/XcOFSsQIe4qSl40OGFWnUOn+62LVyXJ
ev0FBAgQIEAAAFhkY4vsLCt/IQECBAgQAAAW2e1F5S8kQIAAAXxtA1YXCTJthdYIAAAAAElFTkSu
QmCC
"""