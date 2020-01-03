import operator
from functools import reduce
import math
from .log import LogRecord, LogLevel


class Metrics:
    """ Compaction metrics.

    This is a base class for data summary.
    """

    @classmethod
    def from_value(cls, value):
        if isinstance(value, (float, int)):
            return ValueMetrics.from_value(value)
        elif isinstance(value, LogRecord):
            return LogMetrics.from_record(value)
        elif isinstance(value, dict):
            return EventMetrics.from_event(value)
        else:
            raise ValueError(f"No metric for {type(value)}")

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


class ValueMetrics(Metrics):
    """ Compaction range matrics.

    Idea is this is some sort of summary about data.
    For example, we have the count of samples about which
    these metrics are a summary. Also, we have minimum and
    maximum values.

    Note that addition is not commutative, the chunks are ordered
    in sequence.
    """

    def __init__(self, count, minimum, maximum, first, last, mean, m2):
        self.count = count
        self.minimum = minimum
        self.maximum = maximum
        self.first = first  # First observed value
        self.last = last  # Last observed value
        self._mean = mean
        # The M2 value is a handy value for calculating the
        # variance online. See welford method on wikipedia.
        # https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm
        self._m2 = m2

    @classmethod
    def from_value(cls, value):
        """ Convert a single sample into metrics. """
        return cls(1, value, value, value, value, value, 0.0)

    def __add__(self, other):
        if isinstance(other, ValueMetrics):
            count = self.count + other.count
            assert count > 0
            mean = (self._mean * self.count + other._mean * other.count) / count
            delta = self._mean - other._mean
            m2 = (
                self._m2
                + other._m2
                + delta * delta * (self.count * other.count) / count
            )
            return ValueMetrics(
                count=count,
                minimum=min(self.minimum, other.minimum),
                maximum=max(self.maximum, other.maximum),
                first=self.first,
                last=other.last,
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


class EventMetrics(Metrics):
    """ Simplest metric, simply counting the events.
    """

    def __init__(self, count):
        self.count = count

    @classmethod
    def from_event(cls, event):
        return cls(1)

    def __add__(self, other):
        if isinstance(other, EventMetrics):
            count = self.count + other.count
            return EventMetrics(count)
        else:  # pragma: no cover
            return NotImplemented


class LogMetrics(Metrics):
    def __init__(self, count):
        self.count = count
        self.level_counters = {level: 0 for level in LogLevel.LEVELS}

    @classmethod
    def from_record(cls, record):
        m = cls(0)
        m.include(record)
        return m

    def include(self, record):
        self.count += 1
        self.level_counters[record.level] += 1

    def __add__(self, other):
        if isinstance(other, LogMetrics):
            count = self.count + other.count
            result = LogMetrics(count)
            for level in LogLevel.LEVELS:
                result.level_counters[level] = (
                    self.level_counters[level] + other.level_counters[level]
                )
            return result
        else:  # pragma: no cover
            return NotImplemented
