use crate::commands::Commands;
use crate::processor::Processor;
use clap::{Arg, ArgMatches, Command, Parser};
use std::collections::{HashMap, HashSet};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ProcessorArgs {
    pub template: String,
    pub project: Option<String>,
}

pub fn parse_commands(processors: &HashMap<String, Box<dyn Processor>>) -> ArgMatches {
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
        .arg_required_else_help(true)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new(Commands::INIT)
                .about("initializes tem config file")
                .arg(
                    Arg::new("force")
                        .long("force")
                        .required(false)
                        .help("Recreate config file if already existing")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new(Commands::CONFIG).about("lists the current config file"))
        .subcommand(
            Command::new(Commands::PROC)
                .about("prints information about a specific processor")
                .arg(
                    Arg::new("processor")
                        .value_name("processor")
                        .help(format!(
                            "must be one of the following valid processors: {}",
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
                        .required(true),
                ),
        )
        .subcommand(Command::new("<<configuration>>").about("executes a specific configuration"))
        .get_matches()
}

