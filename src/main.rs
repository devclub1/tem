use clap::Parser;
use home::home_dir;
use std::collections::HashMap;
use std::fs;
use toml::Value;

const CONFIG_LOCATION: &str = "/.config/temple/config.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    template: String,
    project: Option<String>,
}

fn main() {
    let args = Args::parse();

    let configs = load_config();
}

fn load_config() -> HashMap<String, Value> {
    let home_path = home_dir().expect("Failed to read home dir");
    let config_file = format!("{}{}", home_path.display(), CONFIG_LOCATION);
    let unparsed_config = fs::read_to_string(&config_file).expect("Failed to read config file");
    let configs: HashMap<String, Value> =
        toml::from_str(&unparsed_config).expect("Failed to parse config file as TOML");

    return configs;
}

