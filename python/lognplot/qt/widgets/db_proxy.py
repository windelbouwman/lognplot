from ..qtapi import QtCore


class DbProxy(QtCore.QObject):
    """ A Qt intermediate layer for the TSDB.

    Functionality:
    - Provide data changed event with limited rate.
    """

    changed = QtCore.pyqtSignal(bool)

    def __init__(self, db):
        super().__init__()
        self.db = db
        self.register_on_changed(self.on_changed)

    def on_changed(self):
        """ Callback which is called when there is change in the database
        """
        self.changed.emit(True)
