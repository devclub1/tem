use crate::processor::Processor;
use crate::ProcessorArgs;
use std::env;
use std::process::Command;
use toml::Value;

pub struct GitProcessor;

impl Processor for GitProcessor {
    fn types(&self) -> String {
        "git".to_string()
    }

    fn help(&self) -> String {
        String::from(
            r#"
git processor usage

Configuration:

[git]
react-vite = ["git@github.com:axbg/react-vite-starter"]              # target repository, default branch
react-webpack = ["git@github.com:axbg/react-webpack-starter", "dev"] # target repository, dev branch


Execution:

tem react-vite           # clones into a directory with the same name as the repo
tem react-vite myProject # clones into a directory named myProject

"#,
        )
    }

    fn process(&self, prog_args: ProcessorArgs, config: &Value) -> bool {
        let mut args: Vec<&str> = vec![];

        if let Some(branch) = config.get(1).take() {
            args.push("-b");
            args.push(branch.as_str().unwrap());
        };

        args.push(
            config
                .get(0)
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
                    .get(0)
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
            .map(|_| {
                let _ = Command::new("rm")
                    .arg("-r")
                    .arg(format!("{}/.git", target_directory))
                    .output();

                true
            })
            .unwrap_or(false)
    }
}
