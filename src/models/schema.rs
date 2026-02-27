use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OptionKind {
    Dropdown(Vec<(String, String)>),
    Toggle,
    Number { min: i64, max: i64, step: i64 },
    Text,
    TextArea,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OptionDef {
    pub key: String,
    pub label: String,
    pub help: String,
    pub section: String,
    pub kind: OptionKind,
    pub default: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DotfileType {
    Vimrc,
    Shellrc,
    Gitconfig,
    Tmux,
    Ssh,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ConfigSchema {
    pub name: String,
    pub dotfile_type: DotfileType,
    pub default_path: String,
    pub options: Vec<OptionDef>,
}

impl ConfigSchema {
    pub fn sections(&self) -> Vec<String> {
        let mut sections = Vec::new();
        for opt in &self.options {
            if !sections.contains(&opt.section) {
                sections.push(opt.section.clone());
            }
        }
        sections
    }

    pub fn options_for_section(&self, section: &str) -> Vec<&OptionDef> {
        self.options.iter().filter(|o| o.section == section).collect()
    }

    pub fn defaults_map(&self) -> std::collections::HashMap<String, String> {
        self.options
            .iter()
            .map(|o| (o.key.clone(), o.default.clone()))
            .collect()
    }
}
