
use crate::storage::{NotebookError, Storage};

pub fn searchnote(storage: &mut Storage, keyword: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 搜索笔记内容
    println!("searchnote: 搜索关键词 \"{}\"", keyword);
    Ok(())
}
