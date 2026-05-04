
use crate::storage::{StorageError, Storage};

/// 列出所有已注册的仓库
pub fn listlib(storage: &Storage) -> Result<(), StorageError> {
    let notebooks = storage.list_notebooks();
    if notebooks.is_empty() {
        println!("listlib: 暂无已注册的仓库，请先运行 initlib <path>");
        return Ok(());
    }

    let current_idx = storage.current_notebook_index();
    println!("listlib: 共 {} 个仓库", notebooks.len());
    for (i, nb) in notebooks.iter().enumerate() {
        let marker = if current_idx == Some(i) { " *" } else { "" };
        println!("  [{}] {} ({}){}", i, nb.name, nb.path.display(), marker);
    }
    if current_idx.is_some() {
        println!("  * = 当前选中");
    }
    Ok(())
}
