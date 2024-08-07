mod db;
mod unmarshal;
pub mod models;

pub async fn create_invoice(json_data: impl AsRef<[u8]>) -> String {
    let invoice = unmarshal::unmarshal_invoice(json_data);
    let insert_result = db::get_collection().await.insert_one(invoice).await.unwrap();
    insert_result.inserted_id.to_string()
}
