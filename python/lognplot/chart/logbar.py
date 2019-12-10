from .axis import Axis


class LogBar:
    """ Like a chart, but contains tracks of log messages.
    """

    def __init__(self, db):
        self.x_axis = Axis()
        self.log_tracks = []
        self.db = db

    def has_track(self, name):
        for track in self.log_tracks:
            if track.name == name:
                return True

        return False

    def add_track(self, name):
        if not self.has_track(name):
            self.log_tracks.append(LogTrack(self.db, name))

    def clear_tracks(self):
        self.log_tracks.clear()


class LogTrack:
    """ A single log track, refering to a log message stream in the database. """

    def __init__(self, db, name):
        self._db = db
        self.name = name

    def query_summary(self, timespan=None):
        return self._db.query_summary(self.name, timespan=timespan)

    def query(self, selection_timespan, min_count):
        # TODO: cache calls here?
        return self._db.query(self.name, selection_timespan, min_count)
