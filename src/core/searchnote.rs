
use crate::storage::{StorageError, Storage};

pub fn searchnote(storage: &Storage, keyword: &str) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let notes = storage.list_notes(Some(idx))?;

    if notes.is_empty() {
        println!("searchnote: 当前仓库无笔记");
        return Ok(());
    }

    let mut found = Vec::new();
    for note in &notes {
        let note_path = storage.get_note_path(idx, &note.filename)?;
        if note_path.exists() {
            let content = std::fs::read_to_string(&note_path).unwrap_or_default();
            if content.contains(keyword) {
                found.push(note.filename.clone());
            }
        }
    }

    if found.is_empty() {
        println!("searchnote: 未找到包含 '{}' 的笔记", keyword);
    } else {
        println!("searchnote: 找到 {} 篇匹配的笔记:", found.len());
        for name in &found {
            println!("  - {}", name);
        }
    }
    Ok(())
}
