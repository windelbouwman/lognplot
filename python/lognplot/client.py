""" Implement client connection to lognplot protocol.

"""

import socket
import struct

import cbor


class LognplotTcpClient:
    """ Use this client to transmit sample to the lognplot tool.
    """

    def __init__(self, hostname="127.0.0.1", port=12345):
        self._hostname = hostname
        self._port = port

    def connect(self):
        """ Connect to the server.
        """
        self._sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._sock.connect((self._hostname, self._port))

    def send_sample(self, timestamp, value):
        self.send_samples(timestamp, 1.0, [value])

    def send_samples(self, t0, dt, samples):
        data2 = cbor.dumps({"t0": t0, "dt": dt, "data": samples})
        self.send_message(data2)

    def send_message(self, msg_data):
        """ Transmit a whole message prefixed with a length.
        """
        data = bytearray()
        data.extend(struct.pack(">I", len(msg_data)))
        data.extend(msg_data)
        self._sock.sendall(data)
