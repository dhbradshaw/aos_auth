use actix_http::{cookie::Cookie, http::StatusCode};
use actix_web::{http::header, web::Data, web::Form, Error, HttpResponse, Responder};
use askama::Template;

use crate::atomics::StatusError;
use crate::datalayer::DataLayer;

use super::extractors::user_session::UserSession;
use super::forms;
use super::hashing::PasswordHasher;
use super::templates::LoginTemplate;

pub fn http_redirect(target: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(header::LOCATION, target)
        .finish()
}

pub fn http_redirect_with_cookie(target: &str, cookie: Cookie) -> HttpResponse {
    HttpResponse::Found()
        .cookie(cookie)
        .header(header::LOCATION, target)
        .finish()
}

pub async fn login_get() -> Result<HttpResponse, Error> {
    let template = LoginTemplate {};
    template
        .render()
        .map_err(|e| StatusError::new(&format!("{}", e), StatusCode::INTERNAL_SERVER_ERROR).into())
        .map(|s| HttpResponse::Ok().body(s))
}

/// Ideally there are a few different possible outcomes:
/// 1. The password matches and we log in. (This means we set a session cookie that points to a userid.)
/// 2. The password doesn't match so we don't log in. (This means that we go to some helpful view.)
/// 3. There was some error. (This means a helpful logged message and a 500 error.)
pub async fn login_post(
    password_hasher: Data<PasswordHasher>,
    form: Form<forms::LoginForm>,
    dl: Data<DataLayer>,
) -> Result<HttpResponse, StatusError> {
    // 1. Did the db work?
    let old_hash_option = dl.passwordhash_from_email(&form.email).map_err(|e| {
        StatusError::new(
            &format!("Database error fetching password hash: {}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    // 2. Was there a password hash for that email?
    let old_hash = old_hash_option.ok_or(StatusError::new(
        "You don't have a password in our records.  Are you registered?",
        StatusCode::UNAUTHORIZED,
    ))?;

    // 3. Does the submitted password match?
    if !password_hasher.verify(form.password.to_str(), old_hash.to_str()) {
        return Err(StatusError::new(
            "Your email and password don't match.",
            StatusCode::UNAUTHORIZED,
        ));
    }

    // 4. Create a session.
    let session = UserSession::new();
    
    // 5. Hurray!  Save the session db-side and send them home with a cookie!
    dl.set_session(&form.email, &session)
        .map_err(|e| {
            StatusError::new(
                &format!("Database error saving session: {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .map(|_| Ok(http_redirect_with_cookie("/", session.create_cookie())))?
}

pub async fn logout_get() -> impl Responder {
    http_redirect_with_cookie("/auth/login/", UserSession::null_cookie())
}
