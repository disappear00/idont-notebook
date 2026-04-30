
use crate::storage::{StorageError, Storage};

pub fn listnote(storage: &mut Storage) -> Result<(), StorageError> {
    if !storage.is_initialized() {
        return Err(StorageError::NotInitialized);
    }
    let notes = storage.list_notes(None)?;
    if notes.is_empty() {
        println!("listnote: 暂无笔记");
    } else {
        println!("listnote: 共 {} 篇笔记", notes.len());
        for note in &notes {
            println!("  - {} (创建于 {})", note.filename, note.created_at);
        }
    }
    Ok(())
}
