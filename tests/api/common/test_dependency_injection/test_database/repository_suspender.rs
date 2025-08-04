pub trait RepositorySuspender {
    fn suspend(
        &self,
    ) -> impl Future<Output = Result<(), eyre::Report>> + Send;
}
