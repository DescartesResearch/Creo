import os

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
