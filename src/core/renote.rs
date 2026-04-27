
use crate::storage::{StorageError, Storage};

pub fn renote(
    storage: &mut Storage,
    old: &str,
    new: &str,
) -> Result<(), StorageError> {
    if !storage.is_initialized() {
        return Err(StorageError::NotInitialized);
    }
    // TODO: 重命名笔记文件、更新 notes.toml
    println!("renote: 重命名 {} -> {}", old, new);
    Ok(())
}
