pub fn hash_passwd(passwd: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash(passwd, bcrypt::DEFAULT_COST)?)
}

pub fn verify_passwd(passwd: &str, hashed_passwd: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(passwd, hashed_passwd)?)
}
