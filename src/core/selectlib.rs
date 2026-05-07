
use crate::storage::{StorageError, Storage};

/// 选中一个仓库（按索引或名称）
pub fn selectlib(storage: &mut Storage, selector: &str) -> Result<(), StorageError> {
    let idx = storage.select_notebook(selector)?;
    let entry = storage.get_notebook(idx)?;
    println!("selectlib: 已选中仓库 '{}' (#{})，路径: {}", entry.name, idx, entry.path.display());
    Ok(())
}
