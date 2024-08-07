from create_invoice import db, unmarshal


async def create_invoice(json_data: bytes) -> str:
    invoice = unmarshal.unmarshal_invoice(json_data)
    insert_dict = invoice.model_dump()
    insert_result = await db.collection.insert_one(insert_dict)
    return str(insert_result.inserted_id)
