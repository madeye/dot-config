use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadResult {
    pub content: String,
    pub exists: bool,
}

#[server(ReadDotfile)]
pub async fn read_dotfile(path: String) -> Result<ReadResult, ServerFnError> {
    let expanded = expand_path(&path);
    if expanded.exists() {
        let content = tokio::fs::read_to_string(&expanded).await
            .map_err(|e| ServerFnError::new(format!("Failed to read {}: {}", path, e)))?;
        Ok(ReadResult { content, exists: true })
    } else {
        Ok(ReadResult { content: String::new(), exists: false })
    }
}

#[server(WriteDotfile)]
pub async fn write_dotfile(path: String, content: String) -> Result<String, ServerFnError> {
    let expanded = expand_path(&path);

    // Create backup if file exists
    if expanded.exists() {
        create_backup(&expanded).await?;
    }

    // Ensure parent directory exists
    if let Some(parent) = expanded.parent() {
        tokio::fs::create_dir_all(parent).await
            .map_err(|e| ServerFnError::new(format!("Failed to create directory: {}", e)))?;
    }

    tokio::fs::write(&expanded, &content).await
        .map_err(|e| ServerFnError::new(format!("Failed to write {}: {}", path, e)))?;

    Ok(format!("Saved to {}", path))
}

#[cfg(feature = "ssr")]
async fn create_backup(path: &std::path::Path) -> Result<(), ServerFnError> {
    let backup_dir = dirs::home_dir()
        .ok_or_else(|| ServerFnError::new("Cannot find home directory".to_string()))?
        .join(".dot-config-backups");

    tokio::fs::create_dir_all(&backup_dir).await
        .map_err(|e| ServerFnError::new(format!("Failed to create backup dir: {}", e)))?;

    let filename = path.file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("{}.{}", filename, timestamp);
    let backup_path = backup_dir.join(backup_name);

    tokio::fs::copy(path, &backup_path).await
        .map_err(|e| ServerFnError::new(format!("Failed to create backup: {}", e)))?;

    Ok(())
}

#[cfg(feature = "ssr")]
fn expand_path(path: &str) -> std::path::PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    std::path::PathBuf::from(path)
}
