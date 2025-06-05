use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use chrono::Utc;
use const_format::formatcp;
use nameof::name_of;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String,
}

const SUBSCRIBE_INSTRUMENT_NAME: &str = formatcp!("Enter Route '{}':", name_of!(subscribe));

#[tracing::instrument(
    name = SUBSCRIBE_INSTRUMENT_NAME,
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    insert_subscriber(&form, &pool).await.map_or_else(
        |_| HttpResponse::InternalServerError().finish(),
        |_| HttpResponse::Ok().finish(),
    )
}

const INSERT_SUBSCRIBER_INSTRUMENT_NAME: &str =
    formatcp!("Enter Route '{}':", name_of!(insert_subscriber));

#[tracing::instrument(
    name = INSERT_SUBSCRIBER_INSTRUMENT_NAME,
    skip(form, pool)
)]
pub async fn insert_subscriber(form: &SubscribeFormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .inspect_err(|e| tracing::error!("Query error in {}: {:?}", name_of!(insert_subscriber), e))?;
    Ok(())
}
