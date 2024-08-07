import re
from typing import Any, Optional

import argon2

from login import cache
from login.read import read_user_by_username, read_user_by_email

TIME_COST: int = 1
MEMORY_COST: int = 6144
SALT_LEN: int = 16

hasher = argon2.PasswordHasher(
    time_cost=TIME_COST,
    memory_cost=MEMORY_COST,
    salt_len=SALT_LEN,
)

EMAIL_REGEX = re.compile(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)")


async def login_with_username_or_email(
    username_or_email: str,
    password: str,
) -> Optional[dict[str, Any]]:
    user = (
        await read_user_by_email(username_or_email)
        if EMAIL_REGEX.fullmatch(username_or_email)
        else await read_user_by_username(username_or_email)
    )
    if user is not None and hasher.verify(hash=user.password_hash, password=password):
        repo = cache.get_session_repository()
        session = repo.set_new_session(user.id)
        return session.model_dump()
    return None
