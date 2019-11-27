""" Implement a signal list of a time series database.
"""

from PyQt5.QtCore import QAbstractListModel, Qt, QMimeData


class SignalListModel(QAbstractListModel):
    def __init__(self, db):
        super().__init__()
        self.db = db
        self.names = []

    def update(self):
        """ Refresh the list content! """
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
        mimeData = QMimeData()

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
