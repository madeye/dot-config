use super::schema::{ConfigSchema, DotfileType, OptionDef, OptionKind};

pub fn editorconfig_schema() -> ConfigSchema {
    ConfigSchema {
        name: "EditorConfig".into(),
        dotfile_type: DotfileType::Editorconfig,
        default_path: "~/.editorconfig".into(),
        options: vec![
            // Global
            OptionDef {
                key: "root".into(),
                label: "Root".into(),
                help: "Set to true to stop .editorconfig search at this file".into(),
                section: "Global".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            // Defaults (applied to all files via [*])
            OptionDef {
                key: "indent_style".into(),
                label: "Indent Style".into(),
                help: "Use hard tabs or soft spaces for indentation".into(),
                section: "Defaults".into(),
                kind: OptionKind::Dropdown(vec![
                    ("space".into(), "Spaces".into()),
                    ("tab".into(), "Tabs".into()),
                ]),
                default: "space".into(),
            },
            OptionDef {
                key: "indent_size".into(),
                label: "Indent Size".into(),
                help: "Number of columns for each indentation level".into(),
                section: "Defaults".into(),
                kind: OptionKind::Number { min: 1, max: 8, step: 1 },
                default: "4".into(),
            },
            OptionDef {
                key: "end_of_line".into(),
                label: "End of Line".into(),
                help: "Line ending file format".into(),
                section: "Defaults".into(),
                kind: OptionKind::Dropdown(vec![
                    ("lf".into(), "LF (Unix/Mac)".into()),
                    ("crlf".into(), "CRLF (Windows)".into()),
                    ("cr".into(), "CR (Classic Mac)".into()),
                ]),
                default: "lf".into(),
            },
            OptionDef {
                key: "charset".into(),
                label: "Charset".into(),
                help: "Character encoding".into(),
                section: "Defaults".into(),
                kind: OptionKind::Dropdown(vec![
                    ("utf-8".into(), "UTF-8".into()),
                    ("utf-8-bom".into(), "UTF-8 with BOM".into()),
                    ("latin1".into(), "Latin-1".into()),
                    ("utf-16be".into(), "UTF-16 BE".into()),
                ]),
                default: "utf-8".into(),
            },
            OptionDef {
                key: "trim_trailing_whitespace".into(),
                label: "Trim Trailing Whitespace".into(),
                help: "Remove trailing whitespace on save".into(),
                section: "Defaults".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "insert_final_newline".into(),
                label: "Insert Final Newline".into(),
                help: "Ensure file ends with a newline".into(),
                section: "Defaults".into(),
                kind: OptionKind::Toggle,
                default: "true".into(),
            },
            OptionDef {
                key: "max_line_length".into(),
                label: "Max Line Length".into(),
                help: "Maximum number of columns per line (0 = off)".into(),
                section: "Defaults".into(),
                kind: OptionKind::Number { min: 0, max: 200, step: 10 },
                default: "0".into(),
            },
            // Makefile overrides
            OptionDef {
                key: "makefile_indent_style".into(),
                label: "Makefile Indent Style".into(),
                help: "Makefiles require tabs for indentation".into(),
                section: "Makefile Overrides".into(),
                kind: OptionKind::Dropdown(vec![
                    ("tab".into(), "Tabs (required)".into()),
                    ("space".into(), "Spaces".into()),
                ]),
                default: "tab".into(),
            },
            // Markdown overrides
            OptionDef {
                key: "md_trim_trailing_whitespace".into(),
                label: "Markdown Trim Whitespace".into(),
                help: "Trailing spaces are meaningful in Markdown (line breaks)".into(),
                section: "Markdown Overrides".into(),
                kind: OptionKind::Toggle,
                default: "false".into(),
            },
        ],
    }
}
