
use crate::storage::{StorageError, Storage};

pub fn initnote(storage: &mut Storage, path: &str) -> Result<(), StorageError> {

    storage.init(path)?;

    println!("initnote: 笔记本已初始化于 {}", path);

    Ok(())

}