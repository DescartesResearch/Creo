import datetime as dt
from datetime import datetime

from pydantic import BaseModel, Field, field_serializer


class User(BaseModel):
    """Model for representing a user."""

    id: str = Field(
        default=...,
        title="ID",
        description="The ID of the user.",
    )
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
        default=...,
        title="Created at",
        description="The time the user was created at.",
    )


class SessionData(BaseModel):
    user_id: str = Field(
        default=...,
        title="User ID",
        description="The user ID the session belongs to",
    )
    session_id: str = Field(
        default=...,
        title="Session ID",
        description="The ID of the session",
    )
    exp: datetime = Field(
        default_factory=lambda: datetime.now(dt.timezone.utc) + dt.timedelta(days=3),
        title="Expiry",
        description="The expiry time of the session",
    )


class SessionResponse(BaseModel):
    token: str = Field(
        default=...,
        title="Session Token",
        description="The session token, i.e. ID",
    )
    exp: datetime = Field(
        default=...,
        title="Expiry",
        description="The expiry time of the session",
    )

    @field_serializer("exp")
    def serialize_dt(exp: datetime) -> int:
        return int(exp.timestamp())
