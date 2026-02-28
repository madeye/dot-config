use leptos::prelude::*;
use crate::models::config_state::DetectedDotfile;

#[server(DetectDotfiles)]
pub async fn detect_dotfiles() -> Result<Vec<DetectedDotfile>, ServerFnError> {
    use crate::models::schema::DotfileType;

    let home = dirs::home_dir()
        .ok_or_else(|| ServerFnError::new("Cannot find home directory".to_string()))?;

    let checks = vec![
        (DotfileType::Vimrc, "Vimrc", "~/.vimrc", home.join(".vimrc")),
        (DotfileType::Shellrc, "Bash RC", "~/.bashrc", home.join(".bashrc")),
        (DotfileType::Gitconfig, "Git Config", "~/.gitconfig", home.join(".gitconfig")),
        (DotfileType::Tmux, "Tmux", "~/.tmux.conf", home.join(".tmux.conf")),
        (DotfileType::Ssh, "SSH Config", "~/.ssh/config", home.join(".ssh/config")),
        (DotfileType::Editorconfig, "EditorConfig", "~/.editorconfig", home.join(".editorconfig")),
        (DotfileType::Inputrc, "Readline", "~/.inputrc", home.join(".inputrc")),
        (DotfileType::Npmrc, "npm Config", "~/.npmrc", home.join(".npmrc")),
        (DotfileType::Wgetrc, "Wget Config", "~/.wgetrc", home.join(".wgetrc")),
        (DotfileType::Emacs, "Emacs", "~/.emacs", home.join(".emacs")),
    ];

    let mut results = Vec::new();
    for (dtype, name, display_path, full_path) in checks {
        results.push(DetectedDotfile {
            dotfile_type: dtype,
            name: name.into(),
            path: display_path.into(),
            exists: full_path.exists(),
        });
    }

    Ok(results)
}
