""" Test noise source, which transmits a sine wave
over TCP/IP
"""

import math
import time
import random

from lognplot.client import LognplotTcpClient


def main():
    t = 0.0
    A = 10.0  # Sine wave amplitude [-]
    F = 1.3  # Sine wave frequency [Hz]
    A2 = 1.2
    F2 = 100
    B = 5.0  # Sine wave offset
    sigma_delta_step = 0.3
    sigma_delta_value = 0
    client = LognplotTcpClient()
    client.connect()

    dt = 0.0001  # 10 kHz
    n_samples = 2000
    while True:
        samples = []
        samples2 = []
        samples3 = []
        samples4 = []

        t0 = t
        # Generate samples:
        for _ in range(n_samples):
            omega = 2 * math.pi * F
            omega2 = 2 * math.pi * F2
            sample = A * math.sin(omega * t) + B + A2 * math.cos(omega2 * t)
            sample2 = A * math.sin(omega * t) + B + A2 * math.cos(omega2 * t) + 9

            # Track sample with binary output:
            if sigma_delta_value < sample:
                sample3 = 1.0
                sigma_delta_value += sigma_delta_step
            else:
                sample3 = 0.0
                sigma_delta_value -= sigma_delta_step

            samples.append(sample)
            samples2.append(sample2)
            samples3.append(sample3)
            samples4.append(float(random.randint(0, 1)))

            # Increment time:
            t += dt
        
        gen = random_bits()
        samples5 = [float(next(gen)) for _ in range(n_samples)]

        print(f"Sending {len(samples)} samples")
        client.send_samples("Trace1", t0, dt, samples)
        client.send_samples("Trace2", t0, dt, samples2)
        client.send_samples("SIGMA-DELTA", t0, dt, samples3)
        client.send_samples("RANDOM", t0, dt, samples4)
        client.send_samples("TEXT_BITS", t0, dt, samples5)

        time.sleep(n_samples * dt)


def random_bits():
    """ Generate some bits based on the docstring of this script.
    """
    while True:
        for char in __doc__:
            bits = ord(char)
            for n in range(8):
                yield bool((1 << n) & bits)


if __name__ == "__main__":
    main()
