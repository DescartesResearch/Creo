mod db;


pub async fn delete_invoice_by_id(id: i64) -> u64 {
    let delete_result = db::get_collection().await.delete_one(mongodb::bson::doc!{ "_id": id}).await.unwrap();
    delete_result.deleted_count
}
