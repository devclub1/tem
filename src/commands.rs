use std::collections::HashMap;
use clap::ArgMatches;
use crate::config;
use crate::config::{build_default_home_config_path, create_config_file};
use crate::processor::Processor;

static PREDEFINED_COMMANDS: [&str; 5] = [Commands::INIT, Commands::CONFIG, Commands::PROC, "--help", "-h"];

pub struct Commands;
impl Commands {
    pub const INIT: &'static str = "init";
    pub const CONFIG: &'static str = "config";
    pub const PROC: &'static str = "proc";

    pub fn init(args: &ArgMatches) {
        create_config_file(&build_default_home_config_path(), args.get_flag("force"));
    }

    pub fn config() {
        println!(
            "{}",
            config::load_raw_config(&build_default_home_config_path())
        );
    }

    pub fn proc(args: &ArgMatches, processors: &HashMap<String, Box<dyn Processor>>) {
        if let Some(proc) = args.get_one::<String>("processor") {
            if let Some(processor) = processors.get(proc) {
                println!("{}", processor.help());
            }
        }
    }

    pub fn is_predefined_command(command: &str) -> bool {
        PREDEFINED_COMMANDS.contains(&command)
    }
}
