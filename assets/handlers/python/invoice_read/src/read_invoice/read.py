from typing import Optional, Any
from read_invoice import db, unmarshal


async def read_invoice_by_id(id: int) -> Optional[dict[str, Any]]:
    find_dict = await db.collection.find_one({"_id": id})
    if find_dict is None:
        return None
    find_dict["id"] = str(find_dict.pop("_id"))
    return unmarshal.unmarshal_invoice(find_dict).model_dump()
