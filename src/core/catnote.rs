
use crate::storage::{StorageError, Storage};

pub fn catnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    if !storage.is_initialized() {
        return Err(StorageError::NotInitialized);
    }
    // TODO: 读取并输出笔记内容
    println!("catnote: 查看笔记 {}", filename);
    Ok(())
}
