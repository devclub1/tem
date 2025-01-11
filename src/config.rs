use home::home_dir;
use std::collections::HashMap;
use std::process::exit;
use toml::Value;

const CONFIG_LOCATION: &str = "/.config/tem/config.toml";

pub fn build_default_home_config_path() -> String {
    build_home_config_path(CONFIG_LOCATION)
}

pub fn build_home_config_path(partial_location: &str) -> String {
    let home_path = home_dir().expect("show:Failed to read home dir");

    format!("{}{}", home_path.display(), partial_location)
}

pub fn load_raw_config(config_location: &str) -> String {
    std::fs::read_to_string(&config_location).expect("show:Failed to read config file")
}

pub fn load_toml_config(unparsed_config: &str) -> HashMap<String, Value> {
    let configs: HashMap<String, Value> =
        toml::from_str(&unparsed_config).expect("show:Failed to parse config file as TOML");

    if configs.len() == 0 {
        print_and_exit("Config file is empty");
    }

    configs
}

pub fn print_and_exit(message: &str) {
    println!("\n{}", message);
    exit(0);
}
