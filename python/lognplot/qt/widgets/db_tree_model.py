""" Tree model of a time series database.

Ideas:
- Use a color for the age of elements
- Provide column with name
- Provide the latest value as a column
- Provide the latest time as a value
- Groups signals some way?
"""

import time
from ..qtapi import QtCore, Qt, QtGui


class TsDbTreeModel(QtCore.QAbstractItemModel):
    """ Implement a complex tree view onto the time series database.
    """

    FADE_TIME = 5

    def __init__(self, db):
        super().__init__()
        self.db = db
        self.names = []
        self._column_names = ["Name", "Datasize", "Last value"]
        self._last_values = {}  # Keep track of last value
        self._last_times = {}
        self._last_active = {}

        # Do a polling on the model...
        # TODO: how to prevent polling?
        self._timer = QtCore.QTimer()
        self._timer.timeout.connect(self._on_timeout)
        self._timer.start(500)

    def _on_timeout(self):
        """ Refresh the list content! """
        self._update_names()
        self._update_color()

    def _update_names(self):
        new_names = list(sorted(self.db.signal_names()))
        if new_names != self.names:
            self.names = new_names
            # TODO: finer grained change emission:
            self.modelReset.emit()

    def _signal_age(self, name):
        return time.time() - self._last_active.get(name, 0)

    def _update_color(self):
        parent = QtCore.QModelIndex()
        for row, name in enumerate(self.names):
            last_time, last_value = self.db.last_value(name)

            previous_last_value = self._last_values.get(name, None)
            if last_value != previous_last_value:
                self._last_values[name] = last_value
                # We have a new value!

                roles = [Qt.DisplayRole]
                self.row_changed(row, parent, roles)

            previous_last_time = self._last_times.get(name, None)
            if last_time != previous_last_time:
                self._last_times[name] = last_time
                self._last_active[name] = time.time()

            # Color the row based on age:
            row_age = self._signal_age(name)
            if row_age < self.FADE_TIME + 4:
                roles = [Qt.BackgroundRole]
                self.row_changed(row, parent, roles)

    def row_changed(self, row, parent, roles):
        from_index = self.index(row, 0, parent)
        to_index = self.index(row, len(self._column_names) - 1, parent)
        self.dataChanged.emit(from_index, to_index, roles)

    def columnCount(self, parent):
        if parent.isValid():
            return 0
        else:
            return len(self._column_names)

    def headerData(self, section, orientation, role):
        if orientation == Qt.Horizontal:
            if role == Qt.DisplayRole:
                return self._column_names[section]

    def rowCount(self, parent):
        if parent.isValid():
            # TODO: enable nested groups..
            return 0
        else:
            return len(self.names)

    def index(self, row, column, parent):
        if parent.isValid():
            return None
        else:
            return self.createIndex(row, column, None)

    def parent(self, index):
        return QtCore.QModelIndex()

    def flags(self, index):
        default_flags = super().flags(index)
        if index.isValid():
            return default_flags | Qt.ItemIsDragEnabled
        else:
            return default_flags

    def mimeTypes(self):
        return ["text/plain"]

    def mimeData(self, indexes):
        mimeData = QtCore.QMimeData()

        signal_names = []
        for index in indexes:
            if index.isValid():
                row, column = index.row(), index.column()
                # print(index.row(), index.column())
                if column == 0:
                    name = self.names[row]
                    signal_names.append(name)

        payload = ":".join(signal_names).encode("utf8")
        mimeData.setData("text/plain", payload)
        return mimeData

    def data(self, index, role):
        if index.isValid():
            row, column = index.row(), index.column()
            name = self.names[row]
            if role == Qt.DisplayRole:
                if column == 0:
                    return name
                elif column == 1:
                    return str(self.db.query_len(name))
                elif column == 2:
                    return str(self._last_values.get(name, "?"))
                else:
                    return "?"
            elif role == Qt.BackgroundRole:
                age = self._signal_age(name)
                age_color = self.age_to_color(age)
                return QtGui.QBrush(age_color)

    def age_to_color(self, age):
        """ Convert a certain old-ness to a color. """
        if age > self.FADE_TIME:
            percent = 1.0
        elif age < 0:
            percent = 0.0
        else:
            percent = age / self.FADE_TIME

        r = 0
        g = 255
        b = 0
        a = 255 * (1.0 - percent)
        return QtGui.QColor(r, g, b, a)
