from ..qtapi import QtWidgets, pyqtSignal
from ...time import Duration


class DurationToolButton(QtWidgets.QToolButton):
    """ Toolbar button which allows selection some duration.

    Usable when selecting to view recent history.
    """

    duration_selected = pyqtSignal(float)

    def __init__(self):
        super().__init__()
        self.setPopupMode(QtWidgets.QToolButton.InstantPopup)
        self.setText("Zoom to last ...")
        range_menu = QtWidgets.QMenu()
        for name, duration in RANGES:
            self._make_range_handler(range_menu, name, duration)
        self.setMenu(range_menu)

    def _make_range_handler(self, menu, name, duration):
        def handler():
            self.duration_selected.emit(duration.to_seconds())

        zoom_action = menu.addAction(name)
        zoom_action.triggered.connect(handler)


# Selectable ranges:
RANGES = [
    ("nanosecond", Duration.from_nano_seconds(1)),
    ("microsecond", Duration.from_nano_seconds(1000)),
    ("millisecond", Duration.from_milli_seconds(1)),
    ("100 milliseconds", Duration.from_milli_seconds(100)),
    ("5 seconds", Duration.from_seconds(5)),
    ("10 seconds", Duration.from_seconds(10)),
    ("15 seconds", Duration.from_seconds(15)),
    ("30 seconds", Duration.from_seconds(30)),
    ("minute", Duration.from_minutes(1)),
    ("5 minutes", Duration.from_minutes(5)),
    ("10 minutes", Duration.from_minutes(10)),
    ("15 minutes", Duration.from_minutes(15)),
    ("30 minutes", Duration.from_minutes(30)),
    ("hour", Duration.from_hours(1)),
    ("day", Duration.from_days(1)),
    ("week", Duration.from_days(7)),
    ("month", Duration.from_days(30)),
    ("year", Duration.from_days(365)),
]
