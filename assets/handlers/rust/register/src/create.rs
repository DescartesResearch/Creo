use crate::{db, models};

pub async fn create_user(user: models::CreateUser) -> models::User {
    let collection = db::get_collection().await;
    let insert_result = collection.clone_with_type::<models::CreateUser>().insert_one(user).await.unwrap();
    let user = collection.find_one(mongodb::bson::doc! {"_id": insert_result.inserted_id}).await.unwrap();
    user.expect("Should find newly created user")
}
