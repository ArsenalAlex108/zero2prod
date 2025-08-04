use std::convert::Infallible;

use zero2prod::{
    dependency_injection::app_state::{
        AppState, AppStateTypes, DefaultAppStateTypes,
        PgPoolConcrete,
    },
    startup::GlobalSharedPointer,
};

use crate::common::test_dependency_injection::test_database::{get_subscriptions_repository::GetSubscriptionsRepository, insert_newsletter_writer_repository::InsertNewsletterWriterRepository, repository_suspender::RepositorySuspender};

pub trait TestAppStateTypes {
    type InsertNewsletterWriterRepository: InsertNewsletterWriterRepository;
    type GetSubscriptionsRepository: GetSubscriptionsRepository;
    type RepositorySuspender: RepositorySuspender;
}

pub struct TestAppState<A: TestAppStateTypes> {
    pub insert_newsletter_writer_repository:
        GlobalSharedPointer<
            A::InsertNewsletterWriterRepository,
        >,
    pub get_subscriptions_repository:
        GlobalSharedPointer<A::GetSubscriptionsRepository>,
    pub repository_suspender:
        GlobalSharedPointer<A::RepositorySuspender>,
}

impl<A: TestAppStateTypes> Clone for TestAppState<A> {
    fn clone(&self) -> Self {
        TestAppState {
            insert_newsletter_writer_repository: self
                .insert_newsletter_writer_repository
                .clone(),
            get_subscriptions_repository: self
                .get_subscriptions_repository
                .clone(),
            repository_suspender: self
                .repository_suspender
                .clone(),
        }
    }
}

pub struct TestAppTypesImpl(Infallible);

impl TestAppStateTypes for TestAppTypesImpl {
    type InsertNewsletterWriterRepository = PgPoolConcrete;
    type GetSubscriptionsRepository = PgPoolConcrete;
    type RepositorySuspender = PgPoolConcrete;
}

pub fn get_test_app_state(
    pool: sqlx::PgPool,
) -> TestAppState<TestAppTypesImpl> {
    let pg_pool =
        GlobalSharedPointer::new(PgPoolConcrete::new(pool));

    TestAppState {
        insert_newsletter_writer_repository: pg_pool
            .clone(),
        get_subscriptions_repository: pg_pool.clone(),
        repository_suspender: pg_pool.clone(),
    }
}

pub trait TestAppStateFactory {
    type TestAppStateTypes: TestAppStateTypes;
    type AppStateTypes: AppStateTypes;

    fn build(
        app_state: &AppState<Self::AppStateTypes>,
    ) -> TestAppState<Self::TestAppStateTypes>;
}

pub struct TestAppStateFactoryImpl(Infallible);

impl TestAppStateFactory for TestAppStateFactoryImpl {
    type TestAppStateTypes = TestAppTypesImpl;
    type AppStateTypes = DefaultAppStateTypes;

    fn build(
        app_state: &AppState<Self::AppStateTypes>,
    ) -> TestAppState<Self::TestAppStateTypes> {
        get_test_app_state(
            app_state.begin_unit_of_work.pool().clone(),
        )
    }
}
