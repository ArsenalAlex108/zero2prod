use uuid::Uuid;

use crate::{
    database::transactional::unit_of_work::UnitOfWorkRepository,
    idempotency::IdempotencyKey,
};

pub struct HeaderPairRecord {
    pub name: String,
    pub value: Vec<u8>,
}

#[derive(Debug, derive_more::Display)]
#[display(
    r"{{
    user_id: {user_id},
    idempotency_key: {idempotency_key}
}}"
)]
pub struct SavedResponseKey<'a> {
    pub user_id: Uuid,
    pub idempotency_key: IdempotencyKey<'a>,
}

impl SavedResponseKey<'_> {
    #[must_use]
    pub fn into_owned(self) -> SavedResponseKey<'static> {
        SavedResponseKey {
            user_id: self.user_id,
            idempotency_key: self
                .idempotency_key
                .into_owned(),
        }
    }
}

pub struct SavedResponseBody {
    pub response_status_code: u16,
    pub response_headers: Vec<HeaderPairRecord>,
    pub response_body: Vec<u8>,
}

pub trait PersistenceRepository:
    UnitOfWorkRepository
{
    fn get_saved_response_body<'a>(
        &self,
        unit_of_work: &'a mut Self::UnitOfWork,
        idempotency_key: &'a IdempotencyKey<'a>,
        user_id: Uuid,
    ) -> impl Future<
        Output = Result<
            Option<SavedResponseBody>,
            GetSavedResponseBodyError<'a>,
        >,
    > + Send;

    fn save_response_body(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        user_id: Uuid,
        idempotency_key: &IdempotencyKey<'_>,
        status_code: u16,
        headers: Vec<HeaderPairRecord>,
        body: &[u8],
    ) -> impl Future<
        Output = Result<(), SaveResponseBodyError>,
    > + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum GetSavedResponseBodyError<'a> {
    #[error("No saved response body found for keys: '{0}'")]
    NotFound(SavedResponseKey<'a>),
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

impl GetSavedResponseBodyError<'_> {
    #[must_use]
    pub fn into_owned(
        self,
    ) -> GetSavedResponseBodyError<'static> {
        match self {
            GetSavedResponseBodyError::NotFound(key) => {
                GetSavedResponseBodyError::NotFound(
                    key.into_owned(),
                )
            }
            GetSavedResponseBodyError::Unexpected(e) => {
                GetSavedResponseBodyError::Unexpected(e)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SaveResponseBodyError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}
