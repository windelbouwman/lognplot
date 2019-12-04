import asyncio
from .server import Server


def run_server(db):
    """ Start an asyncio server filling the given database. """
    server = Server(db)
    asyncio.run(server.serve())


def start_server():
    raise NotImplementedError()
