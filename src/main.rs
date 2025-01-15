mod commands;
mod config;
mod gitprocessor;
mod loader;
mod parsers;
mod processor;

use crate::commands::Commands;
use crate::parsers::{parse_commands, ProcessorArgs};
use clap::Parser;
use processor::Processor;
use std::collections::HashMap;
use std::panic;
use toml::Value;

fn main() {
    bind_panic_handler();

    let processors = loader::load_processors();

    let unparsed_args: Vec<String> = std::env::args().collect();

    if unparsed_args.len() < 2 || Commands::is_predefined_command(&unparsed_args[1]) {
        let matches = parse_commands(&processors);
        match matches.subcommand() {
            Some((Commands::INIT, args)) => Commands::init(args),
            Some((Commands::CONFIG, _)) => Commands::config(),
            Some((Commands::PROC, args)) => Commands::proc(args, &processors),
            Some((_, _)) => {}
            None => {}
        }
    } else {
        let args = ProcessorArgs::parse();
        let configs = load_config();

        match execute_processor(processors, configs, args) {
            true => println!("\nhappy coding! (◕‿◕)"),
            false => println!("\nouch, something bad happened during processing :("),
        }
    }
}

fn bind_panic_handler() {
    panic::set_hook(Box::new(|info| {
        if let Some(message) = info.payload().downcast_ref::<String>() {
            if message.starts_with("show:") {
                println!("{}", &message.strip_prefix("show:").unwrap());
                return;
            }
        }

        println!("{}", info);
    }));
}

fn load_config() -> HashMap<String, Value> {
    let home_path = config::build_default_home_config_path();
    let raw_config_file = config::load_raw_config(&home_path);

    config::load_toml_config(&raw_config_file)
}

fn execute_processor(
    processors: HashMap<String, Box<dyn Processor>>,
    configs: HashMap<String, Value>,
    args: ProcessorArgs,
) -> bool {
    for (section, value) in &configs {
        if let Value::Table(table) = value {
            if table.contains_key(&args.template) && processors.contains_key(section) {
                if let Some(processor) = processors.get(section).take() {
                    let config: &Value = table.get(&args.template).unwrap();
                    return processor.process(args, config);
                }
            }
        }
    }

    config::print_and_exit(&format!("Configuration {} was not found", &args.template));
    false
}
