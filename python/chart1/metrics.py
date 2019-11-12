import operator
from functools import reduce


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

    def __repr__(self):
        return (
            f"Metrics(count={self.count},minimum={self.minimum},maximum={self.maximum})"
        )


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


def merge_metrics(metrics):
    """ Merge several metrics into a single metric.
    """
    return reduce(operator.add, metrics)
