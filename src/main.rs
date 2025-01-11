use clap::Parser;
use home::home_dir;
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::panic;
use std::path::Path;
use std::process::{exit, Command};
use std::{env, fs};
use toml::Value;

const CONFIG_LOCATION: &str = "/.config/temple/config.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    template: String,
    project: Option<String>,
}

trait Provider {
    fn types(&self) -> String;
    fn process(&self, prog_args: Args, config: &Value) -> bool;
}

struct GitProvider {}
impl Provider for GitProvider {
    fn types(&self) -> String {
        return "git".to_string();
    }

    fn process(&self, prog_args: Args, config: &Value) -> bool {
        let mut args: Vec<&str> = vec![];

        match config.get("branch").take() {
            Some(branch) => {
                args.push("-b");
                args.push(branch.as_str().unwrap());
            }
            None => {}
        };

        args.push(
            config
                .get("source")
                .take()
                .expect("show:git: source parameter is mandatory")
                .as_str()
                .unwrap(),
        );

        match &prog_args.project {
            Some(directory) => {
                args.push(directory);
            }
            None => {}
        };

        Command::new("git")
            .arg("clone")
            .args(args)
            .current_dir(env::current_dir().unwrap())
            .exec();

        return true;
    }
}

fn main() {
    bind_panic_handler();
    create_config_file();

    let args = Args::parse();
    let processors = initialize_processors();
    let configs = load_config();

    match execute_provider(processors, configs, args) {
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

fn initialize_processors() -> HashMap<String, Box<dyn Provider>> {
    let mut processors: HashMap<String, Box<dyn Provider>> = HashMap::new();

    let git_provider: Box<dyn Provider> = Box::new(GitProvider {});
    processors.insert(git_provider.types(), git_provider);

    return processors;
}

fn create_config_file() {
    let home_path = home_dir().expect("show:Failed to read home dir");
    let config_file = format!("{}{}", home_path.display(), CONFIG_LOCATION);

    let path = Path::new(&config_file);

    if !path.exists() {
        print!("Creating config file: {}", &config_file);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent);
            }
        }

        File::create(path);
    }
}

fn load_config() -> HashMap<String, Value> {
    let home_path = home_dir().expect("show:Failed to read home dir");
    let config_file = format!("{}{}", home_path.display(), CONFIG_LOCATION);
    let unparsed_config =
        fs::read_to_string(&config_file).expect("show:Failed to read config file");
    let configs: HashMap<String, Value> =
        toml::from_str(&unparsed_config).expect("show:Failed to parse config file as TOML");

    if configs.len() == 0 {
        println!("Config file is empty");
        exit(0);
    }

    return configs;
}

fn execute_provider(
    providers: HashMap<String, Box<dyn Provider>>,
    configs: HashMap<String, Value>,
    args: Args,
) -> bool {
    for (section, value) in &configs {
        if let Value::Table(table) = value {
            if table.contains_key(&args.template) && providers.contains_key(section) {
                let provider = providers.get(section).take().unwrap();
                let config: &Value = table.get(&args.template).unwrap();
                return provider.process(args, config);
            }
        }
    }

    return false;
}
