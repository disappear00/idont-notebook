
use crate::storage::{NotebookError, Storage};

pub fn rmnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 删除笔记文件、更新 notes.toml
    println!("rmnote: 删除笔记 {}", filename);
    Ok(())
}
