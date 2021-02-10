use aos_auth::atomics::{Email, PasswordHash};
use aos_auth::Config;
use std::env;

/// Read out database key.
fn main() {
    let config = Config::from_envs();

    // 1. Read in the email and password.
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("You need an email and a password!");
        return;
    }

    let email = &args[1];
    let email = Email::from_str(email);

    let password = &args[2];
    let password_hasher = config.get_password_hasher();
    let hashed_password = password_hasher.hash(password).unwrap();
    let password_hash = PasswordHash::from_str(&hashed_password);

    println!(
        "email: {:?}, password: {:?}, password_hash {:?}",
        email, password, password_hash
    );

    // 2. Now create a new user!
    let data_layer = config.get_datalayer();
    data_layer.create_user(&email, &password_hash).unwrap();
}
