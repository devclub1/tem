use crate::processor::Processor;
use crate::Args;
use std::env;
use std::process::Command;
use toml::Value;

pub struct GitProcessor;

impl Processor for GitProcessor {
    fn types(&self) -> String {
        "git".to_string()
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

        let target_directory = match &prog_args.project {
            Some(directory) => {
                args.push(directory);
                directory.to_string()
            }
            None => {
                let repo_name = config
                    .get("source")
                    .unwrap()
                    .to_string()
                    .split('/')
                    .last()
                    .unwrap()
                    .to_string();

                if repo_name.ends_with(".git") {
                    repo_name
                        .rsplit_once(".")
                        .map(|(first, _)| first)
                        .unwrap()
                        .to_string()
                } else {
                    repo_name
                }
            }
        };

        Command::new("git")
            .arg("clone")
            .args(args)
            .current_dir(env::current_dir().unwrap())
            .status()
            .ok()
            .filter(|status| status.success())
            .and_then(|_| {
                let _ = Command::new("rm")
                    .arg("-r")
                    .arg(format!("{}/.git", target_directory))
                    .output();

                Some(true)
            })
            .unwrap_or(false)
    }
}
