
use crate::storage::{StorageError, Storage};

/// 显示当前选中的仓库
pub fn currentlib(storage: &Storage) -> Result<(), StorageError> {
    let entry = storage.require_current_notebook()?;
    let idx = storage.current_notebook_index().unwrap();
    println!("currentlib: 当前选中仓库 '{}' (#{})，路径: {}", entry.name, idx, entry.path.display());
    Ok(())
}
