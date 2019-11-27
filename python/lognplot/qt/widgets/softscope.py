""" A complete softscope widget.

Include this into your application to view signals.
"""

import queue

from PyQt5.QtWidgets import QWidget, QHBoxLayout, QListView, QSplitter
from PyQt5.QtCore import QTimer

from ...tsdb import TsDb
from .chartwidget import ChartWidget
from .signal_list import SignalListModel


class SoftScope(QWidget):
    """ A complete softscope widget.

    Include this into your application to view signals.
    """

    def __init__(self):
        super().__init__()
        self.db = TsDb()

        # Child widgets:
        self._signal_view = QListView()
        self._chart_widget = ChartWidget(self.db)
        self._signal_list_model = SignalListModel(self.db)
        self._signal_view.setModel(self._signal_list_model)
        self._signal_view.setDragEnabled(True)

        # Layouting:
        splitter = QSplitter()
        splitter.addWidget(self._signal_view)
        splitter.addWidget(self._chart_widget)
        l = QHBoxLayout()
        l.addWidget(splitter)
        self.setLayout(l)

        self._rx_queue = queue.Queue()
        self._timer = QTimer()
        self._timer.timeout.connect(self._on_timeout)
        self._timer.start(50)

    def add_samples(self, channel, samples):
        """ Call this function with new data.

        Data is stored into a queue, so this is presumably thread safe.
        """
        self._rx_queue.put((channel, samples))

    def add_curve(self, name, color):
        self._chart_widget.add_curve(name, color)

    def _on_timeout(self):
        if not self._rx_queue.empty():
            while not self._rx_queue.empty():
                chunk = self._rx_queue.get()
                name, samples = chunk
                self.db.add_samples(name, samples)
                self._rx_queue.task_done()
            self._chart_widget.update()

        # Hmm, ugly polling?
        self._signal_list_model.update()
