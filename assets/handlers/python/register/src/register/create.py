from register import db
from register.models import CreateUser


async def create_user(user: CreateUser) -> str:
    insert_dict = user.model_dump()
    insert_result = await db.collection.insert_one(insert_dict)
    return str(insert_result.inserted_id)
