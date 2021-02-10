use serde::Deserialize;

use crate::atomics::{Email, Password};

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: Email,
    pub password: Password,
}
