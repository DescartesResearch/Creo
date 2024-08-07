mod db;
mod models;
mod hash;
mod unmarshal;

pub async fn create_user(json_data: impl AsRef<[u8]>) -> String {
    let user = unmarshal::unmarshal_user(json_data);
    let insert_result = db::get_collection().await.insert_one(user).await.unwrap();
    insert_result.inserted_id.to_string()
}
