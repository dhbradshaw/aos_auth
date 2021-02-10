use crate::atomics::StatusError;
use crate::auth::extractors::authed_user::AuthedUser;
use super::templates::IndexTemplate;
use actix_http::{Error, http::StatusCode};
use actix_web::HttpResponse;
use askama::Template;

pub async fn index(user: AuthedUser) -> Result<HttpResponse, Error> {
    let template = IndexTemplate { user_id: user.id };
    template
        .render()
        .map_err(|e| StatusError::new(&format!("{}", e), StatusCode::INTERNAL_SERVER_ERROR).into())
        .map(|s| HttpResponse::Ok().body(s))
}
