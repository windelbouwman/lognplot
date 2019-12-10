""" Demo showing how to send trace a python snippet!

Register trace function.
Capture timestamps of events.
Transfer events over tha wire.
"""

import sys
import time
import inspect
from lognplot.client import LognplotTcpClient

client = LognplotTcpClient()
client.connect()


def trace_hook(frame, event, arg):
    t = time.time()
    nanos = int(t * 1e9)
    print("hooked at t=", nanos, "frame:", frame, "event", event, "arg", arg)
    client.send_sample('callstack', nanos, nanos)
    client.send_event('trace_hook', nanos, {
        'event': str(event),
        'arg': str(arg)
        # 'file': 
    })
    if event == "call":
        return trace_hook


sys.settrace(trace_hook)
# sys.set


def do_spiffy(x):
    for y in [1, 2, 3]:
        print(y, x)


l = ["a", "bla", "bla-di-bla"]
for t in l:
    do_spiffy(t)
