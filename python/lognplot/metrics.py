import operator
from functools import reduce
import math


class Metrics:
    """ Compaction range matrics.

    Idea is this is some sort of summary about data.
    For example, we have the count of samples about which
    these metrics are a summary. Also, we have minimum and
    maximum values.
    """

    def __init__(self, count, x1, x2, minimum, maximum, value_sum, value_squared_sum):
        self.count = count
        self.x1 = x1
        self.x2 = x2
        self.minimum = minimum
        self.maximum = maximum
        self.value_sum = value_sum
        self.value_squared_sum = value_squared_sum
        # TODO:
        # mean/stddev/median other statistics??

    def __add__(self, other):
        if isinstance(other, Metrics):
            count = self.count + other.count
            return Metrics(
                count=count,
                x1=min(self.x1, other.x1),
                x2=max(self.x2, other.x2),
                minimum=min(self.minimum, other.minimum),
                maximum=max(self.maximum, other.maximum),
                value_sum=self.value_sum + other.value_sum,
                value_squared_sum=self.value_squared_sum + other.value_squared_sum,
            )
        else:  # pragma: no cover
            return NotImplemented

    def __repr__(self):
        return (
            f"Metrics(count={self.count},minimum={self.minimum},maximum={self.maximum})"
        )

    @property
    def mean(self):
        """ Mean value of this data chunk """
        return self.value_sum / self.count

    @property
    def stddev(self):
        """ Calculate standard deviation. """
        if self.count > 1:
            variance = (
                self.value_squared_sum
                - ((self.value_sum * self.value_sum) / self.count)
            ) / (self.count - 1)
        else:
            variance = 0.0

        stddev = math.sqrt(variance)
        assert stddev >= 0
        return stddev


def samples_to_metric(samples) -> Metrics:
    """ Take a bunch of samples, and convert into a single metric. """
    assert samples
    return reduce(operator.add, map(sample_to_metric, samples))


def sample_to_metric(sample) -> Metrics:
    """ Convert a single sample into metrics. """
    x, y = sample
    metric = Metrics(1, x, x, y, y, y, y * y)
    # print(sample, metric)
    return metric


def merge_metrics(metrics) -> Metrics:
    """ Merge several metrics into a single metric.
    """
    assert metrics
    return reduce(operator.add, metrics)
