
use crate::storage::{StorageError, Storage};

pub fn rmnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    if !storage.is_initialized() {
        return Err(StorageError::NotInitialized);
    }
    // TODO: 删除笔记文件、更新 notes.toml
    println!("rmnote: 删除笔记 {}", filename);
    Ok(())
}
