""" Implement client connection to lognplot protocol.

"""

import socket
import struct
import datetime
import time
import cbor


class LognplotTcpClient:
    """ Use this client to transmit sample to the lognplot tool.
    """

    def __init__(self, hostname="localhost", port=12345):
        self._hostname = hostname
        self._port = port

    def connect(self):
        """ Connect to the server.
        """
        self._sock = socket.create_connection((self._hostname, self._port))

    def send_sample(self, name: str, timestamp, value: float):
        """ Send a single timestamp / value pair to the trace with the given name.
        """
        timestamp = coerce_timestamp(timestamp)
        value = float(value)
        self._send_dict(
            {"name": name, "t": timestamp, "type": "sample", "value": value}
        )

    def send_sample_batch(self, name: str, samples):
        """ Send a batch of samples.

        samples is a list of tuples of what you would pass to send_sample.

        """
        self._send_dict(
            {"type": "batch", "name": name, "batch": samples,}
        )

    def send_samples(self, name: str, timestamp, dt: float, samples):
        """ Send equidistant spaced samples.
        """

        timestamp = coerce_timestamp(timestamp)

        self._send_dict(
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
        timestamp = coerce_timestamp(timestamp)
        self._send_dict(
            {"name": name, "t": timestamp, "type": "event", "attributes": attributes}
        )

    def _send_dict(self, data):
        data2 = cbor.dumps(data)
        self._send_message(data2)

    def _send_message(self, msg_data):
        """ Transmit a whole message prefixed with a length.
        """
        data = bytearray()
        data.extend(struct.pack(">I", len(msg_data)))
        data.extend(msg_data)
        self._sock.sendall(data)


def coerce_timestamp(timestamp) -> float:
    """ Convert a timestamp into a floating point number. """
    if isinstance(timestamp, datetime.datetime):
        return time.mktime(timestamp.timetuple()) + timestamp.microsecond / 1e6
    else:
        return float(timestamp)
