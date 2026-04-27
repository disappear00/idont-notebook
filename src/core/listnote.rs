
use crate::storage::{NotebookError, Storage};

pub fn listnote(storage: &mut Storage) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    let notes = storage.list_notes()?;
    if notes.is_empty() {
        println!("listnote: 暂无笔记");
    } else {
        println!("listnote: 共 {} 篇笔记", notes.len());
        for note in &notes {
            println!("  - {}", note);
        }
    }
    Ok(())
}
