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

    def __init__(self, count, minimum, maximum, mean, m2):
        self.count = count
        self.minimum = minimum
        self.maximum = maximum
        self._mean = mean
        # The M2 value is a handy value for calculating the
        # variance online. See welford method on wikipedia.
        # https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm
        self._m2 = m2

    @classmethod
    def from_value(cls, value):
        """ Convert a single sample into metrics. """
        return cls(1, value, value, value, 0.0)

    @classmethod
    def from_values(cls, values):
        """ Convert a single sample into metrics. """
        metrics = [cls.from_value(v) for v in values]
        return cls.from_metrics(metrics)

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
            assert count > 0
            mean = (self._mean * self.count + other._mean * other.count) / count
            delta = self._mean - other._mean
            m2 = (
                self._m2
                + other._m2
                + delta * delta * (self.count * other.count) / count
            )
            return Metrics(
                count=count,
                minimum=min(self.minimum, other.minimum),
                maximum=max(self.maximum, other.maximum),
                mean=mean,
                m2=m2,
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
        return self._mean

    def variance(self):
        """ Calculate the variance of this aggregation """
        if self.count > 0:
            return self._m2 / self.count
        else:
            return 0.0

    @property
    def stddev(self):
        """ Calculate standard deviation. """
        variance = self.variance()
        stddev = math.sqrt(variance)
        assert stddev >= 0
        return stddev
