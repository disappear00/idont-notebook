
use std::io::{self, BufRead, Write};
use crate::storage::{StorageError, Storage};

/// 创建笔记：列出仓库 → 用户选择 → 创建文件 → 更新 notes.toml
pub fn mknote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    // 1. 检查是否有已注册的仓库
    if storage.notebook_count() == 0 {
        return Err(StorageError::NoNotebooksRegistered);
    }

    // 2. 列出所有仓库
    let notebooks = storage.list_notebooks();
    println!("请选择要创建笔记的仓库:");
    for (i, nb) in notebooks.iter().enumerate() {
        println!("  [{}] {} ({})", i, nb.name, nb.path.display());
    }

    // 3. 提示用户选择
    let selection = prompt_selection(notebooks.len())?;

    // 4. 在选中的仓库中创建笔记
    let note_path = storage.create_note(selection, filename)?;
    println!("mknote: 笔记已创建 -> {}", note_path.display());

    Ok(())
}

/// 从 stdin 读取用户输入的索引编号
fn prompt_selection(max: usize) -> Result<usize, StorageError> {
    print!("> 请输入编号: ");
    io::stdout().flush().map_err(|e| StorageError::Other(format!("刷新输出失败: {}", e)))?;

    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut input)
        .map_err(|e| StorageError::Other(format!("读取输入失败: {}", e)))?;

    let trimmed = input.trim();
    let index: usize = trimmed.parse()
        .map_err(|_| StorageError::InvalidSelection(
            format!("请输入 0-{} 之间的数字", max - 1)
        ))?;

    if index >= max {
        return Err(StorageError::InvalidSelection(
            format!("索引 {} 超出范围 (0-{})", index, max - 1)
        ));
    }

    Ok(index)
}
