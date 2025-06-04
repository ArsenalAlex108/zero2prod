use actix_web::{web::Form, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    pub email: String,
    pub name: String
}

pub async fn subscribe(form: Form<SubscribeFormData>) -> impl Responder {
    HttpResponse::Ok()
}