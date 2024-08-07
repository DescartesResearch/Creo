from update_user import db, unmarshal


async def update_user_by_id(id: int, json_data: bytes) -> bool:
    user = unmarshal.unmarshal_user(json_data)
    update_dict = user.model_dump(exclude_none=True)
    if update_dict:
        update_result = await db.collection.update_one(
            {"_id": id}, {"$set": update_dict}
        )
        return update_result.acknowledged

    return True
