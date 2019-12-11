import logging
from ..qtapi import QtGui
from ..render.event import render_events_on_qpainter
from ...chart import EventTracks
from . import mime
from .basewidget import BaseWidget


class EventTracksWidget(BaseWidget):
    """ Visualize events in chronological order.
    """

    logger = logging.getLogger("event-widget")

    def __init__(self, db):
        super().__init__()
        self.event_tracks = EventTracks(db)
        self.setAcceptDrops(True)

    def add_track(self, name, color=None):
        self.event_tracks.add_track(name)
        self.zoom_fit()

    def paintEvent(self, e):
        super().paintEvent(e)

        # Contrapt graph via QPainter!
        painter = QtGui.QPainter(self)
        render_events_on_qpainter(self.event_tracks, painter, self.rect())
        self.draw_focus_indicator(painter, self.rect())

    # Drag-n-drop:
    def dragEnterEvent(self, event):
        if event.mimeData().hasFormat(mime.event_names_mime_type):
            event.acceptProposedAction()

    def dropEvent(self, event):
        names = bytes(event.mimeData().data(mime.event_names_mime_type)).decode("ascii")
        for name in names.split(":"):
            self.logger.debug(f"Add logger {name}")
            self.event_tracks.add_track(name)
        self.update()

    def clear_curves(self):
        """ Clear all curves """
        self.event_tracks.clear_tracks()
        self.update()

    def horizontal_pan(self, amount):
        self.event_tracks.x_axis.pan(amount)
        self.update()

    def horizontal_zoom(self, amount):
        self.event_tracks.x_axis.zoom(amount)
        self.update()

    def zoom_fit(self):
        """ Autoscale all in fit! """
        self.event_tracks.zoom_fit()
        self.update()
