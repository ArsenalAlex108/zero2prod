use actix_web::{
    HttpResponse, http::header::ContentType, web,
};
use uuid::Uuid;

use crate::{
    authentication::UserId,
    database::transactional::authentication::AuthenticationRepository,
    dependency_injection::app_state::Inject, utils::Pipe,
};

pub async fn admin_dashboard<
    U: AuthenticationRepository,
>(
    user_id: web::ReqData<UserId>,
    users_repository: Inject<U>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id: Uuid = user_id.into_inner().into();

    let username = users_repository
        .get_hashed_credentials_from_user_id(user_id)
        .await
        .map_err(
            actix_web::error::ErrorInternalServerError,
        )?
        .map(|credentials| credentials.username)
        .ok_or_else(|| {
            actix_web::error::ErrorNotFound(
                "User not found",
            )
        })?;

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
