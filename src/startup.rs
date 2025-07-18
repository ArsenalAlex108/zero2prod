use std::net::TcpListener;

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
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_actix_web::TracingLogger;

use crate::{
    authentication::reject_anonymous_users,
    configuration::{
        DatabaseSettings, HmacSecret, Settings,
    },
    email_client::EmailClient,
    hkt::{
        ArcHKT, HKT1Unsized, K1, RefHKT, SharedPointerHKT,
    },
    routes::{
        admin_dashboard, confirm_subscription_token,
        get_newsletter_form, get_reset_password_form,
        health_check, home, login, login_form, logout,
        post_reset_password, publish_newsletter, subscribe,
    },
    utils::Pipe,
};
use secrecy::ExposeSecret;

pub type GlobalSharedPointerType = ArcHKT;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl<P: HKT1Unsized>(
    pub K1<P, str>,
);

pub async fn run<P: RefHKT>(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient<P>,
    // Cow<'_, str> can be alternative using into_owned() (clone at most once).
    // But String is more honest at that point
    // Arc-like can be shared accross threads without cloning at the cost of intial memory allocation.
    base_url: K1<P, str>,
    hmac_secret: HmacSecret<P>,
) -> Result<Server, eyre::Report>
where
    P::T<Client>: Send + Sync,
    P::T<str>: Send + Sync,
    P::T<SecretString>: Send + Sync,
{
    let db_pool = db_pool.pipe(web::Data::new);
    let email_client = email_client.pipe(web::Data::new);
    let base_url = base_url
        .pipe(ApplicationBaseUrl)
        .pipe(web::Data::new);
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
            .route("/login", web::post().to(login))
            .route(
                "/health_check",
                web::get().to(health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(subscribe),
            )
            .route(
                "/subscriptions/confirm",
                web::get().to(confirm_subscription_token),
            )
            .service(
                web::scope("/admin")
                    .wrap(actix_web::middleware::from_fn(
                        reject_anonymous_users,
                    ))
                    .route(
                        "/dashboard",
                        web::get().to(admin_dashboard),
                    )
                    .route(
                        "/reset_password",
                        web::get()
                            .to(get_reset_password_form),
                    )
                    .route(
                        "/reset_password",
                        web::post().to(post_reset_password),
                    )
                    .route(
                        "/logout",
                        web::post().to(logout),
                    )
                    .route(
                        "/newsletters",
                        web::post().to(publish_newsletter),
                    )
                    .route(
                        "/newsletters",
                        web::get().to(get_newsletter_form),
                    ),
            )
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(hmac_secret.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

impl Application {
    pub async fn build<P: SharedPointerHKT>(
        configuration: &Settings<P>,
    ) -> Result<Application, eyre::Report>
    where
        P::T<Client>: Send + Sync,
        P::T<str>: Send + Sync,
        P::T<SecretString>: Send + Sync,
    {
        let connection_pool =
            get_connection_pool(&configuration.database);

        let sender_email =
            configuration.email_client.sender().unwrap();

        let email_client = configuration
            .email_client
            .as_ref()
            .clone()
            .client();

        let address = format!(
            "{}:{}",
            configuration.application.host,
            configuration.application.port
        );

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        run::<P>(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url.clone(),
            configuration
                .application
                .hmac_secret
                .as_ref()
                .clone(),
        )
        .await
        .map(|server| Application { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(
        self,
    ) -> std::io::Result<()> {
        self.server.await
    }
}

pub fn get_connection_pool<P: RefHKT>(
    configuration: &DatabaseSettings<P>,
) -> PgPool {
    PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
