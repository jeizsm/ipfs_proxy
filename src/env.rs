use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_key: String,
    pub port: u16,
    pub host: String,
    pub ipfs_host: String,
    pub ipfs_port: u16,
}
