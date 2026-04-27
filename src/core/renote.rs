
use crate::storage::{NotebookError, Storage};

pub fn renote(
    storage: &mut Storage,
    old: &str,
    new: &str,
) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 重命名笔记文件、更新 notes.toml
    println!("renote: 重命名 {} -> {}", old, new);
    Ok(())
}
