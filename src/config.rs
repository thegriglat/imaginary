use std::env;

use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub workers: usize,
}

impl Config {
    pub fn read() -> Self {
        dotenv().ok();
        let port = Config::port();
        let workers = Config::workers();

        let config = Config { port, workers };
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

    fn workers() -> usize {
        match Config::get_env_var("WORKERS") {
            Some(value) => value.parse::<usize>().expect("Cannot parse WORKERS"),
            None => {
                let cpus = num_cpus::get_physical();
                println!(
                    "Cannot parse WORKERS variable. Will use all available CPUs ({})",
                    cpus
                );
                cpus
            }
        }
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
