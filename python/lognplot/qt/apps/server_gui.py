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
from ...demo_data import create_demo_samples, create_demo_log_messages


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
        self.db.add_samples("L1", create_demo_log_messages(5000))
        self.db.add_samples("L2", create_demo_log_messages(50))

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

        self.create_menus()

    def create_menus(self):
        # Menu
        menu_bar = self.menuBar()
        file_menu = menu_bar.addMenu("File")
        quit_action = file_menu.addAction("Quit")
        quit_action.triggered.connect(self.close)
        help_menu = menu_bar.addMenu("Help")
        usage_action = help_menu.addAction("Usage")
        usage_action.triggered.connect(self.show_usage_dialog)
        usage_action.setShortcut(QtGui.QKeySequence("F1"))
        about_action = help_menu.addAction("About")
        about_action.triggered.connect(self.show_about_dialog)

    def show_usage_dialog(self):
        QtWidgets.QMessageBox.information(self, "Usage", help_text)

    def show_about_dialog(self):
        QtWidgets.QMessageBox.about(self, "About lognplot", about_text)


about_text = """
<h1>About lognplot</h1>

<p>
Lognplot is a tool to log and visualize data while the data
is being recorded.
</p>

<p>
Please report bugs / submit changes if you have any improvements!
</p>

Website:
<a href="https://github.com/windelbouwman/lognplot">
https://github.com/windelbouwman/lognplot
</a>
"""

help_text = """
<h1>Usage</h1>

You can view signals in the left panel, and drag them onto
the dashboard. Then, to clear signals, select a plot and press
delete / backspace.

<h2>Keys</h2>

<ul>
<li> <b>w,a,s,d</b> panning a plot </li>
<li> <b>up,left,down,right</b> panning a plot </li>
<li> <b>i,j,k,l</b> zooming a plot </li>
<li> <b>+,-</b> zooming horizontally</li>
<li> <b>space/enter</b> zoom to fit</li>
<li> <b>delete/backspace</b> delete all curves</li>
</ul>

"""


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
