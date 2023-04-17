#![feature(let_chains)]

mod config;
mod episode;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut config = config::Config::init();

    match std::fs::create_dir(&config.upload_folder) {
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => panic!("Failed to create directory! Reason: {:?}", e),
        _ => {}
    }

    let port = config.port;

    //? Registering self in servise registry and try to find master node
    let tclient = tellme_client::TellmeClient::new(config.service_registry.clone(), None, None);

    let _self_identifier = tclient
        .register(
            config.port,
            "api/health".to_string(),
            config.access_token.clone(),
            "slave".to_owned(),
        )
        .await
        .expect("Failed to register self in service registry");

    config.master_node = Some(
        tclient
            .find(Some("master".to_owned()), Some(1), Some(true))
            .await
            .expect("Failed to find master node in service registry")
            .get(0)
            .expect("There are no available master nodes in service registry")
            .clone(),
    );

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(actix_web::web::Data::new(config.clone()))
            .service(services::health::health)
            .service(services::episode::stream)
            .service(services::hook::endpoint)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
