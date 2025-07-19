use actix_web::{
    HttpResponse, http::header::ContentType, web,
};
use std::fmt::Write;
use uuid::Uuid;

use crate::authentication::UserId;

pub async fn get_newsletter_form(
    _user_id: web::ReqData<UserId>,
    flash_messages: actix_web_flash_messages::IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut notification_html = String::new();

    flash_messages
        .iter()
        .filter(|m| {
            m.level()
                > actix_web_flash_messages::Level::Debug
        })
        .for_each(|m| {
            writeln!(
            notification_html,
            "<p><i>{}</i></p>",
            m.content()
        )
        .expect(
            "Write to string should have been successful.",
        );
        });

    let idempotency_key = Uuid::new_v4();

    Ok(HttpResponse::Ok()
.content_type(ContentType::html())
.body(
    format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Change Password</title>
</head>
<body>
{notification_html}
<form action="/admin/newsletters" method="post">
<label>Title
<input
type="text"
placeholder="Enter title"
name="title"
>
<br>
<label>Newsletter Content (Text)
<input
type="text"
placeholder="Enter newsletter content (text)"
name="content_text"
>
<br>
<label>Newsletter Content (Html)
<input
type="text"
placeholder="Enter newsletter content (Html)"
name="content_html"
>
<br>
<input hidden type="text" name="idempotency_key" value="{idempotency_key}">
<button type="submit">Send newsletter</button>
</form>
<p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
)))
}
