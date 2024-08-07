import datetime as dt
from datetime import datetime
from typing import Literal
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


class Address(BaseModel):
    """Model for representing a postal address."""

    first_name: str = Field(
        default_factory=lambda: random_string(2, 64),
        title="First Name",
        description="The first name of the person this address belongs to.",
        min_length=2,
        max_length=64,
    )
    last_name: str = Field(
        default_factory=lambda: random_string(2, 64),
        title="Last Name",
        description="The last name of the person this address belongs to.",
        min_length=2,
        max_length=64,
    )
    street: str = Field(
        default_factory=lambda: random_string(2, 128),
        title="Street",
        description="The street name of the address.",
        min_length=2,
        max_length=128,
    )
    number: int = Field(
        default_factory=lambda: random_int(1, 2000),
        title="House Number",
        description="The house number of the address.",
        gt=0,
    )
    zip_code: int = Field(
        default_factory=lambda: random_int(1000, 99999),
        title="ZIP Code",
        description="The ZIP Code of the address",
    )
    city: str = Field(
        default_factory=lambda: random_string(3, 64),
        title="City",
        description="The city of the address",
    )
    country: str = Field(
        default_factory=lambda: random_string(3, 64),
        title="Country",
        description="The country of the address",
    )


class Item(BaseModel):
    """Model for representing a store item."""

    price_in_cents: int = Field(
        default_factory=lambda: random_int(1, 1_000_000_000),
        title="Price in Cents",
        description="The price of the item in euro cents",
        gt=0,
    )
    name: str = Field(
        default_factory=lambda: random_string(1, 128),
        title="Name",
        description="The name of the item.",
        min_length=1,
        max_length=128,
    )


class OrderItem(BaseModel):
    """Model for representing a store item in an order."""

    item: Item = Field(
        default_factory=Item, title="Item", description="The item in the order"
    )
    quantity: int = Field(
        default_factory=lambda: random_int(1, 10_000),
        title="Quantity",
        description="The quantity of the item in the order",
        gt=0,
    )


class Invoice(BaseModel):
    """Model for representing an invoice for an order."""

    items: list[OrderItem] = Field(
        default_factory=lambda: [OrderItem() for _ in range(1, random_int(1, 100))],
        title="Order Items",
        description="The order items of the invoice",
    )
    billing_address: Address = Field(
        default_factory=Address,
        title="Billing address",
        description="The billing address of the order",
    )
    shipping_address: Address = Field(
        default_factory=Address,
        title="Shipping address",
        description="The shipping address of the order",
    )
    user_id: str = Field(
        default_factory=lambda: random_string(10, 24),
        title="User ID",
        description="The user id of the user the invoice belongs to.",
    )
    tax_rate: float = Field(
        default=0.15,
        title="Tax Rate",
        description="The tax rate to use for the invoice.",
    )
    issued_at: datetime = Field(
        default_factory=lambda: datetime.now(dt.timezone.utc),
        title="Order Time",
        description="The time the order was issued at.",
    )
    extra_info: str = Field(
        default_factory=lambda: random_string(0, 512),
        title="Extra Info",
        description="Additional information about the invoice/order.",
    )
    status: Literal["OPEN", "PAID"] = Field(
        default="OPEN",
        title="Invoice Status",
        description="The payment status of the invoice",
    )
    invoice_number: str = Field(
        default_factory=lambda: random_string(10, 13),
        title="Invoice Number",
        description="The invoice number",
        min_length=10,
        max_length=13,
    )
