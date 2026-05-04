
use crate::storage::{StorageError, Storage};

pub fn renote(
    storage: &mut Storage,
    old: &str,
    new: &str,
) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let old_path = storage.get_note_path(idx, old)?;

    if !old_path.exists() {
        return Err(StorageError::NoteNotFound(old.to_string()));
    }

    // 新文件名也确保有 .md 后缀
    let new_normalized = if new.ends_with(".md") {
        new.to_string()
    } else {
        format!("{}.md", new)
    };

    let new_path = storage.get_note_path(idx, &new_normalized)?;

    if new_path.exists() {
        return Err(StorageError::NoteAlreadyExists(new_normalized));
    }

    // 重命名文件
    std::fs::rename(&old_path, &new_path)?;

    // 更新 notes.toml
    storage.update_note_filename(idx, old, &new_normalized)?;

    println!("renote: {} -> {}", old, new_normalized);
    Ok(())
}
