""" Demo of a many different signals.

This is a stress test on the list of signals on the left panel.

"""


import math
import time

from lognplot.client import LognplotTcpClient


def main():
    t = time.time()
    dt = 0.1
    client = LognplotTcpClient()
    client.connect()
    while True:
        print(f'Sending at t={t}')
        for x in range(2000):
            client.send_sample(f"Trace_{x}", t, (t + x) % 36)
        time.sleep(dt)
        t += dt


if __name__ == "__main__":
    main()
