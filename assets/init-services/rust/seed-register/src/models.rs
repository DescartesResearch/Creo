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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    #[serde(rename="_id")]
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: Vec<u8>,
    #[serde(default="chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>
}

impl User {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            username: random_string(3, 64),
            email: random_string(3, 64),
            password_hash: {
                let length = random_int(32, 128);
                (0..length).map(|_| random_int(u8::MIN, u8::MAX)).collect()
            },
            created_at: chrono::Utc::now(),
        }
    }
}
