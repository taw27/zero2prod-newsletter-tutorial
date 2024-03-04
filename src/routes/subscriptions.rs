use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    email: String,
    name: String,
}
pub async fn subscribe(_form: web::Form<SubscribeFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
