import datetime as dt
from datetime import datetime
from typing import Any

from create_user.hash import hash_password
from pydantic import BaseModel, Field, field_validator


class User(BaseModel):
    """Model for representing a user."""

    username: str = Field(
        default=...,
        title="Username",
        description="The name of the user.",
        min_length=3,
        max_length=64,
    )
    email: str = Field(
        default=...,
        title="Email",
        description="The email of the user.",
        min_length=3,
        max_length=64,
    )
    password_hash: bytes = Field(
        default=...,
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

    @field_validator("password_hash", mode="before")
    @classmethod
    def hash_password(cls, v: Any) -> bytes:
        if isinstance(v, str):
            return hash_password(v).encode("utf-8")
        raise ValueError("expected type string")
