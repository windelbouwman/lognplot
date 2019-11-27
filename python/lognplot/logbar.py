from .chart.axis import Axis


class LogBar:
    def __init__(self):
        self.x_axis = Axis()
        self.logs = []


class LogRecord:
    def __init__(self, level, message):
        self.level = level
        self.message = message


class LogMetrics:
    def __init__(self):
        self.level_counters = {}
