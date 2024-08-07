from typing import Any, Optional

from update_user.hash import hash_password
from pydantic import BaseModel, Field, field_validator


class User(BaseModel):
    """Model for representing a user."""

    username: Optional[str] = Field(
        default=None,
        title="Username",
        description="The name of the user.",
        min_length=3,
        max_length=64,
    )
    email: Optional[str] = Field(
        default=None,
        title="Email",
        description="The email of the user.",
        min_length=3,
        max_length=64,
    )
    password_hash: Optional[bytes] = Field(
        default=None,
        title="Password",
        alias="password",
        description="The password hash of the user.",
        min_length=32,
        max_length=128,
    )

    @field_validator("password_hash", mode="before")
    @classmethod
    def hash_password(cls, v: Any) -> Optional[bytes]:
        if v is None:
            return None
        if isinstance(v, str):
            return hash_password(v).encode("utf-8")
        raise ValueError("expected type string")
