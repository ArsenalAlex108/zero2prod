use std::future::{Ready, ready};

use actix_session::{
    Session, SessionExt, SessionGetError,
    SessionInsertError,
};
use actix_web::FromRequest;
use eyre::bail;
use uuid::Uuid;

use crate::utils::Pipe;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew()
    }

    pub fn insert_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(
        &self,
    ) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get::<Uuid>(Self::USER_ID_KEY)
    }

    pub fn logout(&self) {
        self.0.purge()
    }

    pub fn get_required_user_id(
        &self,
    ) -> Result<Uuid, eyre::Report> {
        match self
            .get_user_id()
            .map_err(eyre::Report::new)?
        {
            Some(i) => Ok(i),
            None => bail!("User has not logged in."),
        }
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        req.get_session()
            .pipe(TypedSession)
            .pipe(Ok)
            .pipe(ready)
    }
}
