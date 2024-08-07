from typing import Literal, Optional

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


class UpdateInvoice(BaseModel):
    """Model for representing an invoice for an order."""

    items: Optional[list[OrderItem]] = Field(
        default=None, title="Order Items", description="The order items of the invoice"
    )
    billing_address: Optional[Address] = Field(
        default=None,
        title="Billing address",
        description="The billing address of the order",
    )
    shipping_address: Optional[Address] = Field(
        default=None,
        title="Shipping address",
        description="The shipping address of the order",
    )
    tax_rate: Optional[float] = Field(
        default=None,
        title="Tax Rate",
        description="The tax rate to use for the invoice.",
    )
    extra_info: Optional[str] = Field(
        default=None,
        title="Extra Info",
        description="Additional information about the invoice/order.",
    )
    status: Optional[Literal["OPEN", "PAID"]] = Field(
        default=None,
        title="Invoice Status",
        description="The payment status of the invoice",
    )

    def pay(self) -> None:
        self.status = "PAID"
