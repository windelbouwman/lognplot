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

    def __init__(self, count, minimum, maximum, value_sum, value_squared_sum):
        self.count = count
        self.minimum = minimum
        self.maximum = maximum
        self.value_sum = value_sum
        self.value_squared_sum = value_squared_sum
        # TODO:
        # mean/stddev/median other statistics??

    @classmethod
    def from_value(cls, value):
        """ Convert a single sample into metrics. """
        return cls(1, value, value, value, value * value)

    @staticmethod
    def from_metrics(metrics):
        """ Merge several metrics into a single metric.
        """
        assert metrics
        assert all(isinstance(m, Metrics) for m in metrics)
        return reduce(operator.add, metrics)

    def __add__(self, other):
        if isinstance(other, Metrics):
            count = self.count + other.count
            return Metrics(
                count=count,
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
