use crate::Args;
use toml::Value;

pub trait Processor {
    fn types(&self) -> String;
    fn process(&self, prog_args: Args, config: &Value) -> bool;
}