use std::env;
use std::process::Command;
use toml::Value;
use crate::Args;
use crate::provider::Provider;

pub struct GitProvider {}
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
            .output()
            .expect("show:Repository cloning failed");

        true
    }
}