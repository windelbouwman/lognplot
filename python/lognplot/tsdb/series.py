import abc
from .btree import Btree
from .aggregation import Aggregation
from ..time import TimeSpan
from .log import LogRecord


class Serie(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def add_sample(self, sample):
        raise NotImplementedError()

    def add_samples(self, samples):
        """ Naive implementation to add samples.
        """
        for sample in samples:
            self.add_sample(sample)


class LogSeries:
    """ Collection of log messages.

    Each log record has a severity, time and message. Also a log source?

    TODO: maybe merged with ZoomSerie
    """

    def __init__(self):
        self._tree = Btree()


class ZoomSerie(Serie):
    def __init__(self):
        self._tree = Btree()

    def __repr__(self):
        return "ZoomSerie"

    def get_type(self):
        first = next(iter(self), None)
        if first is None:
            return "?"
        else:
            value = first[1]
            if isinstance(value, float):
                return "signal"
            elif isinstance(value, LogRecord):
                return "logger"
            elif isinstance(value, dict):
                return "event"
            else:
                raise NotImplementedError(f"Unknown signal type: {type(value)}")

    def add_sample(self, sample):
        self._tree.append(sample)

    def add_samples(self, samples):
        self._tree.extend(samples)

    def __len__(self):
        return len(self._tree)

    def __iter__(self):
        return iter(self._tree)

    def query(self, selection_timespan: TimeSpan, min_count: int):
        return self._tree.query(selection_timespan, min_count)

    def query_summary(self, selection_timespan=None) -> Aggregation:
        if selection_timespan:
            return self._tree.query_metrics(selection_timespan)
        else:
            return self._tree.aggregation

    def query_value(self, timestamp):
        return self._tree.query_value(timestamp)


class FuncSerie(Serie):
    """ A serie which might depend upon other series. """

    def __init__(self, db, expr):
        self._db = db
        self._expr = expr

    def get_type(self):
        return "signal"

    def add_sample(self, sample):
        raise NotImplementedError("Cannot append sample to calculated serie")

    def query(self, selection_timespan: TimeSpan, min_count: int):
        signal_name = self._expr
        return self._db.query(signal_name, selection_timespan, min_count)

    def query_summary(self, selection_timespan=None) -> Aggregation:
        signal_name = self._expr
        return self._db.query_summary(signal_name, timespan=selection_timespan)

    def query_value(self, timestamp):
        signal_name = self._expr
        return self._db.query_value(signal_name, timestamp)
