import operator
from functools import reduce
from .utils import chunk


class Serie:
    def __init__(self, samples=None):
        self.samples = samples or []

    def __iter__(self):
        return iter(self.samples)

    def __len__(self):
        return len(self.samples)


class PointSerie(Serie):
    def create_compaction_series(self, interval):
        """ Create a summarizing series!
        """
        compactions = list(map(samples_to_metric, chunk(self.samples, interval)))
        compacted_serie = CompactedSerie(samples=compactions)

        return compacted_serie

    def __repr__(self):
        return f"Point serie with {len(self.samples)} points"


def samples_to_metric(samples):
    return reduce(operator.add, map(sample_to_metric, samples))

    # i = iter(samples)
    # r = sample_to_metric(next(i))
    # for sample in i:
    #     r = r + sample_to_metric(sample)
    # return r


def sample_to_metric(sample):
    """ Convert a single sample into metrics. """
    x, y = sample
    metric = Metrics(1, x, x, y, y)
    # print(sample, metric)
    return metric


class CompactedSerie(Serie):
    def __repr__(self):
        return f"Compacted serie with {len(self.samples)} aggregates"


class Compaction:
    """ A single compaction.
    """

    def __init__(self, metrics):
        self.metrics = metrics


class Metrics:
    """ Compaction range matrics.

    Idea is this is some sort of summary about data.
    For example, we have the count of samples about which
    these metrics are a summary. Also, we have minimum and
    maximum values.
    """

    def __init__(self, count, x1, x2, minimum, maximum):
        self.count = count
        self.x1 = x1
        self.x2 = x2
        self.minimum = minimum
        self.maximum = maximum
        # TODO:
        # mean/stddev/median other statistics??

    def __add__(self, other):
        if isinstance(other, Metrics):
            return Metrics(
                count=self.count + other.count,
                x1=min(self.x1, other.x1),
                x2=max(self.x2, other.x2),
                minimum=min(self.minimum, other.minimum),
                maximum=max(self.maximum, other.maximum),
            )
        else:
            return NotImplemented
