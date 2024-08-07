from update_invoice import db, unmarshal


async def update_invoice(id: int, json_data: bytes) -> bool:
    invoice = unmarshal.unmarshal_invoice(json_data)
    update_dict = invoice.model_dump(exclude_none=True)
    if update_dict:
        update_result = await db.collection.update_one(
            {"_id": id}, {"$set": update_dict}
        )
        return update_result.acknowledged
    return True
