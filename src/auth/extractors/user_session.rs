
use actix_http::cookie::Cookie;
use actix_web::error::{ErrorForbidden, ParseError};
use actix_web::HttpMessage;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSession {
    pub session_key: Uuid,
}

pub static SESSION_ID_KEY: &str = "session_id";
pub static SESSION_DELETED_VALUE: &str = "deleted";

impl UserSession {
    pub fn new() -> Self {
        UserSession {
            session_key: Uuid::new_v4(),
        }
    }
    pub fn create_cookie(&self) -> Cookie {
        Cookie::build(SESSION_ID_KEY, self.session_key.to_string())
            .http_only(true) // not accessible by javascript
            .max_age(Duration::days(30)) // can last 30 days in browser
            // .secure(true) // only works for https
            .path("/") // set for all paths in the domain
            // .domain("www.aos_auth.com")
            .finish()
    }
    pub fn null_cookie() -> Cookie<'static> {
        let ten_years_ago = time::OffsetDateTime::now_utc() - time::Duration::weeks(521);
        Cookie::build(SESSION_ID_KEY, SESSION_DELETED_VALUE)
            .http_only(true) // not accessible by javascript
            .path("/") // set for all paths in the domain
            .expires(ten_years_ago)
            // .domain("www.aos_auth.com")
            .finish()
    }
}

impl FromRequest for UserSession {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.cookie(SESSION_ID_KEY) {
            Some(cookie) => match Uuid::parse_str(cookie.value()) {
                Ok(uuid) => ok(UserSession { session_key: uuid }),
                Err(_) => err::<UserSession, Error>(ParseError::Header.into()),
            },
            None => err::<UserSession, Error>(ErrorForbidden(
                "No session uuid found.  You need to log in.",
            )),
        }
    }
}

