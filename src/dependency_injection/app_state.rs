use std::{
    convert::Infallible, marker::PhantomData, ops::Deref,
};

use actix_web::web;

use crate::{
    configuration::{DatabaseSettings, Settings},
    database::{
        postgres::{
            PgPool, PgPoolDependencies, PgRepository,
            PgRepositoryDependencies, PgTransaction,
        },
        transactional::{
            authentication::AuthenticationRepository,
            issue_delivery_queue::IssueDeliveryQueueRepository,
            newsletters::NewslettersRepository,
            persistence::PersistenceRepository,
            subscriptions::SubscriptionsRepository,
            subscriptions_confirm::SubscriptionsConfirmRepository,
            unit_of_work::{BeginUnitOfWork, UnitOfWork},
        },
    },
    hkt::{RefHKT, SendHKT, SharedPointerHKT, SyncHKT},
    issue_delivery_worker::IssueDeliveryWorkerDependencyAlias,
    services::{
        clock::{Clock, SystemClock},
        uuid::{DefaultUuidGenerator, UuidGenerator},
    },
    startup::GlobalSharedPointer,
};

pub trait AppStateTypes: Marker {
    type UuidGenerator: UuidGenerator;
    type Clock: Clock;

    type UnitOfWork: UnitOfWork;
    type BeginUnitOfWork: BeginUnitOfWork<
        UnitOfWork = Self::UnitOfWork,
    >;

    type AuthenticationRepository: AuthenticationRepository;
    type SubscriptionsConfirmRepository: SubscriptionsConfirmRepository;

    type IssueDeliveryQueueRepository: IssueDeliveryQueueRepository<UnitOfWork = Self::UnitOfWork>;
    type NewslettersRepository: NewslettersRepository<
        UnitOfWork = Self::UnitOfWork,
    >;
    type PersistenceRepository: PersistenceRepository<
        UnitOfWork = Self::UnitOfWork,
    >;
    type SubscriptionsRepository: SubscriptionsRepository<
        UnitOfWork = Self::UnitOfWork,
    >;
}

pub struct DefaultAppStateTypes;

impl Marker for DefaultAppStateTypes {}

impl AppStateTypes for DefaultAppStateTypes {
    type UuidGenerator = DefaultUuidGenerator;
    type Clock = SystemClock;

    type UnitOfWork = PgTransaction;
    type BeginUnitOfWork = PgPoolConcrete;

    type AuthenticationRepository = PgPoolConcrete;
    type SubscriptionsConfirmRepository = PgPoolConcrete;

    type IssueDeliveryQueueRepository =
        PgRepositoryConcrete;
    type NewslettersRepository = PgRepositoryConcrete;
    type PersistenceRepository = PgRepositoryConcrete;
    type SubscriptionsRepository = PgRepositoryConcrete;
}

pub trait AppStateFactory: Marker {
    type AppStateTypes: AppStateTypes;
    fn build<P: SharedPointerHKT>(
        configuration: &Settings<P>,
    ) -> AppState<Self::AppStateTypes>;
}

pub struct PgRepositoryDependencyTypes;
pub type PgRepositoryConcrete =
    PgRepository<PgRepositoryDependencyTypes>;

impl PgRepositoryDependencies
    for PgRepositoryDependencyTypes
{
    type UuidGenerator = DefaultUuidGenerator;
    type Clock = SystemClock;
}

pub struct PgPoolDependencyTypes;
pub type PgPoolConcrete = PgPool<PgPoolDependencyTypes>;

impl PgPoolDependencies for PgPoolDependencyTypes {}

pub trait SendSyncStatic: Send + Sync {}
impl<T: Send + Sync> SendSyncStatic for T {}

pub trait Marker: Send + Sync + 'static {}

pub struct AppState<A: AppStateTypes> {
    pub uuid_generator:
        GlobalSharedPointer<A::UuidGenerator>,
    pub clock: GlobalSharedPointer<A::Clock>,

    pub begin_unit_of_work:
        GlobalSharedPointer<A::BeginUnitOfWork>,

    pub authentication_repository:
        GlobalSharedPointer<A::AuthenticationRepository>,
    pub subscriptions_confirm_repository:
        GlobalSharedPointer<
            A::SubscriptionsConfirmRepository,
        >,

    pub issue_delivery_queue_repository:
        GlobalSharedPointer<
            A::IssueDeliveryQueueRepository,
        >,
    pub newsletters_repository:
        GlobalSharedPointer<A::NewslettersRepository>,
    pub persistence_repository:
        GlobalSharedPointer<A::PersistenceRepository>,
    pub subscriptions_repository:
        GlobalSharedPointer<A::SubscriptionsRepository>,
}

impl<A: AppStateTypes> Clone for AppState<A> {
    fn clone(&self) -> Self {
        AppState {
            uuid_generator: self.uuid_generator.clone(),
            clock: self.clock.clone(),
            begin_unit_of_work: self
                .begin_unit_of_work
                .clone(),
            authentication_repository: self
                .authentication_repository
                .clone(),
            subscriptions_confirm_repository: self
                .subscriptions_confirm_repository
                .clone(),
            issue_delivery_queue_repository: self
                .issue_delivery_queue_repository
                .clone(),
            newsletters_repository: self
                .newsletters_repository
                .clone(),
            persistence_repository: self
                .persistence_repository
                .clone(),
            subscriptions_repository: self
                .subscriptions_repository
                .clone(),
        }
    }
}

impl<A: AppStateTypes> AppState<A> {
    #[must_use]
    #[allow(clippy::type_complexity)]
    pub fn into_tuple(
        self,
    ) -> (
        GlobalSharedPointer<A::UuidGenerator>,
        GlobalSharedPointer<A::Clock>,
        GlobalSharedPointer<A::BeginUnitOfWork>,
        GlobalSharedPointer<A::AuthenticationRepository>,
        GlobalSharedPointer<
            A::SubscriptionsConfirmRepository,
        >,
        GlobalSharedPointer<
            A::IssueDeliveryQueueRepository,
        >,
        GlobalSharedPointer<A::NewslettersRepository>,
        GlobalSharedPointer<A::PersistenceRepository>,
        GlobalSharedPointer<A::SubscriptionsRepository>,
    ) {
        (
            self.uuid_generator,
            self.clock,
            self.begin_unit_of_work,
            self.authentication_repository,
            self.subscriptions_confirm_repository,
            self.issue_delivery_queue_repository,
            self.newsletters_repository,
            self.persistence_repository,
            self.subscriptions_repository,
        )
    }
}

pub struct DefaultAppStateFactory(Infallible);

impl Marker for DefaultAppStateFactory {}

impl AppStateFactory for DefaultAppStateFactory {
    type AppStateTypes = DefaultAppStateTypes;

    fn build<P: SharedPointerHKT>(
        configuration: &Settings<P>,
    ) -> AppState<Self::AppStateTypes> {
        let uuid_generator =
            GlobalSharedPointer::new(DefaultUuidGenerator);
        let clock = GlobalSharedPointer::new(SystemClock);

        let connection_pool =
            get_connection_pool(&configuration.database);

        let pg_pool = GlobalSharedPointer::new(PgPool::<
            PgPoolDependencyTypes,
        >::new(
            connection_pool.clone(),
        ));

        let get_pool_arc = || pg_pool.clone();

        let begin_unit_of_work = get_pool_arc();

        let authentication_repository = get_pool_arc();
        let subscriptions_confirm_repository =
            get_pool_arc();

        let repository =
            GlobalSharedPointer::new(PgRepository::<
                PgRepositoryDependencyTypes,
            >::new(
                clock.clone(),
                uuid_generator.clone(),
            ));

        let issue_delivery_queue_repository =
            repository.clone();
        let newsletters_repository = repository.clone();
        let persistence_repository = repository.clone();
        let subscriptions_repository = repository.clone();

        AppState {
            uuid_generator,
            clock,
            begin_unit_of_work,
            authentication_repository,
            subscriptions_confirm_repository,
            issue_delivery_queue_repository,
            newsletters_repository,
            persistence_repository,
            subscriptions_repository,
        }
    }
}

pub struct Inject<T>(web::ThinData<GlobalSharedPointer<T>>);

impl<T> Inject<T> {
    #[must_use]
    pub fn new(
        data: web::ThinData<GlobalSharedPointer<T>>,
    ) -> Self {
        Inject(data)
    }
}

impl<T> Deref for Inject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct IssueDeliveryWorkerTypes<
    P: SharedPointerHKT + SendHKT + SyncHKT,
    A: AppStateTypes,
>(PhantomData<(P, A)>, Infallible);

// This is why we do not introduce a new trait as alias.
impl<
    P: SharedPointerHKT + SendHKT + SyncHKT,
    A: AppStateTypes,
> IssueDeliveryWorkerDependencyAlias
    for IssueDeliveryWorkerTypes<P, A>
{
    type P = P;

    type B = A::BeginUnitOfWork;

    type N = A::NewslettersRepository;

    type I = A::IssueDeliveryQueueRepository;
}

pub fn get_connection_pool<P: RefHKT>(
    configuration: &DatabaseSettings<P>,
) -> sqlx::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
