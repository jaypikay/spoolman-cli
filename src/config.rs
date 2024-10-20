use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub spoolman: Spoolman,
    pub mqtt: Mqtt,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spoolman {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mqtt {
    pub clientid: String,
    pub topic: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            spoolman: Spoolman {
                url: "http://localhost:8000".to_string(),
            },
            mqtt: Mqtt {
                clientid: "SpoolMQ".to_string(),
                topic: "spoolman/spool".to_string(),
                host: "localhost".to_string(),
                port: 1883,
                username: "".to_string(),
                password: "".to_string(),
            },
        }
    }
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("spool").unwrap();
    let config_path = xdg_dirs
        .place_config_file("config.toml")
        .expect("cannot create configuration directory");

    let toml_str = match fs::read_to_string(config_path) {
        Ok(contents) => contents,
        Err(_) => {
            let default_config = Config::default();
            match toml::to_string(&default_config) {
                Ok(default_toml) => default_toml,
                Err(e) => {
                    eprintln!("Error serializing default config: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
    };

    let config: Config = match toml::from_str(&toml_str) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error parsing config: {}", e);
            return Err(Box::new(e));
        }
    };

    // #[cfg(debug_assertions)]
    // println!("{:#?}", config);

    Ok(config)
}
