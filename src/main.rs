mod events;
mod providers;
mod filters;

use std::fs;
use std::path::PathBuf;
use today::Config;
use crate::providers::EventProvider;

fn main() {
    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            println!("Looking for config file'{}'", &toml_path.display());
            let config_str = fs::read_to_string(toml_path).expect("config file");
            let config: Config = toml::from_str(&config_str).expect("valid config file");
            //println!("config: {:?}", config);           
            if let Err(e) = today::run(&config, &path) {
                eprintln!("Error running program");
                return;
            }
        }
        None => {
            eprintln!("Unable to configure the application");
            return;
        }
    }
}

fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join(app_name);
        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            }
        }
        return Some(config_path);
    }
    None
}
