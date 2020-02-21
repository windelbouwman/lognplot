""" Demo of a slow updating signal.

"""


import math
import time

from lognplot.client import LognplotTcpClient


def main():
    t = time.time()
    dt = 2.0
    client = LognplotTcpClient()
    client.connect()
    while True:
        print(f'Sending at t={t}')
        for x in range(20):
            client.send_sample(f"Trace_{x}", t, (t + x) % 36)
        time.sleep(dt)
        t += dt


if __name__ == "__main__":
    main()
