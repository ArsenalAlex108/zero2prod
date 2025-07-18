use crate::utils::Pipe;

use super::IdempotencyKey;
use actix_web::{HttpResponse, http::StatusCode};
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
pub struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}

pub async fn get_saved_response(
    connection: &mut sqlx::PgConnection,
    idempotency_key: &IdempotencyKey<'_>,
    user_id: Uuid,
) -> Result<Option<HttpResponse>, eyre::Report> {
    let saved_response = sqlx::query!(
        r#"--sql
        SELECT
        response_status_code,
        response_headers as "response_headers: Vec<HeaderPairRecord>",
        response_body
        FROM idempotency
        WHERE
        user_id = $1 AND
        idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(connection)
    .await?;

    if let Some(r) = saved_response {
        let status_code = StatusCode::from_u16(
            r.response_status_code.try_into()?,
        )?;
        let mut response = HttpResponse::build(status_code);
        for HeaderPairRecord { name, value } in
            r.response_headers
        {
            response.append_header((name, value));
        }
        Ok(Some(response.body(r.response_body)))
    } else {
        Ok(None)
    }
}

pub async fn save_response(
    connection: &mut sqlx::PgConnection,
    idempotency_key: &IdempotencyKey<'_>,
    user_id: Uuid,
    http_response: HttpResponse,
) -> Result<HttpResponse, eyre::Report> {
    let status_code =
        http_response.status().as_u16() as i16;

    let (headers_response, body) =
        http_response.into_parts();

    let headers = headers_response
        .headers()
        .iter()
        .map(|pair| {
            let (name, value) = pair;
            let name = name.as_str().to_owned();
            let value = value.as_bytes().to_owned();
            HeaderPairRecord { name, value }
        })
        .collect::<Vec<_>>();

    let body = actix_web::body::to_bytes(body)
        .await
        .map_err(|e| eyre::eyre!("{e}"))?;

    sqlx::query_unchecked!(
        r#"--sql
    INSERT INTO idempotency (
        user_id,
        idempotency_key,
        response_status_code,
        response_headers,
        response_body,
        created_at
    )
    VALUES ($1, $2, $3, $4, $5, now())
    "#,
        user_id,
        idempotency_key.as_ref(),
        status_code,
        headers,
        body.as_ref()
    )
    .execute(connection)
    .await?;

    headers_response
        .set_body(body)
        .map_into_boxed_body()
        .pipe(Ok)
}
