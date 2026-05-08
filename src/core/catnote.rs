use std::fs;

use crate::storage::{StorageError, Storage};

pub fn catnote(storage: &Storage, filename: &str, head: Option<usize>, tail: Option<usize>) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    let content = fs::read(&note_path)?;
    let text = match String::from_utf8(content) {
        Ok(s) => s,
        Err(_) => {
            println!("二进制文件无法预览，请用 editnote 打开");
            return Ok(());
        }
    };

    match (head, tail) {
        (Some(n), _) => {
            for line in text.lines().take(n) {
                println!("{}", line);
            }
        }
        (_, Some(n)) => {
            let lines: Vec<&str> = text.lines().collect();
            let start = lines.len().saturating_sub(n);
            for line in &lines[start..] {
                println!("{}", line);
            }
        }
        (None, None) => {
            print!("{}", text);
        }
    }

    Ok(())
}
