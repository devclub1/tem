use toml::Value;
use crate::Args;

pub trait Provider {
    fn types(&self) -> String;
    fn process(&self, prog_args: Args, config: &Value) -> bool;
}