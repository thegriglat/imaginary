use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub redis_url: String,
}

impl Config {
    pub fn read() -> Self {
        dotenv().ok();
        let port = Config::port();
        let redis_url = Config::get_env_var("REDIS_URL").expect("REDIS_URL is not set");

        let config = Config { port, redis_url };
        config.dump();
        config
    }

    fn port() -> u16 {
        let default_port = 8080;

        Config::get_env_var("PORT")
            .unwrap_or(default_port.to_string())
            .parse::<u16>()
            .unwrap_or(default_port)
    }

    fn get_env_var(key: &str) -> Option<String> {
        match env::vars().find(|x| x.0 == key) {
            Some(pair) => Some(pair.1),
            None => None,
        }
    }

    fn dump(&self) {
        println!("{:#?}", self);
    }
}
