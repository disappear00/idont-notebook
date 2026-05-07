
use crate::storage::{StorageError, Storage};

/// 初始化/注册一个新仓库
pub fn initlib(storage: &mut Storage, path: &str, name: Option<&str>) -> Result<(), StorageError> {
    let idx = storage.init_notebook(path, name)?;
    let entry = storage.get_notebook(idx)?;
    println!("initlib: 仓库 '{}' 已注册 (#{})，路径: {}", entry.name, idx, path);
    Ok(())
}
