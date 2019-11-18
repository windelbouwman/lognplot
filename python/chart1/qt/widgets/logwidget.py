from PyQt5.QtWidgets import QWidget
from PyQt5.QtGui import QPainter

from ..render_log import render_logs_on_qpainter


class LogWidget(QWidget):
    """ Visualize log records in chronological order.
    """

    def __init__(self):
        super().__init__()
        self.logs = LogBar()

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QPainter(self)
        render_logs_on_qpainter(self.logs, painter, self.rect())
