use super::schema::{ConfigSchema, DotfileType, OptionDef, OptionKind};

pub fn gitconfig_schema() -> ConfigSchema {
    ConfigSchema {
        name: "Git Config".into(),
        dotfile_type: DotfileType::Gitconfig,
        default_path: "~/.gitconfig".into(),
        options: vec![
            // User
            OptionDef {
                key: "user.name".into(),
                label: "User Name".into(),
                help: "Your name for commit authorship".into(),
                section: "User".into(),
                kind: OptionKind::Text,
                default: "".into(),
            },
            OptionDef {
                key: "user.email".into(),
                label: "Email".into(),
                help: "Your email for commit authorship".into(),
                section: "User".into(),
                kind: OptionKind::Text,
                default: "".into(),
            },
            // Core
            OptionDef {
                key: "core.editor".into(),
                label: "Editor".into(),
                help: "Default editor for commit messages".into(),
                section: "Core".into(),
                kind: OptionKind::Dropdown(vec![
                    ("vim".into(), "Vim".into()),
                    ("nvim".into(), "Neovim".into()),
                    ("nano".into(), "Nano".into()),
                    ("emacs".into(), "Emacs".into()),
                    ("code --wait".into(), "VS Code".into()),
                    ("subl -n -w".into(), "Sublime Text".into()),
                ]),
                default: "vim".into(),
            },
            OptionDef {
                key: "core.autocrlf".into(),
                label: "Auto CRLF".into(),
                help: "Handle line endings automatically".into(),
                section: "Core".into(),
                kind: OptionKind::Dropdown(vec![
                    ("input".into(), "Input (convert on commit)".into()),
                    ("true".into(), "True (convert both ways)".into()),
                    ("false".into(), "False (no conversion)".into()),
                ]),
                default: "input".into(),
            },
            OptionDef {
                key: "core.whitespace".into(),
                label: "Whitespace Handling".into(),
                help: "How to handle trailing whitespace".into(),
                section: "Core".into(),
                kind: OptionKind::Dropdown(vec![
                    ("fix".into(), "Fix trailing whitespace".into()),
                    ("warn".into(), "Warn about whitespace".into()),
                    ("nowarn".into(), "No warnings".into()),
                ]),
                default: "fix".into(),
            },
            // Branch & Merge
            OptionDef {
                key: "init.defaultBranch".into(),
                label: "Default Branch".into(),
                help: "Default branch name for new repos".into(),
                section: "Branch & Merge".into(),
                kind: OptionKind::Dropdown(vec![
                    ("main".into(), "main".into()),
                    ("master".into(), "master".into()),
                    ("trunk".into(), "trunk".into()),
                    ("develop".into(), "develop".into()),
                ]),
                default: "main".into(),
            },
            OptionDef {
                key: "pull.rebase".into(),
                label: "Pull Rebase".into(),
                help: "Rebase on pull instead of merge".into(),
                section: "Branch & Merge".into(),
                kind: OptionKind::Dropdown(vec![
                    ("false".into(), "Merge (default)".into()),
                    ("true".into(), "Rebase".into()),
                    ("merges".into(), "Rebase preserving merges".into()),
                ]),
                default: "false".into(),
            },
            OptionDef {
                key: "push.default".into(),
                label: "Push Default".into(),
                help: "Default push behavior".into(),
                section: "Branch & Merge".into(),
                kind: OptionKind::Dropdown(vec![
                    ("simple".into(), "Simple (current to same-named)".into()),
                    ("current".into(), "Current branch".into()),
                    ("upstream".into(), "Upstream tracking".into()),
                    ("matching".into(), "All matching branches".into()),
                ]),
                default: "simple".into(),
            },
            OptionDef {
                key: "merge.conflictstyle".into(),
                label: "Conflict Style".into(),
                help: "Style for displaying merge conflicts".into(),
                section: "Branch & Merge".into(),
                kind: OptionKind::Dropdown(vec![
                    ("merge".into(), "Standard (2-way)".into()),
                    ("diff3".into(), "Diff3 (3-way with base)".into()),
                    ("zdiff3".into(), "Zdiff3 (zealous diff3)".into()),
                ]),
                default: "diff3".into(),
            },
            // UI
            OptionDef {
                key: "color.ui".into(),
                label: "Color Output".into(),
                help: "Colorize git output".into(),
                section: "UI".into(),
                kind: OptionKind::Dropdown(vec![
                    ("auto".into(), "Auto".into()),
                    ("always".into(), "Always".into()),
                    ("false".into(), "Never".into()),
                ]),
                default: "auto".into(),
            },
            OptionDef {
                key: "help.autocorrect".into(),
                label: "Auto-correct".into(),
                help: "Auto-correct mistyped commands (deciseconds to wait)".into(),
                section: "UI".into(),
                kind: OptionKind::Number { min: 0, max: 50, step: 5 },
                default: "0".into(),
            },
            OptionDef {
                key: "rerere.enabled".into(),
                label: "Rerere".into(),
                help: "Remember how you resolved conflicts".into(),
                section: "UI".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
        ],
    }
}
