use serde::Serialize;
use sled::{Db, Tree};

use crate::atomics::{BoxResult, CouldBe, Email, Id, PasswordHash, BasicError, Worked};
use crate::auth::extractors::user_session::UserSession;

#[allow(non_snake_case)] // Use double underscore to separate keys from values in tree names.

/// Goals for this layer:
/// 1. If you change to a new back end, it's only this layer that has to be rewritten.
/// That is, only this layer knows that sled exists!
/// 2. Keep this layer as small as possible without sacrificing
///     a. correctness, or
///     b. performance.
/// Example: data needs to be parsed and cleaned before storage.  
/// This isn't where that happens because that would make this layer larger than it has
/// to be.  Also, that cleaning should be backend independent.  It's a type of business logic.
#[derive(Clone, Debug)]
pub struct DataLayer {
    // These are all private while the methods are public.
    // That's how we keep a separation between 
    db: Db,
    email__user_id: Tree,
    email__passwordhash: Tree,
    session_uuid__user_id: Tree,
    user_id__email: Tree,
}

fn serialize<T: Serialize>(value: T) -> BoxResult<Vec<u8>> {
    Ok(serde_json::to_vec(&value)?)
}

impl DataLayer {
    pub fn new(db_location: &str) -> Self {
        let db = sled::open(db_location).expect("database access");
        return DataLayer {
            email__user_id: db
                .open_tree("email__user_id")
                .expect("session__userid tree access"),
            email__passwordhash: db
                .open_tree("email__passwordhash")
                .expect("email__passwordhash tree access"),
            session_uuid__user_id: db
                .open_tree("session__userid")
                .expect("session__userid tree access"),
            user_id__email: db
                .open_tree("user_id__email")
                .expect("user_id__email tree access"),
            db,
        };
    }

    pub fn passwordhash_from_email(&self, email: &Email) -> CouldBe<PasswordHash> {
        match self.email__passwordhash.get(serialize(email)?)? {
            Some(ivec) => Ok(serde_json::from_slice(&ivec)?),
            None => Ok(None),
        }
    }

    pub fn set_session(&self, email: &Email, session: &UserSession) -> Worked {
        let option_userid = self.userid_from_email(email)?;
        match option_userid {
            Some(userid) => {
                self.session_uuid__user_id
                    .insert(serialize(session)?, serialize(userid)?)?;
                Ok(())
            }
            None => Err(Box::new(BasicError::new("No userid for this email"))),
        }
    }

    pub fn userid_from_session(&self, session: &UserSession) -> CouldBe<Id> {
        match self.session_uuid__user_id.get(serialize(session)?)? {
            Some(ivec) => Ok(serde_json::from_slice(&ivec)?),
            None => Ok(None),
        }
    }

    pub fn set_password(&self, email: &Email, passwordhash: &PasswordHash) -> Worked {
        self.email__passwordhash
            .insert(serialize(email)?, serialize(passwordhash)?)?;
        Ok(())
    }

    pub fn userid_from_email(&self, email: &Email) -> CouldBe<Id> {
        match self.email__user_id.get(serialize(email)?)? {
            Some(ivec) => Ok(serde_json::from_slice(&ivec)?),
            None => Ok(None),
        }
    }

    pub fn email_join_userid(&self, email: &Email, userid: &Id) -> Worked {
        self.email__user_id
            .insert(serialize(email)?, serialize(userid)?)?;
        self.user_id__email
            .insert(serialize(userid)?, serialize(email)?)?;
        Ok(())
    }

    pub fn generate_id(&self) -> BoxResult<Id> {
        Ok(Id::from_u64(self.db.generate_id()?))
    }

    pub fn create_user(&self, email: &Email, passwordhash: &PasswordHash) -> BoxResult<Id> {
        match self.userid_from_email(email)? {
            Some(id) => Ok(id),
            None => {
                self.set_password(email, passwordhash)?;
                let id = self.generate_id()?;
                self.email_join_userid(email, &id)?;
                Ok(id)
            }
        }
    }
}
