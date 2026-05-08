use crate::storage::{StorageError, Storage};

pub fn mknote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.create_note(idx, filename)?;
    println!("mknote: 笔记已创建 -> {}", note_path.display());
    Ok(())
}
