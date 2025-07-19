use actix_web::{
    HttpResponse, http::header::ContentType, web,
};
use std::fmt::Write;

use crate::authentication::UserId;

pub async fn get_reset_password_form(
    _user_id: web::ReqData<UserId>,
    flash_messages: actix_web_flash_messages::IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut notification_html = String::new();

    flash_messages.iter().for_each(|m| {
        writeln!(
            notification_html,
            "<p><i>{}</i></p>",
            m.content()
        )
        .expect(
            "Write to string should have been successful.",
        );
    });

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
<form action="/admin/reset_password" method="post">
<label>Current password
<input
type="password"
placeholder="Enter current password"
name="old_password"
>
</label>
<br>
<label>New password
<input
type="password"
placeholder="Enter new password"
name="new_password"
>
</label>
<br>
<label>Confirm new password
<input
type="password"
placeholder="Type the new password again"
name="confirm_new_password"
>
</label>
<br>
<button type="submit">Change password</button>
</form>
<p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
)))
}
