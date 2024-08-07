#[derive(Debug, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Address {
    #[validate(min_length = 2)]
    #[validate(max_length = 64)]
    pub first_name: String,

    #[validate(min_length = 2)]
    #[validate(max_length = 64)]
    pub last_name: String,

    #[validate(min_length = 2)]
    #[validate(max_length = 128)]
    pub street: String,

    #[validate(minimum=1)]
    pub number: u32,

    pub zip_code: u32,

    pub city: String,

    pub country: String,
}


#[derive(Debug, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Item {
    #[validate(minimum=1)]
    pub price_in_cents: u64,

    #[validate(min_length=1)]
    #[validate(max_length=128)]
    pub name: String,
}


#[derive(Debug, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct OrderItem {
    pub item: Item,

    #[validate(minimum=1)]
    pub quantity: u64,
}

impl OrderItem {
    pub fn total_amount_in_cents(&self) -> u64 {
        self.item.price_in_cents * self.quantity
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum InvoiceStatus {
    OPEN,
    PAID
}

impl std::default::Default for InvoiceStatus {
    fn default() -> Self {
        Self::OPEN
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize, serde_valid::Validate)]
pub struct Invoice {
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub items: Option<Vec<OrderItem>>,
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub billing_address: Option<Address>,
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub shipping_address: Option<Address>,
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub tax_rate: Option<f32>,
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub extra_info: Option<String>,
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub status: Option<InvoiceStatus>,
}
