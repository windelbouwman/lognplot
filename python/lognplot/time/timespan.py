class TimeSpan:
    def __init__(self, begin, end):
        self.begin = begin
        self.end = end

    def is_valid(self):
        return self.begin <= self.end

    @classmethod
    def from_timespans(cls, timespans):
        assert timespans
        begin = min(ts.begin for ts in timespans)
        end = max(ts.end for ts in timespans)
        return cls(begin, end)

    def covers(self, other):
        """ The if this timespan fully covers the other timespan.
        """
        assert self.is_valid()
        assert other.is_valid()
        return (self.begin <= other.begin) and (other.end <= self.end)

    def overlaps(self, other):
        """ Test if this span overlaps the other span.

        """
        assert self.is_valid()
        assert other.is_valid()

        return (self.begin <= other.end) and (other.begin <= self.end)

    def contains_timestamp(self, timestamp):
        """ Test if this timespan contains the given timestamp. """
        return self.begin <= timestamp <= self.end

    def central_timestamp(self):
        """ Retrieve the timestamp in the middle of this timespan. """
        return (self.begin + self.end) / 2.0
