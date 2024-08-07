from typing import Optional
from login import db
from login.models import User


async def _read_user_by_key(key: str, value: str) -> Optional[User]:
    find_dict = await db.collection.find_one({key: value})

    if find_dict is None:
        return None

    find_dict["id"] = find_dict.pop("_id")
    return User.model_validate(find_dict)


async def read_user_by_username(username: str) -> Optional[User]:
    return await _read_user_by_key("username", username)


async def read_user_by_email(email: str) -> Optional[User]:
    return await _read_user_by_key("email", email)
