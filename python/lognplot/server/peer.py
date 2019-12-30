import logging
import cbor


class Peer:
    """ Single connection on the server side. """

    logger = logging.getLogger("peer")

    def __init__(self, connection, data_sink):
        self.connection = connection
        self._running = False
        self.name = "<unnamed>"
        self.data_sink = data_sink

    async def run(self):
        self._running = True
        while self._running:
            msg = await self.connection.read_msg()
            if msg:
                await self._handle_msg(msg)
            else:
                self._running = False

    async def _handle_msg(self, data):
        msg = cbor.loads(data)
        samples = []
        name = msg["name"]
        typ = msg["type"]
        if typ == "samples":
            t, dt = msg["t"], msg["dt"]
            for value in msg["values"]:
                value = float(value)
                samples.append((t, value))
                t += dt
            self.data_sink.add_samples(name, samples)
        elif typ == "sample":
            t, value = msg["t"], msg["value"]
            value = float(value)
            samples = [(t, value)]
            self.data_sink.add_samples(name, samples)
        elif typ == "batch":
            samples = msg["batch"]
            samples = [(t, float(v)) for t, v in samples]
            self.data_sink.add_samples(name, samples)
        elif typ == "event":
            t, attributes = msg["t"], msg["attributes"]
            samples = [(t, attributes)]
            self.data_sink.add_samples(name, samples)
        else:
            self.logger.error(f"Unknown event type {typ}")
