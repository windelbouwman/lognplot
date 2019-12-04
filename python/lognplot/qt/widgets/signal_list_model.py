""" Implement a signal list of a time series database.
"""

from ..qtapi import QtCore, Qt


class SignalListModel(QtCore.QAbstractListModel):
    def __init__(self, db):
        super().__init__()
        self.db = db
        self.names = []

        # Do a polling on the model...
        # TODO: how to prevent polling?
        self._timer = QtCore.QTimer()
        self._timer.timeout.connect(self._on_timeout)
        self._timer.start(500)

    def _on_timeout(self):
        new_names = list(sorted(self.db.signal_names()))
        if new_names != self.names:
            self.names = new_names
            # TODO: finer grained change emission:
            self.modelReset.emit()

    def rowCount(self, parent):
        if parent.isValid():
            return 0  # Must be zero
        else:
            return len(self.names)

    def flags(self, index):
        default_flags = super().flags(index)
        if index.isValid():
            return default_flags | Qt.ItemIsDragEnabled

    def mimeTypes(self):
        return ["text/plain"]

    def mimeData(self, indexes):
        mimeData = QtCore.QMimeData()

        signal_names = []
        for index in indexes:
            if index.isValid():
                text = self.data(index, Qt.DisplayRole)
                signal_names.append(text)

        payload = ":".join(signal_names).encode("utf8")
        mimeData.setData("text/plain", payload)
        return mimeData

    def data(self, index, role):
        if index.isValid():
            if role == Qt.DisplayRole:
                return self.names[index.row()]
