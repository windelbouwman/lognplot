import abc
from .utils import chunk
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

    def query(self, begin, end, min_count):
        return self._tree.query(begin, end, min_count)

    def query_metrics(self, begin, end):
        return self._tree.query_metrics(begin, end)


class PointSerie(Serie):
    """ Plain point serie with a list of samples.
    """

    def __init__(self, samples=None):
        self.samples = samples or []

    def add_sample(self, sample):
        self.samples.append(sample)

    def create_compaction_series(self, interval):
        """ Create a summarizing series!
        """
        compactions = list(map(samples_to_metric, chunk(self.samples, interval)))
        compacted_serie = CompactedSerie(samples=compactions)

        return compacted_serie

    def __repr__(self):
        return f"Point serie with {len(self.samples)} points"

    def __iter__(self):
        return iter(self.samples)

    def __len__(self):
        return len(self.samples)


class CompactedSerie(Serie):
    def __init__(self, samples=None):
        self.samples = samples or []

    def __repr__(self):
        return f"Compacted serie with {len(self.samples)} aggregates"

    def add_sample(self, sample):
        raise ValueError("Cannot append samples to a compacted series")
