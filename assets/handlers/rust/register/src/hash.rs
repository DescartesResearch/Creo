use argon2::PasswordHasher;
use lazy_static::lazy_static;

const TIME_COST: u32 = 1;
const MEMORY_COST: u32 = 6144;
const PARALLELISM: u32 = 4;

const fn params() -> argon2::Params {
    match argon2::Params::new(MEMORY_COST, TIME_COST, PARALLELISM, None) {
        Ok(params) => params,
        Err(_) => panic!("Failed to create params")
    }
}

const PARAMS: argon2::Params = params();
lazy_static!{
    pub static ref HASHER: argon2::Argon2<'static> = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, PARAMS);
}

pub fn hash_password(password: String) -> Vec<u8>  {
    let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    HASHER.hash_password(password.as_bytes(), &salt).unwrap().to_string().into_bytes()
}
