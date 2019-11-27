import abc
from ..utils import chunk
from .metrics import Metrics, sample_to_metric, samples_to_metric
from .btree import Btree


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

    def add_sample(self, sample):
        self._tree.append(sample)

    def add_samples(self, samples):
        self._tree.extend(samples)

    def __len__(self):
        return len(self._tree)

    def __iter__(self):
        return iter(self._tree)

    @property
    def metrics(self):
        return self._tree.metrics

    def query(self, selection_timespan, min_count):
        return self._tree.query(selection_timespan, min_count)

    def query_metrics(self, selection_timespan):
        return self._tree.query_metrics(selection_timespan)
