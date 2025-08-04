use std::{net::TcpListener, sync::Arc};

use actix_session::{
    SessionMiddleware, storage::CookieSessionStore,
};
use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self},
};

use reqwest::Client;
use secrecy::SecretString;
use tracing_actix_web::TracingLogger;

use crate::{
    authentication::reject_anonymous_users,
    configuration::{HmacSecret, Settings},
    dependency_injection::app_state::{
        AppState, AppStateFactory, AppStateTypes, Inject,
    },
    hkt::{
        ArcHKT, HKT1Unsized, K1, RefHKT, SendHKT,
        SharedPointerHKT, SyncHKT,
    },
    routes::{
        admin_dashboard, confirm_subscription_token,
        get_newsletter_form, get_reset_password_form,
        health_check, home, login, login_form, logout,
        post_reset_password, publish_newsletter, subscribe,
    },
    tuples::{LifterMut, ThinDataHKT, TupleMap9},
    utils::Pipe,
};
use secrecy::ExposeSecret;

pub type GlobalSharedPointerType = ArcHKT;
pub type GlobalSharedPointer<T> = Arc<T>;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl<P: HKT1Unsized>(
    pub K1<P, str>,
);

impl<P: SharedPointerHKT> Clone for ApplicationBaseUrl<P> {
    fn clone(&self) -> Self {
        ApplicationBaseUrl(self.0.clone())
    }
}

#[allow(clippy::unused_async)]
pub async fn run<
    P: RefHKT + SendHKT + SyncHKT,
    A: AppStateTypes,
>(
    listener: TcpListener,
    hmac_secret: HmacSecret<P>,
    configurer: impl FnMut(&mut web::ServiceConfig)
    + Send
    + 'static
    + Clone,
) -> Result<Server, eyre::Report> {
    let hmac_secret = hmac_secret.pipe(web::Data::new);

    let hmac_key = actix_web::cookie::Key::try_from(
        hmac_secret
            .as_ref()
            .as_ref()
            .as_ref()
            .expose_secret()
            .as_bytes(),
    )
    .expect("HmacSecret is valid key.");

    let cookie_store = actix_web_flash_messages::storage::CookieMessageStore::builder(
        hmac_key.clone()
    ).build();
    let message_framework = actix_web_flash_messages::FlashMessagesFramework::builder(cookie_store).build();

    let server = HttpServer::new(move || {
        let session_store = CookieSessionStore::default();
        let session_middleware = SessionMiddleware::new(
            session_store,
            hmac_key.clone(),
        );

        App::new()
            .wrap(session_middleware)
            .wrap(message_framework.clone())
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_form))
            .route(
                "/login",
                web::post().to(login::<
                    A::AuthenticationRepository,
                >),
            )
            .route(
                "/health_check",
                web::get().to(health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(subscribe::<
                    A::BeginUnitOfWork,
                    A::SubscriptionsRepository,
                >),
            )
            .route(
                "/subscriptions/confirm",
                web::get().to(
                    confirm_subscription_token::<
                        A::SubscriptionsConfirmRepository,
                    >,
                ),
            )
            .service(
                web::scope("/admin")
                    .wrap(actix_web::middleware::from_fn(
                        reject_anonymous_users,
                    ))
                    .route(
                        "/dashboard",
                        web::get().to(admin_dashboard::<
                            A::AuthenticationRepository,
                        >),
                    )
                    .route(
                        "/reset_password",
                        web::get()
                            .to(get_reset_password_form),
                    )
                    .route(
                        "/reset_password",
                        web::post().to(
                            post_reset_password::<
                                A::AuthenticationRepository,
                            >,
                        ),
                    )
                    .route(
                        "/logout",
                        web::post().to(logout),
                    )
                    .route(
                        "/newsletters",
                        web::post()
                            .to(publish_newsletter::<
                            A::AuthenticationRepository,
                            A::BeginUnitOfWork,
                            A::IssueDeliveryQueueRepository,
                            A::NewslettersRepository,
                            A::PersistenceRepository,
                        >),
                    )
                    .route(
                        "/newsletters",
                        web::get().to(get_newsletter_form),
                    ),
            )
            .configure(configurer.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

impl<T> actix_web::FromRequest for Inject<T>
where
    T: 'static + Send + Sync,
{
    type Error = actix_web::Error;
    type Future =
        std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let data = req
            .app_data::<web::ThinData<GlobalSharedPointer<T>>>()
            .cloned()
            .ok_or_else(|| {
                actix_web::error::ErrorInternalServerError(
                    "AppState not found",
                )
            })
            .map(Inject::new);
        std::future::ready(data)
    }
}

impl Application {
    pub async fn build<
        P: SharedPointerHKT + SendHKT + SyncHKT,
        A: AppStateFactory,
    >(
        configuration: &Settings<P>,
    ) -> Result<Application, eyre::Report> {
        Self::build_with::<P, A::AppStateTypes>(
            configuration,
            A::build(configuration),
        )
        .await
    }

    pub async fn build_with<
        P: SharedPointerHKT + SendHKT + SyncHKT,
        A: AppStateTypes,
    >(
        configuration: &Settings<P>,
        app_state: AppState<A>,
    ) -> Result<Application, eyre::Report> {
        let email_client = configuration
            .email_client
            .as_ref()
            .clone()
            .client();

        let email_client = web::Data::new(email_client);

        let configuration = configuration.clone();

        let address = format!(
            "{}:{}",
            &configuration.application.host,
            &configuration.application.port
        );

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let app_state = app_state
            .into_tuple()
            .lift_map::<ThinDataHKT>();

        struct Cfg<'a>(&'a mut web::ServiceConfig);

        impl LifterMut<'static> for Cfg<'_> {
            fn lift<T: 'static>(
                &mut self,
                t: T,
            ) -> Self::T<T> {
                self.0.app_data(t);
            }

            type T<A: 'static> = ();
        }

        run::<P, A>(
            listener,
            configuration
                .application
                .hmac_secret
                .as_ref()
                .clone(),
            move |cfg| {
                app_state.clone().map_mut(&mut Cfg(cfg));
                cfg.app_data(email_client.clone())
                    .app_data(
                        ApplicationBaseUrl(
                            configuration
                                .application
                                .base_url
                                .clone(),
                        )
                        .pipe(web::ThinData),
                    );
            },
        )
        .await
        .map(|server| Application { port, server })
    }

    #[must_use]
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(
        self,
    ) -> std::io::Result<()> {
        self.server.await
    }
}
