""" Time series database.
"""

from .series import ZoomSerie, FuncSerie
from .aggregation import Aggregation
from ..time import TimeSpan


class TsDb:
    """ A time series database.
    """

    def __init__(self):
        # TODO: load / store data in file!
        self._traces = {}  # The internal trace data.
        self._tokens = 0
        self._event_backlog = False
        self._callbacks = []

    def clear(self):
        """ Remove all signals from the database. """
        self._traces.clear()

    def signal_names_and_types(self):
        """ Get a sorted list of signal names. """
        names_and_types = [(name, self.get_serie_type(name)) for name in self._traces]
        return list(sorted(names_and_types))

    def get_serie_type(self, name):
        serie = self.get_serie(name)
        if serie:
            return serie.get_type()

    def get_serie(self, name):
        if name in self._traces:
            return self._traces[name]

    def get_or_create_serie(self, name):
        if name in self._traces:
            serie = self._traces[name]
        else:
            serie = ZoomSerie()
            self._traces[name] = serie
            self.notify_changed()
        return serie

    # Math operation!
    def add_function(self, name, expr):
        # TODO: name clash?
        assert name not in self._traces
        serie = FuncSerie(self, expr)
        self._traces[name] = serie

    # Data insertion functions:
    def add_sample(self, name: str, sample):
        """ Add a single sample to the given series. """
        serie = self.get_or_create_serie(name)
        serie.add_sample(sample)
        self.notify_changed()

    def add_samples(self, name: str, samples):
        """ Add samples to the given series. """
        serie = self.get_or_create_serie(name)
        serie.add_samples(samples)
        self.notify_changed()

    # Query related functions:
    def query_summary(self, name: str, timespan=None) -> Aggregation:
        serie = self.get_serie(name)
        if serie:
            return serie.query_summary(selection_timespan=timespan)

    def query(self, name: str, timespan: TimeSpan, count: int):
        """ Query the database on the given signal.
        """
        serie = self.get_serie(name)
        if serie:
            return serie.query(timespan, count)

    def query_value(self, name, timestamp):
        serie = self.get_serie(name)
        if serie:
            return serie.query_value(timestamp)

    # Change handlers
    def register_changed_callback(self, callback):
        self._callbacks.append(callback)
        self._tokens += 1

    def insert_token(self):
        self._tokens += 1
        if self._event_backlog:
            self._event_backlog = False
            self._tokens -= 1
            for callback in self._callbacks:
                callback()

    def notify_changed(self):
        """ Notify listeners of a change.

        Rate limit the events to prevent GUI flooding.
        To do this, keep a token counter, if there is
        an event, check the tokens, if there is a token,
        propagate the event. Otherwise, store the event
        for later processing.

        If more events arrive, aggregate the events into
        a resulting event.
        """
        if self._tokens > 0:
            self._tokens -= 1
            for callback in self._callbacks:
                callback()
        else:
            # Simplest event aggregation: there was an event
            self._event_backlog = True
