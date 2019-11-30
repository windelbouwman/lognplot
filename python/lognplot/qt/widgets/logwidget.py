from ..qtapi import QtWidgets, QtGui
from ..render_log import render_logs_on_qpainter


class LogWidget(QtWidgets.QWidget):
    """ Visualize log records in chronological order.
    """

    def __init__(self):
        super().__init__()
        self.logs = LogBar()

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QtGui.QPainter(self)
        render_logs_on_qpainter(self.logs, painter, self.rect())
