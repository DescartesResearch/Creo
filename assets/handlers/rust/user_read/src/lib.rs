mod db;
pub mod models;

pub async fn read_user_by_id(id: i64) -> Option<models::User> {
    db::get_collection().await.find_one(mongodb::bson::doc! {"_id": id}).await.unwrap()
}
