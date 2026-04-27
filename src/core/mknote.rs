
use crate::storage::{StorageError, Storage};

pub fn mknote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    if !storage.is_initialized() {
        return Err(StorageError::NotInitialized);
    }
    // TODO: 创建笔记文件、更新 notes.toml
    println!("mknote: 创建笔记 {}", filename);
    Ok(())
}
