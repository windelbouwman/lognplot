import time
import sys
from .client import LognplotTcpClient


def install_tracer():
    """ Install an intense application tracer.

    The tracer will track:
    - function entry / exit
    - python logging system messages

    Traced events will be forwarded to the lognplot GUI.
    """

    client = LognplotTcpClient()
    client.connect()

    def trace_hook(frame, event, arg):
        print(frame, event, arg)
        t = time.time()
        nanos = int(t * 1e9)
        # print("hooked at t=", nanos, "frame:", frame, "event", event, "arg", arg)
        client.send_text("CALLSTACK", t, f"Event {event} arg {arg}")
        client.send_sample("callstack", nanos, nanos)
        client.send_event(
            "trace_hook",
            nanos,
            {
                "event": str(event),
                "arg": str(arg)
                # 'file':
            },
        )
        if event == "call":
            return trace_hook

    # sys.settrace(trace_hook)

    def profile_hook(frame, event, arg):
        timestamp = time.time()
        print(f"profiled frame: {frame} event: {event} arg: {arg}")
        client.send_text("CALLSTACK", timestamp, f"Event {event} arg {arg}")
        if event == 'call':
            client.send_function_enter("CALLSTACK 2", timestamp, "FUU")
        elif event == 'return':
            client.send_function_exit("CALLSTACK 2", timestamp)

        # print(event)

    sys.setprofile(profile_hook)
