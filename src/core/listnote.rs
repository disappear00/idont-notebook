
use crate::storage::{StorageError, Storage};

pub fn listnote(storage: &Storage) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let notes = storage.list_notes(Some(idx))?;
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
