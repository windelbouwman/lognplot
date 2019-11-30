from ..qtapi import QtWidgets


class CallStackWidget(QtWidgets.QWidget):
    """ Visualize a program callstack. """

    def __init__(self):
        super().__init__()
        self.call_stack = CallStackBar()

    def paintEvent(self, e):
        super().paintEvent(e)
