""" Test noise source, which transmits a sine wave
over TCP/IP
"""

import math
import time
import socket


def main():
    t = 0.0
    A = 10.0  # Sine wave amplitude [-]
    F = 1.3  # Sine wave frequency [Hz]
    B = 5.0  # Sine wave offset
    # dt = 
    while True:
        samples = []
        # Generate samples:
        for _ in range(200):
            omega = 2 * math.pi * F
            sample = A * math.sin(omega * t) + B
            samples.append(sample)

            # Increment time:
            t += 0.001
        
        print(f'Sending {len(samples)} samples')

        time.sleep(0.2)


def send_samples(samples):
    pass


if __name__ == "__main__":
    main()
