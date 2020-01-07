from ..tsdb.aggregation import Aggregation
from .axis import Axis

class Curve:
    """ A curve is a view onto a signal in the database.

    Note that this curve is read only view onto the data.

    This is also the point where we assign a color to the data.

    """

    def __init__(self, db, name, color):
        self._db = db
        self.name = name
        self.color = color
        # Corresponding handle (polygon area)
        self.handle = []
        # Corresponding legend segment (polygon area)
        self.legend_segment = []
        # Each curve has its own vertical axis
        self.axis = Axis()

    def __repr__(self):
        return "Database proxy-curve"

    def __len__(self):
        return self._db.query_len(self.name)

    def query_summary(self, timespan=None) -> Aggregation:
        return self._db.query_summary(self.name, timespan=timespan)

    def query(self, selection_timespan, min_count):
        # TODO: cache calls here?
        return self._db.query(self.name, selection_timespan, min_count)

    def query_value(self, timestamp):
        return self._db.query_value(self.name, timestamp)
