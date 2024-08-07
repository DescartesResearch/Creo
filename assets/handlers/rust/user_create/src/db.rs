use super::models;
use lazy_static::lazy_static;


lazy_static! {
    static ref HOST: String = std::env::var("DB_MONGO_HOST").unwrap();
    static ref PORT: String = std::env::var("DB_MONGO_PORT").unwrap();
    static ref USER: String = std::env::var("DB_MONGO_USER").unwrap();
    static ref PASSWORD: String = std::env::var("DB_MONGO_PASSWORD").unwrap();
    static ref CLIENT: tokio::sync::OnceCell<mongodb::Client> = tokio::sync::OnceCell::const_new();
}

async fn get_client() -> &'static mongodb::Client {
    CLIENT.get_or_init(|| async {
        mongodb::Client::with_uri_str(format!("mongodb://{}:{}@{}:{}", *USER, *PASSWORD, *HOST, *PORT)).await.unwrap()
    }).await
}

const USER_DB: &str = "user_db";
const USER_COLLECTION: &str = "user_collection";

pub async fn get_collection() -> mongodb::Collection<models::User> {
    get_client().await.database(USER_DB).collection(USER_COLLECTION)
}

