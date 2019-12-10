""" Create some random data for demo purposes.
"""

import math
from .tsdb import LogLevel, LogRecord


def create_demo_samples(num_points, offset=0):
    """ Create sin wave with superposed cosine wave """
    # Parameters:
    F = 1
    A = 25.0
    omega = math.pi * 2 * F
    F2 = 50
    A2 = 3.14
    omega2 = math.pi * 2 * F2

    samples = []
    for t in range(num_points):
        x = t * 0.001
        y = offset + A * math.sin(omega * x) + A2 * math.cos(omega2 * x)
        samples.append((x, y))
    return samples


def create_demo_log_messages(num_records):
    samples = []
    for i in range(num_records):
        t = i * 3
        record = LogRecord(LogLevel.WARNING, f"Warning {i}")
        samples.append((t, record))

    return samples
