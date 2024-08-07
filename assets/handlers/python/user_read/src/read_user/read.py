from typing import Any, Optional
from read_user import db, unmarshal


async def read_user_by_id(id: int) -> Optional[dict[str, Any]]:
    find_dict = await db.collection.find_one({"_id": id})

    if find_dict is None:
        return None

    find_dict["id"] = str(find_dict.pop("_id"))
    return unmarshal.unmarshal_user(find_dict).model_dump()
