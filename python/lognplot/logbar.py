from .chart.axis import Axis


class LogBar:
    """ Like a chart, but contains tracks of log messages.
    """

    def __init__(self):
        self.x_axis = Axis()
        self.log_tracks = []


class LogLevel:
    INFO = 1
    WARNING = 2
    ERROR = 3


class LogRecord:
    def __init__(self, level, message: str):
        self.level = level
        self.message = message


class LogMetrics:
    def __init__(self):
        self.level_counters = {}
