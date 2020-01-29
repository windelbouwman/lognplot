""" Implements a sort of dashboard like
area where user can drag-drop stuf onto.
"""
import logging

from ..qtapi import QtWidgets, Qt
from .chartwidget import ChartWidget
from .logwidget import LogBarWidget
from .eventwidget import EventTracksWidget
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
        self._dashboard_slots = []
        l = QtWidgets.QGridLayout()
        self.setLayout(l)
        self.use_4grid()

    def clear_layout(self):
        l = self.layout()
        for dashboard_slot in self._dashboard_slots:
            dashboard_slot.hide()
            l.removeWidget(dashboard_slot)
        self._dashboard_slots.clear()

    def use_one_plot(self):
        self.clear_layout()
        l = self.layout()
        dashboard_slot = DashboardSlot(self._db)
        l.addWidget(dashboard_slot)
        self._dashboard_slots.append(dashboard_slot)

    def use_4grid(self):
        rows = 2
        columns = 2
        self.clear_layout()
        l = self.layout()
        for row in range(rows):
            for column in range(columns):
                dashboard_slot = DashboardSlot(self._db)
                l.addWidget(dashboard_slot, row, column)
                self._dashboard_slots.append(dashboard_slot)

    def enable_tailing(self, duration):
        for dashboard_slot in self._dashboard_slots:
            dashboard_slot.enable_tailing(duration)

    def restore(self, json_slots):
        """ Given some json, restore the state from it. """
        for json_slot, slot in zip(json_slots, self._dashboard_slots):
            slot.restore(json_slot)

    def save(self):
        """ Save current dashboard config as a dict. """
        json_slots = []
        for slot in self._dashboard_slots:
            json_slot = slot.save()
            json_slots.append(json_slot)
        return json_slots


class DashboardSlot(QtWidgets.QFrame):
    """ Placeholder which supports dropping stuff onto.
    """

    logger = logging.getLogger("dashboard")

    def __init__(self, db):
        super().__init__()
        self._db = db
        self._in_use = False
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
        self._event_widget = None
        self._close_button.clicked.connect(self._close_inner)

    def _close_inner(self):
        # Enable drops:
        self._close_button.hide()
        self.placeholder_label.show()
        self.setAcceptDrops(True)
        self._in_use = False

        if self._chart_widget:
            self._layout.removeWidget(self._chart_widget)
            self._chart_widget.deleteLater()
            self._chart_widget = None

        if self._log_widget:
            self._layout.removeWidget(self._log_widget)
            self._log_widget.deleteLater()
            self._log_widget = None

        if self._event_widget:
            self._layout.removeWidget(self._event_widget)
            self._event_widget.deleteLater()
            self._event_widget = None

    def enable_tailing(self, duration):
        if self._chart_widget:
            self._chart_widget.enable_tailing(duration)

    def dragEnterEvent(self, event):
        mime_data = event.mimeData()
        if (
            mime_data.hasFormat(mime.signal_names_mime_type)
            or mime_data.hasFormat(mime.logger_names_mime_type)
            or mime_data.hasFormat(mime.event_names_mime_type)
        ):
            event.acceptProposedAction()

    def dropEvent(self, event):
        self.start_to_use()

        mime_data = event.mimeData()
        if mime_data.hasFormat(mime.signal_names_mime_type):
            names = (
                bytes(mime_data.data(mime.signal_names_mime_type))
                .decode("ascii")
                .split(":")
            )
            self.add_chart_view(names)
        elif mime_data.hasFormat(mime.logger_names_mime_type):
            names = (
                bytes(mime_data.data(mime.logger_names_mime_type))
                .decode("ascii")
                .split(":")
            )

            # Create log bar chart:
            self._log_widget = LogBarWidget(self._db)
            self._layout.addWidget(self._log_widget)

            # Add loggers:
            for name in names:
                self.logger.debug(f"Adding log track {name}")
                self._log_widget.log_bar.add_track(name)
        elif mime_data.hasFormat(mime.event_names_mime_type):
            names = bytes(mime_data.data(mime.event_names_mime_type)).decode("ascii")

            # Create log bar chart:
            self._event_widget = EventTracksWidget(self._db)
            self._layout.addWidget(self._event_widget)

            # Add loggers:
            for name in names.split(":"):
                self.logger.debug(f"Adding event track {name}")
                self._event_widget.add_track(name)
        else:
            raise NotImplementedError()

        self.update()

    def start_to_use(self):
        self.logger.debug("Filling placeholder")

        # Hide place holder:
        self.placeholder_label.hide()
        self._close_button.show()

        # Do not accept new drops:
        self.setAcceptDrops(False)
        self._in_use = True

    def add_chart_view(self, names):
        # Create new chart widget:
        self._chart_widget = ChartWidget(self._db)
        self._layout.addWidget(self._chart_widget)

        # print("Mime data text", names, type(names))
        for name in names:
            self.logger.debug(f"Adding curve |{name}|")
            self._chart_widget.add_curve(name)

    def save(self):
        """ Serialize this dashboard slot. """
        if self._chart_widget:
            curves = []
            for curve in self._chart_widget.chart.curves:
                curves.append(curve.name)
            json_stuff = {
                "type": "graph",
                "curves": curves,
            }
        else:
            json_stuff = {
                "type": "empty",
            }

        return json_stuff

    def restore(self, json_stuff):
        """ Restore this dashboard slot. """
        # print(json_stuff)
        typ = json_stuff["type"]
        if typ == "graph":
            names = json_stuff["curves"]
            if self._in_use:
                self._close_inner()
            self.start_to_use()
            self.add_chart_view(names)
