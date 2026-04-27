
use crate::storage::{NotebookError, Storage};

pub fn editnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 用系统编辑器打开笔记
    println!("editnote: 编辑笔记 {}", filename);
    Ok(())
}
