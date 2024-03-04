use crate::error::AppError;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};

fn compute_password_hash(password: String) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hashed_password = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(15000, 2, 1, None).unwrap())
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hashed_password)
}

fn verify_password_hash(password: String, expected_password_hash: String) -> Result<(), AppError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())?;
    Argon2::default()
        .verify_password(password.as_bytes(), &expected_password_hash)
        .map_err(|e| <argon2::password_hash::Error as Into<AppError>>::into(e))
}
