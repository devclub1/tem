use crate::processor::Processor;
use std::collections::HashMap;

pub fn load_processors() -> HashMap<String, Box<dyn Processor>> {
    let mut processors: HashMap<String, Box<dyn Processor>> = HashMap::new();

    let git_processor: Box<dyn Processor> = Box::new(crate::gitprocessor::GitProcessor {});
    processors.insert(git_processor.types(), git_processor);

    processors
}