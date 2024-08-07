import datetime as dt
from datetime import datetime
import random
import string

from pydantic import BaseModel, Field


def random_string(min_length: int, max_length: int) -> str:
    if min_length > max_length:
        raise ValueError("min_length must not be greater than max_length")
    return "".join(
        random.choices(string.ascii_letters, k=random.randint(min_length, max_length))
    )


def random_int(min: int, max: int) -> int:
    if min > max:
        raise ValueError("min must not be greater than max")
    return random.randint(min, max)


class User(BaseModel):
    """Model for representing a user."""

    username: str = Field(
        default_factory=lambda: random_string(3, 64),
        title="Username",
        description="The name of the user.",
        min_length=3,
        max_length=64,
    )
    email: str = Field(
        default_factory=lambda: random_string(3, 64),
        title="Email",
        description="The email of the user.",
        min_length=3,
        max_length=64,
    )
    password_hash: bytes = Field(
        default_factory=lambda: random_string(97, 97).encode("utf-8"),
        title="Password",
        alias="password",
        description="The password hash of the user.",
        min_length=32,
        max_length=128,
    )
    created_at: datetime = Field(
        default_factory=lambda: datetime.now(dt.timezone.utc),
        title="Created at",
        description="The time the user was created at.",
    )
