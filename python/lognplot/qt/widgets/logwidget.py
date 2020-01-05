import logging
from ..qtapi import QtWidgets, QtGui
from ..render.log import render_logs_on_qpainter
from ...chart import LogBar
from . import mime
from .basewidget import BaseWidget


class LogBarWidget(BaseWidget):
    """ Visualize log records in chronological order.
    """

    logger = logging.getLogger("log-widget")

    def __init__(self, db):
        super().__init__()
        self.log_bar = LogBar(db)
        self.setAcceptDrops(True)

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QtGui.QPainter(self)
        render_logs_on_qpainter(self.log_bar, painter, self.rect())
        self.draw_focus_indicator(painter, self.rect())

    # Drag-n-drop:
    def dragEnterEvent(self, event):
        if event.mimeData().hasFormat(mime.logger_names_mime_type):
            event.acceptProposedAction()

    def dropEvent(self, event):
        names = bytes(event.mimeData().data(mime.logger_names_mime_type)).decode(
            "ascii"
        )
        for name in names.split(":"):
            self.logger.debug(f"Add logger {name}")
            self.log_bar.add_track(name)
        self.update()

    def clear_curves(self):
        """ Clear all curves """
        self.log_bar.clear_tracks()
        self.update()

    def horizontal_pan(self, amount):
        self.log_bar.x_axis.pan_relative(amount)
        self.update()

    def horizontal_zoom(self, amount, around):
        self.log_bar.x_axis.zoom(amount, around=around)
        self.update()
