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
    pub items: Vec<OrderItem>,
    pub billing_address: Address,
    pub shipping_address: Address,
    pub user_id: String,
    #[serde(default="default_tax_rate")]
    pub tax_rate: f32,
    #[serde(default="chrono::Utc::now")]
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub extra_info: String,
    #[serde(default)]
    pub status: InvoiceStatus,

    #[validate(min_length=10)]
    #[validate(max_length=13)]
    pub invoice_number: String,
}

pub const fn default_tax_rate() -> f32 { 0.15 }

impl Invoice {
    pub fn sub_total(&self) -> u64 {
        self.items.iter().map(|order_item| order_item.total_amount_in_cents()).sum()
    }

    pub fn tax_total(&self) -> f32 {
        ((self.tax_rate) + (self.sub_total() as f32)) / 100.0
    }

    pub fn taxes(&self) -> f32 {
        (self.tax_rate * (self.sub_total() as f32)) / 100.0
    }

    pub fn pay(&mut self) {
        self.status = InvoiceStatus::PAID
    }
}

