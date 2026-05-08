use crate::storage::{StorageError, Storage};

pub fn listnote(storage: &Storage, show_all: bool) -> Result<(), StorageError> {
    let idx = storage.require_current_index()?;
    let all_files = storage.scan_notes(idx)?;

    if show_all {
        let hidden = storage.get_hidden_files(idx)?;
        if all_files.is_empty() {
            println!("listnote: .notes/data/ 目录为空");
        } else {
            println!("listnote: .notes/data/ 共 {} 个文件", all_files.len());
            for name in &all_files {
                let status = if hidden.contains(name) { "[已隐藏]" } else { "       " };
                println!("  {} {}", status, name);
            }
        }
    } else {
        let hidden = storage.get_hidden_files(idx)?;
        let visible: Vec<&String> = all_files.iter()
            .filter(|name| !hidden.contains(name))
            .collect();
        if visible.is_empty() {
            println!("listnote: 暂无笔记");
        } else {
            println!("listnote: 共 {} 篇笔记", visible.len());
            for name in &visible {
                println!("  - {}", name);
            }
        }
    }
    Ok(())
}
