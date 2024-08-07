from create_user import db, unmarshal


async def create_user(json_data: bytes) -> str:
    user = unmarshal.unmarshal_user(json_data)
    insert_dict = user.model_dump()
    insert_result = await db.collection.insert_one(insert_dict)
    return str(insert_result.inserted_id)
