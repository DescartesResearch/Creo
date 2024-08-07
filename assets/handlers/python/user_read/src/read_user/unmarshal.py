from typing import Any
from read_user.models import User


def unmarshal_user(data: dict[str, Any]) -> User:
    return User.model_validate(data)
