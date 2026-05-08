use std::io::{self, Write};
use crate::storage::{StorageError, Storage};

pub fn rmnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let note_path = storage.get_note_path(idx, filename)?;

    if !note_path.exists() {
        return Err(StorageError::NoteNotFound(filename.to_string()));
    }

    print!("rmnote: 确定删除笔记 {}？[y/N] ", filename);
    io::stdout().flush().map_err(|e| StorageError::Other(format!("刷新输出失败: {}", e)))?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| StorageError::Other(format!("读取输入失败: {}", e)))?;

    if input.trim().to_lowercase() != "y" {
        println!("rmnote: 已取消删除");
        return Ok(());
    }

    std::fs::remove_file(&note_path)?;
    storage.remove_from_hidden(idx, filename)?;

    println!("rmnote: 已删除笔记 {}", filename);
    Ok(())
}
