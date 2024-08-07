from delete_user import db


async def delete_user_by_id(id: int) -> bool:
    """Deletes the user with the given ID."""
    delete_result = await db.collection.delete_one({"_id": id})
    return delete_result.acknowledged
