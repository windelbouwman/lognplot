from .utils import chunk
from .metrics import Metrics, sample_to_metric, samples_to_metric
from .btree import Btree


class Serie:
    def __init__(self, samples=None):
        self.samples = samples or []

    def add_sample(self, sample):
        self.samples.append(sample)

    def __iter__(self):
        return iter(self.samples)

    def __len__(self):
        return len(self.samples)


class ZoomSerie(Serie):
    def __init__(self):
        self._tree = Btree()

    def add_sample(self, sample):
        self._tree.append(sample)

    def add_samples(self, samples):
        for sample in samples:
            self.add_sample(sample)

    def __len__(self):
        return len(self._tree)

    def __iter__(self):
        return iter(self._tree)

    def query(self, begin, end, min_count):
        return self._tree.query(begin, end, min_count)


class PointSerie(Serie):
    def create_compaction_series(self, interval):
        """ Create a summarizing series!
        """
        compactions = list(map(samples_to_metric, chunk(self.samples, interval)))
        compacted_serie = CompactedSerie(samples=compactions)

        return compacted_serie

    def __repr__(self):
        return f"Point serie with {len(self.samples)} points"


class CompactedSerie(Serie):
    def __repr__(self):
        return f"Compacted serie with {len(self.samples)} aggregates"


class Compaction:
    """ A single compaction.
    """

    def __init__(self, metrics):
        self.metrics = metrics
