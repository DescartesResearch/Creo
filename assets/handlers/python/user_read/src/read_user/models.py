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
    created_at: datetime = Field(
        default=...,
        title="Created at",
        description="The time the user was created at.",
    )

    @field_serializer("created_at")
    def serialize_dt(created_at: datetime) -> int:
        return int(created_at.timestamp())
