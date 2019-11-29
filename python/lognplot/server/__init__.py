import asyncio
from .server import Server


def run_server(db):
    server = Server(db)
    asyncio.run(server.serve())


def start_server():
    raise NotImplementedError()
