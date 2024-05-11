use crate::config::Config;
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: Client,
    pub config: Config,
}
