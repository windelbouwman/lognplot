from .axis import Axis
from ..time import TimeSpan
from ..tsdb import Aggregation
import math


class EventTracks:
    """ A sort of chart of events over time.

    Horizontal axis is the time axis.
    """

    def __init__(self, db):
        self.x_axis = Axis()
        self.tracks = []
        self.db = db

    def has_track(self, name):
        for track in self.tracks:
            if track.name == name:
                return True

        return False

    def add_track(self, name):
        if not self.has_track(name):
            self.tracks.append(EventTrack(self.db, name))

    def clear_tracks(self):
        self.tracks.clear()

    def zoom_fit(self):
        """ Adjust axis to fit all curves. """
        summary = self.data_summary()

        # If we have metrics, adjust axis.
        if summary:
            self.fit_timespan_on_x_axis(summary.timespan)

    def fit_timespan_on_x_axis(self, timespan: TimeSpan):
        """ Adjust X-axis to fit timespan in view. """

        domain = timespan.end - timespan.begin
        if math.isclose(domain, 0):
            domain = 1

        minimum = timespan.begin - domain * 0.05
        maximum = timespan.end + domain * 0.05
        self.x_axis.set_limits(minimum, maximum)

    def data_summary(self, timespan=None) -> Aggregation:
        """ Metrics of all signals in the plot. """

        # Gather bounding boxes of all curves:
        aggregations = []
        for track in self.tracks:
            aggregation = track.query_summary(timespan=timespan)

            if aggregation:
                aggregations.append(aggregation)

        # If we have bounds, merge them and adjust axis.
        if aggregations:
            return Aggregation.from_aggregations(aggregations)


class EventTrack:
    def __init__(self, db, name):
        self._db = db
        self.name = name

    def query_summary(self, timespan=None):
        return self._db.query_summary(self.name, timespan=timespan)

    def query(self, selection_timespan, min_count):
        # TODO: cache calls here?
        return self._db.query(self.name, selection_timespan, min_count)
