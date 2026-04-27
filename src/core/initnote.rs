
use crate::storage::{NotebookError, Storage};

pub fn initnote(storage: &mut Storage, path: &str) -> Result<(), NotebookError> {

    storage.init(path)?;

    println!("initnote: 笔记本已初始化于 {}", path);

    Ok(())

}