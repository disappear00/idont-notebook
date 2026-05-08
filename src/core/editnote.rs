use std::process::Command;
use std::path::Path;
use crate::storage::{StorageError, Storage};

fn is_wsl() -> bool {
    if std::env::var("WSL_DISTRO_NAME").is_ok() {
        return true;
    }
    std::fs::read_to_string("/proc/version")
        .map(|v| v.to_lowercase().contains("microsoft") || v.to_lowercase().contains("wsl"))
        .unwrap_or(false)
}

fn open_file(path: &Path) -> Result<std::process::ExitStatus, StorageError> {
    let path_str = path.to_string_lossy();

    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .args(["/c", "start", "", &path_str])
            .status()
            .map_err(|e| StorageError::Other(format!("打开文件失败: {}", e)));
    }

    if cfg!(target_os = "macos") {
        return Command::new("open")
            .arg(path)
            .status()
            .map_err(|e| StorageError::Other(format!("打开文件失败: {}", e)));
    }

    // Linux / WSL
    if is_wsl() {
        // 尝试 wslview（wslu 包）
        if let Ok(status) = Command::new("wslview").arg(path).status() {
            return Ok(status);
        }
        // 尝试 explorer.exe（Windows 资源管理器）
        if let Ok(status) = Command::new("explorer.exe").arg(&*path_str).status() {
            return Ok(status);
        }
    }

    // 普通 Linux
    Command::new("xdg-open")
        .arg(path)
        .status()
        .map_err(|e| StorageError::Other(format!("打开文件失败: 未找到 xdg-open/wslview/explorer.exe")))
}

pub fn editnote(storage: &Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    open_file(&note_path)?;
    Ok(())
}
