use actix_web::{HttpResponse, http::header::ContentType};

use crate::utils::Pipe;
use std::fmt::Write;

pub async fn login_form(
    flash_messages: actix_web_flash_messages::IncomingFlashMessages,
) -> HttpResponse {
    let mut error_html = String::new();

    flash_messages.iter()
    .filter(|m|m.level() > actix_web_flash_messages::Level::Debug)
    .for_each(|m|
        writeln!(error_html, "<p><i>{}</i></p>", m.content())
        .expect("Write to string should have been successful.")
    );

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(format!(
r#"<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>

<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username
            <input type="text" placeholder="Enter Username" name="username">
        </label>
        <label>Password
            <input type="password" name="password">
        </label>
        <button type="submit">Login</button>
    </form>
</body>

</html>
"#))
    .pipe(|mut i| {
        let _ = i.add_removal_cookie(&actix_web::cookie::Cookie::new("_flash", ""));
        i
    })
}
