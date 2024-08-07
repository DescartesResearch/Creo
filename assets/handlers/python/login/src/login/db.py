import os
import threading
import asyncio

import motor.motor_asyncio as motor

_HOST = os.getenv("DB_MONGO_HOST")
_PORT = os.getenv("DB_MONGO_PORT", "")
_USER = os.getenv("DB_MONGO_USER")
_PASSWORD = os.getenv("DB_MONGO_PASSWORD")

_db_client = motor.AsyncIOMotorClient(
    host=_HOST,
    port=int(_PORT),
    username=_USER,
    password=_PASSWORD,
    uuidRepresentation="standard",
)

_login_db = _db_client.login_db
collection = _login_db.login_collection


async def create_indexes():
    client = motor.AsyncIOMotorClient(
        host=_HOST,
        port=int(_PORT),
        username=_USER,
        password=_PASSWORD,
        uuidRepresentation="standard",
    )
    db = client.login_db
    col = db.login_collection
    await asyncio.gather(
        col.create_index("username"),
        col.create_index("email"),
    )


def setup():
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    try:
        loop.run_until_complete(create_indexes())
    finally:
        loop.close()


t = threading.Thread(target=setup)
t.start()
t.join()
