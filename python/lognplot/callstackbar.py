""" Example call stack plot like a flame graph.

Idea from: https://www.speedscope.app/
"""

from .btree import Btree


class CallStackBar:
    def __init__(self):
        self._tree = BTree()
        self._levels = []

    def enter_function(self, t, func):
        self._levels.append(func)
        event = (t, "O", func)
        self.append_event(event)

    def leave_function(self, t, func):
        top = self._levels.pop()
        assert top == func
        event = (t, "C", func)
        self.append_event(event)

    def append_event(self, event):
        # self._tree.append(event)
        pass


class FunctionCall:
    def __init__(self):
        self.t_enter = None
        self.t_leave = None
        self.child_calls = []
