
use crate::storage::{StorageError, Storage};

pub fn searchnote(storage: &mut Storage, keyword: &str) -> Result<(), StorageError> {
    if !storage.is_initialized() {
        return Err(StorageError::NotInitialized);
    }
    // TODO: 搜索笔记内容
    println!("searchnote: 搜索关键词 \"{}\"", keyword);
    Ok(())
}
