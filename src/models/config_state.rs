use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::schema::DotfileType;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DotfileState {
    pub values: HashMap<String, String>,
    pub extra_lines: Vec<String>,
    pub file_exists: bool,
}

impl DotfileState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_defaults(defaults: HashMap<String, String>) -> Self {
        Self {
            values: defaults,
            extra_lines: Vec::new(),
            file_exists: false,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn get_or_default<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.values.get(key).map(|s| s.as_str()).unwrap_or(default)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetectedDotfile {
    pub dotfile_type: DotfileType,
    pub name: String,
    pub path: String,
    pub exists: bool,
}
