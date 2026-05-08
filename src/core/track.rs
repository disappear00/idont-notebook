use crate::storage::{StorageError, Storage};

pub fn track(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    storage.unhide_file(idx, filename)?;
    println!("track: 已恢复显示 {}", filename);
    Ok(())
}