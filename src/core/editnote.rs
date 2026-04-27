
use crate::storage::{StorageError, Storage};

pub fn editnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    if !storage.is_initialized() {
        return Err(StorageError::NotInitialized);
    }
    // TODO: 用系统编辑器打开笔记
    println!("editnote: 编辑笔记 {}", filename);
    Ok(())
}
