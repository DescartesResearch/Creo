use rand::distributions::{uniform::{SampleRange, SampleUniform}, DistString};

fn random_string(min_length: usize, max_length: usize) -> String {
    let mut rng = rand::thread_rng();
    let length = (min_length..max_length).sample_single(&mut rng);
    rand::distributions::Alphanumeric.sample_string(&mut rng, length)
}

fn random_int<T: PartialOrd + SampleUniform>(min: T, max: T) -> T {
    let mut rng = rand::thread_rng();
    (min..max).sample_single(&mut rng)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Address {
    pub first_name: String,
    pub last_name: String,
    pub street: String,
    pub number: u32,
    pub zip_code: u32,
    pub city: String,
    pub country: String,
}

impl Default for Address {
    fn default() -> Self {
        Self {
            first_name: random_string(2, 64),
            last_name: random_string(2, 64),
            street: random_string(2, 128),
            number: random_int(1, 2000),
            zip_code: random_int(1000, 99999),
            city: random_string(3, 64),
            country: random_string(3, 64),
        }
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub price_in_cents: u64,

    pub name: String,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            price_in_cents: random_int(1, 1000000000),
            name: random_string(1, 128),
        }
    }

}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderItem {
    pub item: Item,
    pub quantity: u64,
}

impl Default for OrderItem {
    fn default() -> Self {
        Self {
            item: Default::default(),
            quantity: random_int(1, 10000),
        }
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


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Invoice {
    #[serde(rename="_id")]
    pub id: i64,
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
    pub invoice_number: String,
}

pub const fn default_tax_rate() -> f32 { 0.15 }

impl Invoice {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            items: {
                let length = random_int(1, 100);
                (0..length).map(|_| OrderItem::default()).collect()
            },
            billing_address: Address::default(),
            shipping_address: Address::default(),
            user_id: random_string(10, 24),
            tax_rate: default_tax_rate(),
            issued_at: chrono::Utc::now(),
            extra_info: random_string(0, 512),
            status: InvoiceStatus::default(),
            invoice_number: random_string(10, 13),
        }
    }
}


