mod db;
mod hash;
pub mod models;
mod unmarshal;

pub async fn update_user_by_id(id: i64, json_data: impl AsRef<[u8]>) -> u64 {
    let user = unmarshal::unmarshal_user(json_data);

    let update_doc = mongodb::bson::to_document(&user).unwrap();

    if update_doc.is_empty() {
        return 0;
    }
    let update_result = db::get_collection().await.update_one(
        mongodb::bson::doc! {
            "_id": id
        },
        mongodb::bson::doc! {
            "$set": update_doc
        }
    ).await.unwrap();

    update_result.modified_count
}
