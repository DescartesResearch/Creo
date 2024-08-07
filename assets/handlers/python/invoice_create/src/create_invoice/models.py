import datetime as dt
from datetime import datetime
from typing import Literal

from pydantic import BaseModel, Field


class Address(BaseModel):
    """Model for representing a postal address."""

    first_name: str = Field(
        default=...,
        title="First Name",
        description="The first name of the person this address belongs to.",
        min_length=2,
        max_length=64,
    )
    last_name: str = Field(
        default=...,
        title="Last Name",
        description="The last name of the person this address belongs to.",
        min_length=2,
        max_length=64,
    )
    street: str = Field(
        default=...,
        title="Street",
        description="The street name of the address.",
        min_length=2,
        max_length=128,
    )
    number: int = Field(
        default=...,
        title="House Number",
        description="The house number of the address.",
        gt=0,
    )
    zip_code: int = Field(
        default=...,
        title="ZIP Code",
        description="The ZIP Code of the address",
    )
    city: str = Field(
        default=...,
        title="City",
        description="The city of the address",
    )
    country: str = Field(
        default=...,
        title="Country",
        description="The country of the address",
    )


class Item(BaseModel):
    """Model for representing a store item."""

    price_in_cents: int = Field(
        default=...,
        title="Price in Cents",
        description="The price of the item in euro cents",
        gt=0,
    )
    name: str = Field(
        default=...,
        title="Name",
        description="The name of the item.",
        min_length=1,
        max_length=128,
    )


class OrderItem(BaseModel):
    """Model for representing a store item in an order."""

    item: Item = Field(default=..., title="Item", description="The item in the order")
    quantity: int = Field(
        default=...,
        title="Quantity",
        description="The quantity of the item in the order",
        gt=0,
    )

    @property
    def total_amount_in_cents(self) -> int:
        return self.item.price_in_cents * self.quantity


class Invoice(BaseModel):
    """Model for representing an invoice for an order."""

    items: list[OrderItem] = Field(
        default=..., title="Order Items", description="The order items of the invoice"
    )
    billing_address: Address = Field(
        default=...,
        title="Billing address",
        description="The billing address of the order",
    )
    shipping_address: Address = Field(
        default=...,
        title="Shipping address",
        description="The shipping address of the order",
    )
    user_id: str = Field(
        default=...,
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
        default="",
        title="Extra Info",
        description="Additional information about the invoice/order.",
    )
    status: Literal["OPEN", "PAID"] = Field(
        default="OPEN",
        title="Invoice Status",
        description="The payment status of the invoice",
    )
    invoice_number: str = Field(
        default=...,
        title="Invoice Number",
        description="The invoice number",
        min_length=10,
        max_length=13,
    )

    @property
    def sub_total(self) -> int:
        return sum(map(lambda order_item: order_item.total_amount_in_cents, self.items))

    @property
    def tax_total(self) -> float:
        return ((self.tax_rate) + self.sub_total) / 100

    @property
    def taxes(self) -> float:
        return (self.tax_rate * self.sub_total) / 100

    def pay(self) -> None:
        self.status = "PAID"
