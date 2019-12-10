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

    def send_sample(self, name: str, timestamp, value: float):
        self.send_dict({"name": name, "t": timestamp, "type": "sample", "value": value})

    def send_samples(self, name: str, timestamp, dt, samples):
        """ Send spaced samples.
        """
        self.send_dict(
            {
                "name": name,
                "t": timestamp,
                "type": "samples",
                "dt": dt,
                "values": samples,
            }
        )

    def send_event(self, name, timestamp, attributes):
        """ Emit an event.
        
        Attributes can be given as a dictionary of key/value strings.
        """
        self.send_dict(
            {"name": name, "t": timestamp, "type": "event", "attributes": attributes}
        )

    def send_dict(self, data):
        data2 = cbor.dumps(data)
        self.send_message(data2)

    def send_message(self, msg_data):
        """ Transmit a whole message prefixed with a length.
        """
        data = bytearray()
        data.extend(struct.pack(">I", len(msg_data)))
        data.extend(msg_data)
        self._sock.sendall(data)
