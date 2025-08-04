use argon2::password_hash;
use secrecy::SecretString;
use uuid::Uuid;

pub trait InsertNewsletterWriterRepository {
    fn insert(
        &self,
        user_id: Uuid,
        username: &str,
        password_hash: &SecretString,
    ) -> impl Future<Output = Result<(), eyre::Report>> + Send;
}
