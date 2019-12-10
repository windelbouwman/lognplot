""" Implements a sort of dashboard like
area where user can drag-drop stuf onto.
"""
import logging

from ..qtapi import QtWidgets, Qt
from .chartwidget import ChartWidget
from .logwidget import LogBarWidget
from . import mime


class Dashboard(QtWidgets.QWidget):
    """ Dashboard widget.

    Features:
    - Drop support for signals

    Initial strategy:
        Split the area into 4 quadrants where the user can drop
        stuff.
    """

    def __init__(self, db):
        super().__init__()
        self._db = db
        rows = 2
        columns = 2

        l = QtWidgets.QGridLayout()
        for row in range(rows):
            for column in range(columns):
                dashboard_slot = DashboardSlot(db)
                l.addWidget(dashboard_slot, row, column)
        self.setLayout(l)

    def enable_tailing(self, duration):
        self.ph1.enable_tailing(duration)
        self.ph2.enable_tailing(duration)
        self.ph3.enable_tailing(duration)
        self.ph4.enable_tailing(duration)


class DashboardSlot(QtWidgets.QFrame):
    """ Placeholder which supports dropping stuff onto.
    """

    logger = logging.getLogger("dashboard")

    def __init__(self, db):
        super().__init__()
        self._db = db
        self.setAcceptDrops(True)
        self.setFrameStyle(QtWidgets.QFrame.Panel | QtWidgets.QFrame.Raised)
        self.setLineWidth(2)
        self.placeholder_label = QtWidgets.QLabel()
        self.placeholder_label.setText("Drop data here!")
        self.placeholder_label.setAlignment(Qt.AlignCenter)
        self._close_button = QtWidgets.QPushButton("Close")
        self._close_button.hide()
        self._layout = QtWidgets.QVBoxLayout()
        self._layout.addWidget(self._close_button)
        self._layout.addWidget(self.placeholder_label)
        self.setLayout(self._layout)
        self._chart_widget = None
        self._log_widget = None
        self._close_button.clicked.connect(self._close_inner)

    def _close_inner(self):
        # Enable drops:
        self._close_button.hide()
        self.placeholder_label.show()
        self.setAcceptDrops(True)

        if self._chart_widget:
            self._layout.removeWidget(self._chart_widget)
            self._chart_widget.deleteLater()
            self._chart_widget = None

        if self._log_widget:
            self._layout.removeWidget(self._log_widget)
            self._log_widget.deleteLater()
            self._log_widget = None

    def enable_tailing(self, duration):
        if self._chart_widget:
            self._chart_widget.enable_tailing(duration)

    def dragEnterEvent(self, event):
        mime_data = event.mimeData()
        if mime_data.hasFormat(mime.signal_names_mime_type) or mime_data.hasFormat(
            mime.logger_names_mime_type
        ):
            event.acceptProposedAction()

    def dropEvent(self, event):
        self.logger.debug("Filling placeholder")

        # Hide place holder:
        self.placeholder_label.hide()
        self._close_button.show()

        # Do not accept new drops:
        self.setAcceptDrops(False)

        mime_data = event.mimeData()
        if mime_data.hasFormat(mime.signal_names_mime_type):
            names = bytes(mime_data.data(mime.signal_names_mime_type)).decode("ascii")

            # Create new chart widget:
            self._chart_widget = ChartWidget(self._db)
            self._layout.addWidget(self._chart_widget)

            # print("Mime data text", names, type(names))
            for name in names.split(":"):
                self.logger.debug(f"Adding curve |{name}|")
                self._chart_widget.add_curve(name)
        elif mime_data.hasFormat(mime.logger_names_mime_type):
            names = bytes(mime_data.data(mime.logger_names_mime_type)).decode("ascii")

            # Create log bar chart:
            self._log_widget = LogBarWidget(self._db)
            self._layout.addWidget(self._log_widget)

            # Add loggers:
            for name in names.split(":"):
                self.logger.debug(f"Adding log track {name}")
                self._log_widget.log_bar.add_track(name)

        self.update()
