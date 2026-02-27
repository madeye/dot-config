use super::schema::{ConfigSchema, DotfileType, OptionDef, OptionKind};

pub fn tmux_schema() -> ConfigSchema {
    ConfigSchema {
        name: "Tmux".into(),
        dotfile_type: DotfileType::Tmux,
        default_path: "~/.tmux.conf".into(),
        options: vec![
            // General
            OptionDef {
                key: "prefix".into(),
                label: "Prefix Key".into(),
                help: "The tmux prefix key combination".into(),
                section: "General".into(),
                kind: OptionKind::Dropdown(vec![
                    ("C-b".into(), "Ctrl-b (default)".into()),
                    ("C-a".into(), "Ctrl-a (screen-like)".into()),
                    ("C-Space".into(), "Ctrl-Space".into()),
                    ("C-s".into(), "Ctrl-s".into()),
                ]),
                default: "C-b".into(),
            },
            OptionDef {
                key: "mouse".into(),
                label: "Mouse Support".into(),
                help: "Enable mouse for scrolling and pane selection".into(),
                section: "General".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "base-index".into(),
                label: "Base Index".into(),
                help: "Starting index for window numbering".into(),
                section: "General".into(),
                kind: OptionKind::Number { min: 0, max: 1, step: 1 },
                default: "1".into(),
            },
            OptionDef {
                key: "pane-base-index".into(),
                label: "Pane Base Index".into(),
                help: "Starting index for pane numbering".into(),
                section: "General".into(),
                kind: OptionKind::Number { min: 0, max: 1, step: 1 },
                default: "1".into(),
            },
            OptionDef {
                key: "history-limit".into(),
                label: "History Limit".into(),
                help: "Maximum scrollback buffer size".into(),
                section: "General".into(),
                kind: OptionKind::Number { min: 1000, max: 50000, step: 1000 },
                default: "5000".into(),
            },
            OptionDef {
                key: "escape-time".into(),
                label: "Escape Time (ms)".into(),
                help: "Time to wait after escape key (lower = faster)".into(),
                section: "General".into(),
                kind: OptionKind::Number { min: 0, max: 1000, step: 10 },
                default: "0".into(),
            },
            OptionDef {
                key: "renumber-windows".into(),
                label: "Renumber Windows".into(),
                help: "Renumber windows when one is closed".into(),
                section: "General".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            // Display
            OptionDef {
                key: "default-terminal".into(),
                label: "Terminal Type".into(),
                help: "Default terminal type".into(),
                section: "Display".into(),
                kind: OptionKind::Dropdown(vec![
                    ("screen-256color".into(), "screen-256color".into()),
                    ("tmux-256color".into(), "tmux-256color".into()),
                    ("xterm-256color".into(), "xterm-256color".into()),
                ]),
                default: "screen-256color".into(),
            },
            OptionDef {
                key: "status-position".into(),
                label: "Status Bar Position".into(),
                help: "Position of the status bar".into(),
                section: "Display".into(),
                kind: OptionKind::Dropdown(vec![
                    ("bottom".into(), "Bottom".into()),
                    ("top".into(), "Top".into()),
                ]),
                default: "bottom".into(),
            },
            OptionDef {
                key: "display-time".into(),
                label: "Display Time (ms)".into(),
                help: "Time to display status messages".into(),
                section: "Display".into(),
                kind: OptionKind::Number { min: 500, max: 5000, step: 500 },
                default: "2000".into(),
            },
            OptionDef {
                key: "monitor-activity".into(),
                label: "Monitor Activity".into(),
                help: "Highlight windows with activity".into(),
                section: "Display".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "visual-activity".into(),
                label: "Visual Activity".into(),
                help: "Show message on activity instead of bell".into(),
                section: "Display".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
        ],
    }
}
