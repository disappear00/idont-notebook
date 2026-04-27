
use crate::storage::{NotebookError, Storage};

pub fn catnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 读取并输出笔记内容
    println!("catnote: 查看笔记 {}", filename);
    Ok(())
}
