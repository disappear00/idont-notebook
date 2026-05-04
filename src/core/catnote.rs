
use crate::storage::{StorageError, Storage};

pub fn catnote(storage: &Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    let content = std::fs::read_to_string(&note_path)?;
    println!("{}", content);
    Ok(())
}
