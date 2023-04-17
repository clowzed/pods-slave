#[actix_web::get("api/health")]
async fn health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}
