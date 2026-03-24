"""libstored adapter.

Connects to a libstored ZeroMQ debugger endpoint, polls numeric objects,
and forwards them to lognplot.

Example usage:
    python libstored_adapter.py --libstored-host localhost --libstored-port 19026
"""

import argparse
import re
import time

from libstored import ZmqClient
from lognplot.client import LognplotTcpClient


def parse_args():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        "--libstored-host",
        default="localhost",
        type=str,
        help="Hostname of the libstored ZeroMQ server.",
    )
    parser.add_argument(
        "--libstored-port",
        default=19026,
        type=int,
        help="Port of the libstored ZeroMQ server.",
    )
    parser.add_argument(
        "--poll-interval",
        default=0.1,
        type=float,
        help="Polling interval in seconds.",
    )
    parser.add_argument(
        "--include-regex",
        default=r".*",
        type=str,
        help="Only include object names matching this regex.",
    )
    parser.add_argument(
        "--exclude-regex",
        default=None,
        type=str,
        help="Exclude object names matching this regex.",
    )
    parser.add_argument(
        "--signal-prefix",
        default="/libstored",
        type=str,
        help="Prefix to prepend to forwarded signal names.",
    )
    parser.add_argument("--lognplot-hostname", default="127.0.0.1", type=str)
    parser.add_argument("--lognplot-port", default=12345, type=int)
    return parser.parse_args()


def is_numeric_object(obj):
    if obj.is_function():
        return False
    return obj.value_type in (int, float, bool)


def should_forward(obj_name, include_pattern, exclude_pattern):
    if include_pattern.match(obj_name) is None:
        return False
    if exclude_pattern is not None and exclude_pattern.search(obj_name) is not None:
        return False
    return True


def build_signal_name(prefix, obj_name):
    if prefix.endswith("/"):
        prefix = prefix[:-1]
    return f"{prefix}{obj_name}"


def main():
    args = parse_args()

    include_pattern = re.compile(args.include_regex)
    exclude_pattern = re.compile(args.exclude_regex) if args.exclude_regex else None

    lognplot_client = LognplotTcpClient(
        hostname=args.lognplot_hostname, port=args.lognplot_port
    )
    lognplot_client.connect()

    print(
        f"Connecting to libstored at tcp://{args.libstored_host}:{args.libstored_port}"
    )
    with ZmqClient(args.libstored_host, args.libstored_port, multi=True) as libstored:
        objects = libstored.list(sync=True)
        objects = [
            obj
            for obj in objects
            if is_numeric_object(obj)
            and should_forward(obj.name, include_pattern, exclude_pattern)
        ]

        print(f"Forwarding {len(objects)} object(s) to lognplot")

        while True:
            cycle_start = time.time()

            for obj in objects:
                try:
                    value = obj.read(sync=True)
                except Exception:
                    continue

                if value is None:
                    continue

                signal_name = build_signal_name(args.signal_prefix, obj.name)
                lognplot_client.send_sample(signal_name, cycle_start, float(value))

            elapsed = time.time() - cycle_start
            sleep_time = args.poll_interval - elapsed
            if sleep_time > 0:
                time.sleep(sleep_time)


if __name__ == "__main__":
    main()
