use std::process::Command;
use crate::storage::{StorageError, Storage};

pub fn editnote(storage: &Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    // 尝试使用系统默认编辑器打开
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            // Windows 默认用 notepad，其他平台用 vi
            if cfg!(windows) { "notepad".to_string() } else { "vi".to_string() }
        });

    let status = Command::new(&editor)
        .arg(&note_path)
        .status()
        .map_err(|e| StorageError::Other(format!("启动编辑器失败: {}", e)))?;

    if !status.success() {
        return Err(StorageError::Other("编辑器退出异常".to_string()));
    }
    Ok(())
}
