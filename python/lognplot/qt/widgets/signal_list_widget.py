from ..qtapi import QtWidgets, QtCore
from .signal_list_model import SignalListModel


class SignalListWidget(QtWidgets.QWidget):
    """ A Widget which has a list of signals to drag into charts.
    """

    def __init__(self, db):
        super().__init__()
        self._signal_view = QtWidgets.QListView()
        self._signal_list_model = SignalListModel(db)
        self._signal_view.setModel(self._signal_list_model)
        self._signal_view.setDragEnabled(True)

        l = QtWidgets.QVBoxLayout()
        l.addWidget(self._signal_view)
        self.setLayout(l)
