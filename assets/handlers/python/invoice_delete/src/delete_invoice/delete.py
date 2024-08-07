from delete_invoice import db


async def delete_invoice_by_id(id: int) -> bool:
    delete_result = await db.collection.delete_one({"_id": id})
    return delete_result.acknowledged
