
use crate::storage::{StorageError, Storage};

pub fn rmnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    // 删除文件
    std::fs::remove_file(&note_path)?;

    // 从 notes.toml 中移除该条目
    storage.remove_note_meta(idx, filename)?;

    println!("rmnote: 已删除笔记 {}", filename);
    Ok(())
}
