mod gitprocessor;
mod loader;
mod processor;

use clap::Parser;
use home::home_dir;
use processor::Processor;
use std::collections::HashMap;
use std::fs::File;
use std::panic;
use std::path::Path;
use std::process::exit;
use toml::Value;

const CONFIG_LOCATION: &str = "/.config/temple/config.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    template: String,
    project: Option<String>,
}

fn main() {
    bind_panic_handler();
    create_config_file();

    let args = Args::parse();
    let processors = loader::load_processors();
    let configs = load_config();

    match execute_processor(processors, configs, args) {
        true => println!("Success"),
        false => println!("Error"),
    }
}

fn bind_panic_handler() {
    panic::set_hook(Box::new(|info| {
        if let Some(message) = info.payload().downcast_ref::<String>() {
            if message.starts_with("show:") {
                println!("{}", &message[5..]);
                return;
            }
        }

        println!("{}", info);
    }));
}

fn create_config_file() {
    let home_path = home_dir().expect("show:Failed to read home dir");
    let config_file = format!("{}{}", home_path.display(), CONFIG_LOCATION);

    let path = Path::new(&config_file);

    if !path.exists() {
        print!("Creating config file: {}", &config_file);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).expect("show:Couldn't create config hierarchy");
            }
        }

        File::create(path).expect("show:Couldn't create config file");
    }
}

fn load_config() -> HashMap<String, Value> {
    let home_path = home_dir().expect("show:Failed to read home dir");
    let config_file = format!("{}{}", home_path.display(), CONFIG_LOCATION);
    let unparsed_config =
        std::fs::read_to_string(&config_file).expect("show:Failed to read config file");
    let configs: HashMap<String, Value> =
        toml::from_str(&unparsed_config).expect("show:Failed to parse config file as TOML");

    if configs.len() == 0 {
        println!("Config file is empty");
        exit(0);
    }

    configs
}

fn execute_processor(
    processors: HashMap<String, Box<dyn Processor>>,
    configs: HashMap<String, Value>,
    args: Args,
) -> bool {
    for (section, value) in &configs {
        if let Value::Table(table) = value {
            if table.contains_key(&args.template) && processors.contains_key(section) {
                match processors.get(section).take() {
                    Some(processor) => {
                        let config: &Value = table.get(&args.template).unwrap();
                        return processor.process(args, config);
                    }
                    None => {}
                }
            }
        }
    }

    false
}
