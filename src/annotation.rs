use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub inline: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    pub kind: String,
    pub content: String,
    pub location: Location,
}
