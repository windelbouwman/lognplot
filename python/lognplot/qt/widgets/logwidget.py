from ..qtapi import QtWidgets, QtGui
from ..render.log import render_logs_on_qpainter
from ...logbar import LogBar


class LogBarWidget(QtWidgets.QWidget):
    """ Visualize log records in chronological order.
    """

    def __init__(self):
        super().__init__()
        self.log_bar = LogBar()

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QtGui.QPainter(self)
        render_logs_on_qpainter(self.log_bar, painter, self.rect())
