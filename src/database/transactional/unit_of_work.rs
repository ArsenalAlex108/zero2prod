use crate::dependency_injection::app_state::SendSyncStatic;

pub trait UnitOfWorkRepository: Send + Sync {
    type UnitOfWork: UnitOfWork;
}

pub trait BeginUnitOfWork: SendSyncStatic {
    type UnitOfWork: UnitOfWork;
    fn begin(
        &self,
    ) -> impl Future<
        Output = Result<Self::UnitOfWork, BeginError>,
    > + Send;
}

pub trait UnitOfWork: Send {
    fn commit(
        self,
    ) -> impl Future<Output = Result<(), CommitError>> + Send;
}

#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[derive(Debug, thiserror::Error)]
pub enum BeginError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}
