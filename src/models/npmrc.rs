use super::schema::{ConfigSchema, DotfileType, OptionDef, OptionKind};

pub fn npmrc_schema() -> ConfigSchema {
    ConfigSchema {
        name: "npm Config".into(),
        dotfile_type: DotfileType::Npmrc,
        default_path: "~/.npmrc".into(),
        options: vec![
            // Registry
            OptionDef {
                key: "registry".into(),
                label: "Registry".into(),
                help: "Base URL of the npm registry".into(),
                section: "Registry".into(),
                kind: OptionKind::Text,
                default: "https://registry.npmjs.org/".into(),
            },
            // Install Behavior
            OptionDef {
                key: "save-exact".into(),
                label: "Save Exact".into(),
                help: "Save dependencies with exact versions instead of ranges".into(),
                section: "Install".into(),
                kind: OptionKind::Toggle,
                default: "false".into(),
            },
            OptionDef {
                key: "save-prefix".into(),
                label: "Save Prefix".into(),
                help: "Version prefix for save operations".into(),
                section: "Install".into(),
                kind: OptionKind::Dropdown(vec![
                    ("^".into(), "^ (compatible)".into()),
                    ("~".into(), "~ (patch only)".into()),
                    ("".into(), "None (exact)".into()),
                ]),
                default: "^".into(),
            },
            OptionDef {
                key: "package-lock".into(),
                label: "Package Lock".into(),
                help: "Generate package-lock.json on install".into(),
                section: "Install".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "fund".into(),
                label: "Fund Messages".into(),
                help: "Show funding messages after install".into(),
                section: "Install".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "audit".into(),
                label: "Auto Audit".into(),
                help: "Run security audit on install".into(),
                section: "Install".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "progress".into(),
                label: "Progress Bar".into(),
                help: "Show progress bar during install".into(),
                section: "Display".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "loglevel".into(),
                label: "Log Level".into(),
                help: "Verbosity of npm output".into(),
                section: "Display".into(),
                kind: OptionKind::Dropdown(vec![
                    ("warn".into(), "Warn".into()),
                    ("notice".into(), "Notice".into()),
                    ("info".into(), "Info".into()),
                    ("verbose".into(), "Verbose".into()),
                ]),
                default: "warn".into(),
            },
            OptionDef {
                key: "engine-strict".into(),
                label: "Engine Strict".into(),
                help: "Refuse to install packages that fail engine requirements".into(),
                section: "Safety".into(),
                kind: OptionKind::Toggle,
                default: "false".into(),
            },
            OptionDef {
                key: "init-author-name".into(),
                label: "Author Name".into(),
                help: "Default author name for npm init".into(),
                section: "Init Defaults".into(),
                kind: OptionKind::Text,
                default: "".into(),
            },
            OptionDef {
                key: "init-author-email".into(),
                label: "Author Email".into(),
                help: "Default author email for npm init".into(),
                section: "Init Defaults".into(),
                kind: OptionKind::Text,
                default: "".into(),
            },
            OptionDef {
                key: "init-license".into(),
                label: "License".into(),
                help: "Default license for npm init".into(),
                section: "Init Defaults".into(),
                kind: OptionKind::Dropdown(vec![
                    ("MIT".into(), "MIT".into()),
                    ("ISC".into(), "ISC".into()),
                    ("Apache-2.0".into(), "Apache 2.0".into()),
                    ("GPL-3.0".into(), "GPL 3.0".into()),
                ]),
                default: "MIT".into(),
            },
        ],
    }
}
