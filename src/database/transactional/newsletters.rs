use uuid::Uuid;

use crate::database::transactional::unit_of_work::UnitOfWorkRepository;

pub trait NewslettersRepository:
    UnitOfWorkRepository
{
    fn get_newsletter_content(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        uuid: Uuid,
    ) -> impl Future<
        Output = Result<
            NewsletterContent,
            GetNewsletterContentError,
        >,
    > + Send;

    fn insert_newsletter_issue(
        &self,
        unit_of_work: &mut Self::UnitOfWork,
        title: &str,
        text_content: &str,
        html_content: &str,
    ) -> impl std::future::Future<
        Output = Result<Uuid, InsertNewsletterIssueError>,
    > + Send;
}

pub struct NewsletterContent {
    pub title: String,
    pub text_content: String,
    pub html_content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GetNewsletterContentError {
    #[error("No newsletter with uuid '{0}' found.")]
    NotFound(Uuid),
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum InsertNewsletterIssueError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}
