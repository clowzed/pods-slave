use futures_util::StreamExt;
use std::io::Write;


#[actix_web::post("/hook")]
async fn endpoint(
    data: actix_web::web::Data<crate::config::Config>,
    episode: actix_web::web::Json<crate::episode::Episode>,
) -> impl actix_web::Responder {

    if let Some(master_node) = &data.master_node                        &&
       let Ok(stream_url) = master_node.ip.join(&format!("/stream/{}", &episode.filename)) &&
       let Ok(response) =  reqwest::get(stream_url.to_string()).await &&
              response.status()   == reqwest::StatusCode::ACCEPTED {

            //? Temporary filename protects from
            //? streaming partly downloaded file
            let temporary_filename = data.upload_folder.join(format!("{}_downloading", &episode.filename));

            if let Ok(mut temporary_file_descriptor) = std::fs::File::create(&temporary_filename) {
                let mut stream = response.bytes_stream();
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(c) => {
                            if let Err(e) = temporary_file_descriptor.write_all(&c) {
                                std::fs::remove_file(&temporary_filename).ok(); // Yeah I trust filesystem ʕっ•ᴥ•ʔっ
                                return actix_web::HttpResponse::UnprocessableEntity().body(e.to_string())
                            }
                        },
                        Err(e) => {
                            std::fs::remove_file(&temporary_filename).ok(); // Yeah I trust filesystem ʕっ•ᴥ•ʔっ
                            return actix_web::HttpResponse::UnprocessableEntity().body(e.to_string())
                        }
                    };
                }
                std::fs::rename(
                    temporary_filename,
                    data.upload_folder.join(&episode.filename),
                ).ok(); // Yeah I trust filesystem ʕっ•ᴥ•ʔっ
        }
    }
    actix_web::HttpResponse::BadRequest().finish()
}
