from typing import Optional

from register.read import read_user_by_username, read_user_by_email
from register.create import create_user
from register import unmarshal


async def register_user(json_data: bytes) -> Optional[str]:
    user = unmarshal.unmarshal_user(json_data)
    if await read_user_by_email(user.email) is not None:
        return None
    if await read_user_by_username(user.username) is not None:
        return None
    user_id = await create_user(user)
    return user_id
