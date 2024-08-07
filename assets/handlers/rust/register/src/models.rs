use serde::Deserialize;

#[derive(Debug, serde::Deserialize, serde::Serialize, serde_valid::Validate)]
pub struct User {
    #[serde(alias="_id", deserialize_with="deserialize_id")]
    pub id: String,
    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub username: String,

    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub email: String,

    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Debug, serde::Deserialize, serde::Serialize, serde_valid::Validate)]
pub struct CreateUser {
    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub username: String,

    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub email: String,

    #[validate(min_items=32)]
    #[validate(max_items=128)]
    #[serde(deserialize_with="hash_password", alias="password")]
    pub password_hash: Vec<u8>,

    #[serde(default="chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>
}

fn hash_password<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where D: serde::Deserializer<'de> {
    let password = String::deserialize(deserializer)?;

    Ok(crate::hash::hash_password(password))
}

fn deserialize_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where D: serde::Deserializer<'de> {
    struct StringOrObjectId;

    impl <'de> serde::de::Visitor<'de> for StringOrObjectId {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or ObjectId")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            Ok(String::from(v))
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, {
            Ok(mongodb::bson::oid::ObjectId::deserialize(serde::de::value::MapAccessDeserializer::new(map))?.to_hex())
        }
    }
    deserializer.deserialize_any(StringOrObjectId)
}
