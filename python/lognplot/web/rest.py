""" REST api.

"""

from aiohttp import web
import json


async def handle(request):
    db = request.app["tsdb"]
    woot = {
        "Moi!": 1337,
        "Fubar!": 34,
        "signal_names": db.signal_names(),
    }
    return web.json_response(woot)


def serve_db_via_rest(db):
    app = web.Application()
    app["tsdb"] = db
    app.router.add_get("/", handle)
    web.run_app(app)


def main():
    from ..tsdb import TsDb

    db = TsDb()
    serve_db_via_rest(db)


if __name__ == "__main__":
    main()
