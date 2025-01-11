mod config;
mod gitprocessor;
mod loader;
mod processor;

use clap::Parser;
use processor::Processor;
use std::collections::HashMap;
use std::fs::File;
use std::panic;
use std::path::Path;
use toml::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    template: String,
    project: Option<String>,
}

fn main() {
    bind_panic_handler();
    create_config_file(&config::build_default_home_config_path());

    let args = Args::parse();
    let processors = loader::load_processors();
    let configs = load_config();

    match execute_processor(processors, configs, args) {
        true => println!("\nhappy coding! (◕‿◕)"),
        false => println!("\nouch, something bad happened during processing :("),
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

fn create_config_file(config_path: &str) {
    let path = Path::new(&config_path);

    if !path.exists() {
        print!("Creating config file: {}", &config_path);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).expect("show:Couldn't create config hierarchy");
            }
        }

        File::create(path).expect("show:Couldn't create config file");
    }
}

fn load_config() -> HashMap<String, Value> {
    let home_path = config::build_default_home_config_path();
    let raw_config_file = config::load_raw_config(&home_path);

    config::load_toml_config(&raw_config_file)
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
