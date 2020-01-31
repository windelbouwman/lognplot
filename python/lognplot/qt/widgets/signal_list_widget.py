from ..qtapi import QtWidgets, QtCore
from .db_tree_model import TsDbTreeModel


class SignalListWidget(QtWidgets.QWidget):
    """ A Widget which has a list of signals to drag into charts.
    """

    def __init__(self, db):
        super().__init__()
        self._signal_view = QtWidgets.QTreeView()
        self._db = db
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

        add_expression = QtWidgets.QPushButton("Add expression...")
        add_expression.clicked.connect(self.add_expression)

        l = QtWidgets.QVBoxLayout()
        l.addWidget(filter_edit)
        l.addWidget(self._signal_view)
        l.addWidget(add_expression)
        self.setLayout(l)

    def add_expression(self):
        """ Start signal expression wizard. """
        dialog = ExpressionCreateDialog(self, self._db)
        dialog.exec()


class ExpressionCreateDialog(QtWidgets.QDialog):
    def __init__(self, parent, db):
        super().__init__(parent)
        self._db = db
        l = QtWidgets.QVBoxLayout()
        l2 = QtWidgets.QFormLayout()
        l.addLayout(l2)
        self.line_edit_name = QtWidgets.QLineEdit()
        l2.addRow("Name:", self.line_edit_name)
        self.line_edit_expr = QtWidgets.QLineEdit()
        l2.addRow("Expression:", self.line_edit_expr)
        ok = QtWidgets.QPushButton("Ok")
        ok.clicked.connect(self.lets_go)
        l.addWidget(ok)
        self.setLayout(l)

    def lets_go(self):
        name = self.line_edit_name.text()
        expr = self.line_edit_expr.text()
        # print("Expression:", name, expr)
        self._db.add_function(name, expr)
        self.close()
