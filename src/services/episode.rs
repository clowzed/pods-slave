#[actix_web::get("/stream/{filename}")]
async fn stream(
    request: actix_web::HttpRequest,
    data: actix_web::web::Data<crate::config::Config>,
    path: actix_web::web::Path<(String,)>,
) -> impl actix_web::Responder {
    match actix_files::NamedFile::open_async(data.upload_folder.join(&path.0)).await {
        Ok(file) => file.into_response(&request),
        Err(_) => actix_web::HttpResponse::NotFound().finish(),
    }
}
