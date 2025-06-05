use actix_web::{
    web::{self}, HttpResponse, Responder
};
use chrono::Utc;
use const_format::formatcp;
use nameof::name_of;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    form: web::Form<SubscribeFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    
    let request_span = tracing::info_span!(
        formatcp!("Enter Route '{}':", name_of!(subscribe)),
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        formatcp!("Saving subscriber in '{}':", name_of!(subscribe))
    );
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
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    .map_or_else(
        |e| {
            tracing::error!("Query error in {}: {:?}", name_of!(subscribe), e);
            HttpResponse::InternalServerError().finish()
        },
        |_| {
            tracing::info!("Success: {}", name_of!(subscribe));
            HttpResponse::Ok().finish() 
        },
    )
}
