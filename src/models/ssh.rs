use super::schema::{ConfigSchema, DotfileType, OptionDef, OptionKind};

pub fn ssh_host_schema() -> ConfigSchema {
    ConfigSchema {
        name: "SSH Config".into(),
        dotfile_type: DotfileType::Ssh,
        default_path: "~/.ssh/config".into(),
        options: vec![
            // Host Entry
            OptionDef {
                key: "Host".into(),
                label: "Host Alias".into(),
                help: "Alias for this SSH host entry".into(),
                section: "Connection".into(),
                kind: OptionKind::Text,
                default: "".into(),
            },
            OptionDef {
                key: "HostName".into(),
                label: "Hostname".into(),
                help: "Actual hostname or IP address".into(),
                section: "Connection".into(),
                kind: OptionKind::Text,
                default: "".into(),
            },
            OptionDef {
                key: "User".into(),
                label: "Username".into(),
                help: "SSH username".into(),
                section: "Connection".into(),
                kind: OptionKind::Text,
                default: "".into(),
            },
            OptionDef {
                key: "Port".into(),
                label: "Port".into(),
                help: "SSH port number".into(),
                section: "Connection".into(),
                kind: OptionKind::Number { min: 1, max: 65535, step: 1 },
                default: "22".into(),
            },
            OptionDef {
                key: "IdentityFile".into(),
                label: "Identity File".into(),
                help: "Path to SSH private key".into(),
                section: "Authentication".into(),
                kind: OptionKind::Text,
                default: "~/.ssh/id_ed25519".into(),
            },
            OptionDef {
                key: "AddKeysToAgent".into(),
                label: "Add Keys to Agent".into(),
                help: "Automatically add keys to SSH agent".into(),
                section: "Authentication".into(),
                kind: OptionKind::Dropdown(vec![
                    ("yes".into(), "Yes".into()),
                    ("no".into(), "No".into()),
                    ("ask".into(), "Ask".into()),
                    ("confirm".into(), "Confirm".into()),
                ]),
                default: "yes".into(),
            },
            OptionDef {
                key: "ForwardAgent".into(),
                label: "Forward Agent".into(),
                help: "Forward SSH agent to remote host".into(),
                section: "Authentication".into(),
                kind: OptionKind::Toggle,
                default: "false".into(),
            },
            OptionDef {
                key: "StrictHostKeyChecking".into(),
                label: "Host Key Checking".into(),
                help: "How to handle unknown host keys".into(),
                section: "Security".into(),
                kind: OptionKind::Dropdown(vec![
                    ("accept-new".into(), "Accept new keys".into()),
                    ("ask".into(), "Ask (default)".into()),
                    ("yes".into(), "Strict (reject unknown)".into()),
                    ("no".into(), "No checking".into()),
                ]),
                default: "accept-new".into(),
            },
            OptionDef {
                key: "ServerAliveInterval".into(),
                label: "Keep-Alive Interval".into(),
                help: "Seconds between keep-alive messages (0=disabled)".into(),
                section: "Security".into(),
                kind: OptionKind::Number { min: 0, max: 300, step: 15 },
                default: "60".into(),
            },
            OptionDef {
                key: "ServerAliveCountMax".into(),
                label: "Keep-Alive Count".into(),
                help: "Max keep-alive messages before disconnect".into(),
                section: "Security".into(),
                kind: OptionKind::Number { min: 1, max: 10, step: 1 },
                default: "3".into(),
            },
        ],
    }
}
