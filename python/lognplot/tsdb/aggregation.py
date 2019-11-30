import operator
from functools import reduce
from ..time import TimeSpan
from .metrics import Metrics


class Aggregation:
    def __init__(self, timespan: TimeSpan, metrics: Metrics):
        self.timespan = timespan
        self.metrics = metrics

    @classmethod
    def from_sample(cls, sample):
        timestamp, value = sample
        timespan = TimeSpan(timestamp, timestamp)
        return cls(timespan, Metrics.from_value(value))

    @staticmethod
    def from_samples(samples):
        """ Take a bunch of samples, and convert into a single metric. """
        assert samples
        return reduce(operator.add, map(Aggregation.from_sample, samples))

    @classmethod
    def from_aggregations(cls, aggregations):
        assert aggregations
        metrics = []
        timespans = []
        for aggregation in aggregations:
            assert isinstance(aggregation, Aggregation)
            metrics.append(aggregation.metrics)
            timespans.append(aggregation.timespan)
        timespan = TimeSpan.from_timespans(timespans)
        metrics = Metrics.from_metrics(metrics)
        return cls(timespan, metrics)

    def __add__(self, other):
        if isinstance(other, Aggregation):
            metrics = self.metrics + other.metrics
            timespan = TimeSpan.from_timespans([self.timespan, other.timespan])
            return Aggregation(timespan, metrics)
        else:  # pragma: no cover
            return NotImplemented
