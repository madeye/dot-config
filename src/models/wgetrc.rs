use super::schema::{ConfigSchema, DotfileType, OptionDef, OptionKind};

pub fn wgetrc_schema() -> ConfigSchema {
    ConfigSchema {
        name: "Wget Config".into(),
        dotfile_type: DotfileType::Wgetrc,
        default_path: "~/.wgetrc".into(),
        options: vec![
            // Network
            OptionDef {
                key: "tries".into(),
                label: "Retries".into(),
                help: "Number of retries on failure (0 = infinite)".into(),
                section: "Network".into(),
                kind: OptionKind::Number { min: 0, max: 20, step: 1 },
                default: "3".into(),
            },
            OptionDef {
                key: "timeout".into(),
                label: "Timeout (sec)".into(),
                help: "Network timeout in seconds".into(),
                section: "Network".into(),
                kind: OptionKind::Number { min: 5, max: 300, step: 5 },
                default: "60".into(),
            },
            OptionDef {
                key: "waitretry".into(),
                label: "Wait Retry (sec)".into(),
                help: "Seconds to wait between retries".into(),
                section: "Network".into(),
                kind: OptionKind::Number { min: 0, max: 30, step: 1 },
                default: "1".into(),
            },
            OptionDef {
                key: "continue".into(),
                label: "Continue Downloads".into(),
                help: "Resume partially downloaded files".into(),
                section: "Network".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            // Download Behavior
            OptionDef {
                key: "timestamping".into(),
                label: "Timestamping".into(),
                help: "Only download newer files (like make)".into(),
                section: "Download".into(),
                kind: OptionKind::Toggle,
                default: "false".into(),
            },
            OptionDef {
                key: "no_parent".into(),
                label: "No Parent".into(),
                help: "Do not ascend to parent directories during recursive downloads".into(),
                section: "Download".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "adjust_extension".into(),
                label: "Adjust Extension".into(),
                help: "Add proper file extensions to HTML/CSS files".into(),
                section: "Download".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            // Security
            OptionDef {
                key: "check_certificate".into(),
                label: "Check Certificate".into(),
                help: "Verify SSL/TLS certificates".into(),
                section: "Security".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "https_only".into(),
                label: "HTTPS Only".into(),
                help: "Only follow HTTPS links in recursive downloads".into(),
                section: "Security".into(),
                kind: OptionKind::Toggle,
                default: "false".into(),
            },
            // Display
            OptionDef {
                key: "verbose".into(),
                label: "Verbose Output".into(),
                help: "Show detailed download progress".into(),
                section: "Display".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "server_response".into(),
                label: "Server Response".into(),
                help: "Print HTTP headers".into(),
                section: "Display".into(),
                kind: OptionKind::Toggle,
                default: "false".into(),
            },
            OptionDef {
                key: "dot_style".into(),
                label: "Dot Style".into(),
                help: "Progress indicator style for non-verbose mode".into(),
                section: "Display".into(),
                kind: OptionKind::Dropdown(vec![
                    ("default".into(), "Default".into()),
                    ("binary".into(), "Binary".into()),
                    ("mega".into(), "Mega".into()),
                    ("giga".into(), "Giga".into()),
                ]),
                default: "default".into(),
            },
        ],
    }
}
