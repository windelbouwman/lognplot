""" Stream audio data into lognplot gui.

Usage:

    $ arecord -c 1 -f S16_LE -r 44100 | python audio_fun.py 

"""

import argparse
import struct
import sys
from lognplot.client import LognplotTcpClient

# Assume 44100 Hz and
fs = 44100
ts = 1 / fs
fmt = "S16_LE"
fmt = "<h"  # 16 bit signed little endian
t = 0
buf_size = 2048  # Number of samples to read at once.
sample_size = struct.calcsize(fmt)

lpc = LognplotTcpClient()
lpc.connect()

while True:
    data = sys.stdin.buffer.read(buf_size * sample_size)

    print(len(data))
    samples = []
    for i in range(buf_size):
        (v,) = struct.unpack(fmt, data[i * sample_size : i * sample_size + sample_size])
        samples.append(v)
    lpc.send_samples(t, ts, samples)
    t += buf_size * ts
