

use crate::storage::{StorageError, Storage};

pub fn initlib(storage: &mut Storage, path: &str) -> Result<(), StorageError> {

    storage.init(path)?;

    println!("initlib: 笔记本已初始化于 {}", path);

    Ok(())

}