use actix_web::{
    HttpResponse, http::header::ContentType, web,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{authentication::UserId, utils::Pipe};

pub async fn admin_dashboard(
    user_id: web::ReqData<UserId>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id: Uuid = user_id.into_inner().into();

    let username = sqlx::query!(
        "--sql
    SELECT username FROM newsletter_writers
    WHERE user_id = $1
    ",
        &user_id
    )
    .fetch_one(pool.as_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .username;

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Admin dashboard</title>
</head>
<body>
<p>Welcome {username}!</p>
<p>Available actions:</p>
<ol>
    <li><a href="/admin/reset_password">Change password</a></li>
    <li>
        <form name="logoutForm" action="/admin/logout" method="post">
            <input type="submit" value="Logout">
        </form>
    </li>
    <li><a href="/admin/newsletters">Post Newsletter</a></li>
</ol>
</body>
</html>"#)).pipe(Ok)
}
