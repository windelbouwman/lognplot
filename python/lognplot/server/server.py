import asyncio
import logging
from .peer import Peer
from .connection import Connection


class Server:
    """ Server which receives time series data. """

    logger = logging.getLogger("lognplot-server")

    def __init__(self, data_sink):
        self.peers = []
        self.data_sink = data_sink

    async def serve(self):
        self.logger.info("Booting server.")
        server = await asyncio.start_server(
            self._on_client, host="localhost", port=12345
        )
        addr = server.sockets[0].getsockname()
        self.logger.info(f"Accepting connections on {addr}")
        async with server:
            await server.serve_forever()

    async def _on_client(self, reader, writer):
        self.logger.info("New client!")
        connection = Connection(reader, writer)
        peer = Peer(connection, self.data_sink)
        self.peers.append(peer)
        await peer.run()
        self.peers.remove(peer)
        await writer.drain()
        writer.close()
