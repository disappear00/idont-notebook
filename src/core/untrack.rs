use crate::storage::{StorageError, Storage};

pub fn untrack(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    storage.hide_file(idx, filename)?;
    println!("untrack: 已隐藏 {}（文件仍保留在磁盘上）", filename);
    Ok(())
}