use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password.as_bytes(), &salt){
        Ok(pass) => Ok(pass.to_string()),
        Err(e) => Err(e),
    }
}

pub fn verify_password(password: &str,password_hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(&password_hashed).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
