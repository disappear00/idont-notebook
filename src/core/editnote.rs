use std::process::Command;
use crate::storage::{StorageError, Storage};

pub fn editnote(storage: &Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "start", "", &note_path.to_string_lossy()])
            .status()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(&note_path)
            .status()
    } else {
        Command::new("xdg-open")
            .arg(&note_path)
            .status()
    };

    status.map_err(|e| StorageError::Other(format!("打开文件失败: {}", e)))?;
    Ok(())
}
