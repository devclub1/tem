mod config;
mod gitprocessor;
mod loader;
mod processor;

use clap::Arg;
use clap::Command;
use clap::{ArgMatches, Parser};
use processor::Processor;
use std::collections::HashMap;
use std::collections::HashSet;
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

struct Commands;
impl Commands {
    pub const PROC: &str = "proc";
    pub const INIT: &str = "init";
    pub const CONFIG: &str = "config";
}

fn main() {
    bind_panic_handler();

    let processors = loader::load_processors();

    let unparsed_args: Vec<String> = std::env::args().collect();
    if unparsed_args.len() < 2 || is_predefined_value(&unparsed_args[1]) {
        let matches = build_commands_interface(&processors);
        match matches.subcommand() {
            Some((Commands::INIT, args)) => {
                create_config_file(
                    &config::build_default_home_config_path(),
                    args.get_flag("force"),
                );
            }
            Some((Commands::CONFIG, _)) => {}
            Some((Commands::PROC, _)) => {}
            Some((_, _)) => {}
            None => {}
        }
    } else {
        let args = Args::parse();
        let configs = load_config();

        match execute_processor(processors, configs, args) {
            true => println!("\nhappy coding! (◕‿◕)"),
            false => println!("\nouch, something bad happened during processing :("),
        }
    }
}

fn is_predefined_value(command: &str) -> bool {
    command == Commands::CONFIG || command == Commands::INIT || command == Commands::PROC
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

fn build_commands_interface(processors: &HashMap<String, Box<dyn Processor>>) -> ArgMatches {
    let processors_set: HashSet<String> = processors
        .values()
        .map(|obj| obj.as_ref().types().to_string())
        .collect();

    let valid_processors = processors_set
        .iter()
        .cloned()
        .collect::<Vec<String>>()
        .join(", ");

    Command::new("tem")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new(Commands::INIT)
                .about("Initializes tem config file")
                .arg(
                    Arg::new("force")
                        .long("force")
                        .required(false)
                        .help("Recreate config file if already existing")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new(Commands::CONFIG).about("Lists the current config file"))
        .subcommand(
            Command::new(Commands::PROC)
                .about("Prints all available processors or information about a specific processor")
                .arg(
                    Arg::new("name")
                        .value_name("Processor name")
                        .help(format!(
                            "The 'name' argument must be one of the following valid processors: {}",
                            &valid_processors
                        ))
                        .value_parser(move |val: &str| {
                            if processors_set.contains(val) {
                                Ok(val.to_string())
                            } else {
                                Err(format!(
                                    "Invalid processor name '{}'. Valid values are: {}",
                                    val, &valid_processors
                                ))
                            }
                        })
                        .required(false),
                ),
        )
        .get_matches()
}

fn create_config_file(config_path: &str, force: bool) {
    let path = Path::new(&config_path);

    if !path.exists() || force {
        print!("Creating config file: {}", &config_path);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).expect("show:Couldn't create config hierarchy");
            }
        }

        File::create(path).expect("show:Couldn't create config file");
    } else {
        print!("Config file already exists. Run the command using --force to recreate it");
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
