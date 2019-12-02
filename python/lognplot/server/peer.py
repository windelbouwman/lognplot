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
            await self._handle_msg(msg)

    async def _handle_msg(self, data):
        msg = cbor.loads(data)
        samples = []
        name = msg["name"]
        t, dt = msg["t0"], msg["dt"]
        for value in msg["data"]:
            samples.append((t, value))
            t += dt
        self.data_sink.add_samples(name, samples)
