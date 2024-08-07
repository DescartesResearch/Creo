mod db;
mod models;
use lazy_static::lazy_static;

lazy_static!{
    static ref SEED_COUNT: usize = std::env::var("MG_SEED_COUNT").unwrap_or("0".into()).parse().unwrap_or(0);
}
const BATCH_SIZE: usize = 50000;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let collection = db::get_collection().await;
    let mut range = (1..*SEED_COUNT+1).peekable();
    let mut invoices = Vec::with_capacity(BATCH_SIZE);
    while range.peek().is_some() {
        invoices.clear();
        for id in range.by_ref().take(BATCH_SIZE) {
            invoices.push(models::Invoice::new(id as i64));
        }
        collection.insert_many(&invoices).await?;
    }

    Ok(())
}
