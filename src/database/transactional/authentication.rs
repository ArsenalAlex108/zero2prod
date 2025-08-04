use secrecy::SecretString;
use uuid::Uuid;

use crate::dependency_injection::app_state::SendSyncStatic;

pub struct HashedCredentials {
    pub user_id: Uuid,
    pub username: String,
    pub salted_password: SecretString,
}

pub trait AuthenticationRepository: SendSyncStatic {
    fn get_hashed_credentials_from_username(
        &self,
        username: &str,
    ) -> impl Future<
        Output = Result<
            Option<HashedCredentials>,
            GetHashedCredentialsError,
        >,
    > + Send;

    fn get_hashed_credentials_from_user_id(
        &self,
        user_id: Uuid,
    ) -> impl Future<
        Output = Result<
            Option<HashedCredentials>,
            GetHashedCredentialsError,
        >,
    > + Send;

    fn update_password(
        &self,
        user_id: Uuid,
        new_salted_password: &SecretString,
    ) -> impl Future<
        Output = Result<(), UpdatePasswordError>,
    > + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum GetHashedCredentialsError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdatePasswordError {
    #[error("User not found with ID: {0}")]
    UserNotFound(String),
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}
