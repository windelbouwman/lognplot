from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QPainter


class CallStackWidget(QWidget):
    """ Visualize a program callstack. """

    def __init__(self):
        super().__init__()
        self.call_stack = CallStackBar()

    def paintEvent(self, e):
        super().paintEvent(e)

