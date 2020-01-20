from ..qtapi import QtWidgets, QtCore
from .db_tree_model import TsDbTreeModel


class SignalListWidget(QtWidgets.QWidget):
    """ A Widget which has a list of signals to drag into charts.
    """

    def __init__(self, db):
        super().__init__()
        self._signal_view = QtWidgets.QTreeView()
        self._signal_list_model = TsDbTreeModel(db)
        sort_filter_proxy = QtCore.QSortFilterProxyModel()
        sort_filter_proxy.setSourceModel(self._signal_list_model)
        self._signal_view.setModel(sort_filter_proxy)
        self._signal_view.setDragEnabled(True)
        self._signal_view.setSelectionMode(
            QtWidgets.QAbstractItemView.ExtendedSelection
        )
        # print(self._signal_view.selectionMode())
        filter_edit = QtWidgets.QLineEdit()
        filter_edit.setPlaceholderText("Signal search box...")
        filter_edit.textChanged.connect(sort_filter_proxy.setFilterWildcard)

        l = QtWidgets.QVBoxLayout()
        l.addWidget(filter_edit)
        l.addWidget(self._signal_view)
        self.setLayout(l)
