class LogLevel:
    INFO = 1
    WARNING = 2
    ERROR = 3

    LEVELS = [INFO, WARNING, ERROR]


class LogRecord:
    def __init__(self, level, message: str):
        self.level = level
        self.message = message
