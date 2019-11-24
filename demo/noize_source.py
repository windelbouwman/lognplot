""" Test noise source, which transmits a sine wave
over TCP/IP
"""

import math
import time
import socket
import struct

import cbor


def main():
    t = 0.0
    A = 10.0  # Sine wave amplitude [-]
    F = 1.3  # Sine wave frequency [Hz]
    A2 = 1.2
    F2 = 100
    B = 5.0  # Sine wave offset
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(('127.0.0.1', 12345))

    dt = 0.0001  # 10 kHz
    n_samples = 2000
    while True:
        samples = []
        t0 = t
        # Generate samples:
        for _ in range(n_samples):
            omega = 2 * math.pi * F
            omega2 = 2 * math.pi * F2
            sample = A * math.sin(omega * t) + B + A2 * math.cos(omega2 * t)
            samples.append(sample)

            # Increment time:
            t += dt
        
        print(f'Sending {len(samples)} samples')
        send_samples(sock, t0, dt, samples)

        time.sleep(n_samples * dt)


def send_samples(sock, t0, dt, samples):
    data = bytearray()
    data2 = cbor.dumps({
        't0': t0,
        'dt': dt,
        'data': samples
    })
    data.extend(struct.pack('>I', len(data2)))
    data.extend(data2)
    # for sample in samples:
    # data.extend(struct.pack('<d', sample))
    # print(data)
    # print(len(data))
    sock.sendall(data)
    pass


if __name__ == "__main__":
    main()
