use home::home_dir;
use std::collections::HashMap;
use std::process::exit;
use toml::Value;

const CONFIG_LOCATION: &str = "/.config/temple/config.toml";

pub fn build_default_home_config_path() -> String {
    build_home_config_path(None)
}

pub fn build_home_config_path(partial_location: Option<&str>) -> String {
    let home_path = home_dir().expect("show:Failed to read home dir");

    format!("{}{}", home_path.display(), partial_location.unwrap_or(CONFIG_LOCATION))
}

pub fn load_raw_config(config_location: &str) -> String {
    std::fs::read_to_string(&config_location).expect("show:Failed to read config file")
}

pub fn load_toml_config(unparsed_config: &str) -> HashMap<String, Value> {
    let configs: HashMap<String, Value> =
        toml::from_str(&unparsed_config).expect("show:Failed to parse config file as TOML");

    if configs.len() == 0 {
        println!("Config file is empty");
        exit(0);
    }

    configs
}