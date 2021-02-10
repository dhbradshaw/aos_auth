use actix_http::http::StatusCode;
use actix_web::HttpMessage;
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use uuid::Uuid;

use crate::atomics::{Id, StatusError};
use crate::datalayer::DataLayer;

use super::user_session::{UserSession, SESSION_ID_KEY};

pub struct AuthedUser {
    pub id: Id,
}

impl FromRequest for AuthedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // 1. Is there a session cookie?
        let option_cookie = req.cookie(SESSION_ID_KEY);
        if option_cookie.is_none() {
            println!("Need cookie");
            return err(StatusError::new("You have no session cookie.  Login!", StatusCode::BAD_REQUEST).into());
        }
        let cookie = option_cookie.unwrap();

        // 2. Is it a uuid?
        let uuid = Uuid::parse_str(cookie.value());
        if uuid.is_err() {
            println!("Session cookie not uuid");
            return err(StatusError::new("Your session cookie is not a uuid.", StatusCode::BAD_REQUEST).into());
        }
        let session = UserSession {
            session_key: uuid.unwrap(),
        };

        // 3. Can we get a user_id for that session?
        let data_layer: &Data<DataLayer> = req.app_data::<Data<DataLayer>>().unwrap();
        let user_id_result = data_layer.userid_from_session(&session);
        if user_id_result.is_err() {
            println!("Database error getting userid from session key.");
            return err(
                StatusError::new("Sled Error getting userid from session key.", StatusCode::INTERNAL_SERVER_ERROR).into(),
            );
        }
        let user_id_option = user_id_result.unwrap();
        if user_id_option.is_none() {
            println!("No user_id for this session.");
            return err(StatusError::new("No user_id for this session.", StatusCode::UNAUTHORIZED).into());
        }
        let user_id = user_id_option.unwrap();
        let authed_user = AuthedUser { id: user_id };

        // 4. If so, return it.
        ok(authed_user)
    }
}
