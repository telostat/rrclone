use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub tasks: Vec<Task>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub source: Source,
    pub target: Target,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub backend: Backend,
    pub path: String,
    pub filters: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub backend: Backend,
    pub path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Backend {
    #[serde(rename = "type")]
    pub ctype: String,
    pub vars: Option<HashMap<String, String>>,
}

pub fn read_config(path: &String) -> Result<Config, String> {
    match fs::read_to_string(path) {
        Ok(content) => match serde_yaml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(err) => Err(format!("Can not parse configuration. Error: {:?}", err)),
        },
        Err(err) => Err(format!("Can not read file {}. Error: {}", path, err)),
    }
}
