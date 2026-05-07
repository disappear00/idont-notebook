use std::fs;

use crate::storage::{StorageError, Storage};

pub fn catnote(storage: &Storage, filename: &str, head: Option<usize>, tail: Option<usize>) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    let content = fs::read_to_string(&note_path)?;

    match (head, tail) {
        (Some(n), _) => {
            let lines: Vec<&str> = content.lines().take(n).collect();
            for line in &lines {
                println!("{}", line);
            }
        }
        (_, Some(n)) => {
            let lines: Vec<&str> = content.lines().collect();
            let start = lines.len().saturating_sub(n);
            for line in &lines[start..] {
                println!("{}", line);
            }
        }
        (None, None) => {
            print!("{}", content);
        }
    }

    Ok(())
}
