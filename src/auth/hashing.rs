use argonautica::{Hasher, Verifier};

pub fn hash(password: &str, key: &str) -> Result<String, argonautica::Error> {
    let mut hasher = Hasher::default();
    hasher.with_password(password).with_secret_key(key).hash()
}

pub fn verify(password: &str, hash: &str, key: &str) -> bool {
    let mut verifier = Verifier::default();
    verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(key)
        .verify()
        .unwrap()
}

#[derive(Clone)]
pub struct PasswordHasher {
    key: String,
}

impl PasswordHasher {
    pub fn new(key: &str) -> PasswordHasher {
        let key = key.to_owned();
        PasswordHasher { key }
    }    
    pub fn hash(&self, password: &str) -> Result<String, argonautica::Error> {
        hash(password, &self.key)
    }
    pub fn verify(&self, password: &str, hash: &str) -> bool {
        verify(password, hash, &self.key)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        let password = "password";
        let key = "key";
        let ph = PasswordHasher::new(&key);
        let h = ph.hash(&password).unwrap();
        println!("{}", h);
        let verified = ph.verify(&password, &h);
        println!("{}", verified);
    }
}
