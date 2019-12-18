""" Layer over raw sockets providing messlength prefixed messages.
"""

import logging
import asyncio
import struct


class Connection:
    """ Message passing connection. """

    logger = logging.getLogger("lognplot-connection")

    def __init__(self, reader, writer):
        self.reader = reader
        self.writer = writer

    async def write_msg(self, data):
        self.logger.debug(f"Packing {len(data)} bytes")
        header = struct.pack(">I", len(data))
        await self.write_bytes(header + data)

    async def write_bytes(self, data):
        self.writer.write(data)
        await self.writer.drain()

    async def read_msg(self):
        try:
            header = await self.read_bytes(4)
            assert len(header) == 4
            (length,) = struct.unpack(">I", header)
            self.logger.debug(f"Reading message of {length} bytes")
            data = await self.read_bytes(length)
            self.logger.debug(f"Read {len(data)} bytes")
            assert len(data) == length
            return data
        except asyncio.IncompleteReadError as ex:
            self.logger.info("Incomplete data, assuming peer disconnect.")

    async def read_bytes(self, amount):
        return await self.reader.readexactly(amount)
