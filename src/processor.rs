use crate::ProcessorArgs;
use toml::Value;

pub trait Processor {
    fn types(&self) -> String;
    fn help(&self) -> String;
    fn process(&self, prog_args: ProcessorArgs, config: &Value) -> bool;
}

