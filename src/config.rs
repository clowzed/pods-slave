#[derive(Debug, Clone)]
pub struct Config {
    pub upload_folder: std::path::PathBuf,
    pub port: u16,
    pub service_registry: url::Url,
    pub access_token: String,
    pub master_node: Option<tellme_client::Service>
}

impl Config {
    pub fn init() -> Config {
        let upload_folder = std::env::var("UPLOAD_FOLDER").expect("UPLOAD_FOLDER must be set");
        let port = std::env::var("PORT").expect("PORT must be set");
        let service_registry = std::env::var("SERVICE_REGISTRY").expect("SERVICE_REGISTRY should be set");
        let access_token = std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");

        Config {
            upload_folder: std::path::PathBuf::from(upload_folder),
            port: port
                .parse::<u16>()
                .expect("Failed to parse port number to u16"),
            service_registry: url::Url::parse(&service_registry).expect("Failed to parse SERVICE_REGISTRY url"),
            access_token,
            master_node: None
        }
    }
}
