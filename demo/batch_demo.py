""" Demonstrate how to send a batch of samples.

This will dump a batch of samples into the GUI tool
via the TCP client.

"""

import random
from lognplot.client import LognplotTcpClient


def main():
    client = LognplotTcpClient()
    client.connect()

    N_batches = 1_000
    batch_size = 10_000

    timestamp = 0.0
    value = 0.0

    for _ in range(N_batches):
        samples = []
        for _ in range(batch_size):
            samples.append((timestamp, value))

            # Create next value:
            timestamp += 0.7 + random.random() * 10.0
            value += -0.5 + random.random()

        client.send_sample_batch("TEN_MEG", samples)


if __name__ == "__main__":
    main()
