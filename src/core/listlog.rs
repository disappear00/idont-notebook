
use crate::storage::{NotebookError};
pub fn listlog(log: &[String]) -> Result<(), NotebookError> {
    if log.is_empty() {
        println!("listlog: 当前会话暂无命令记录");
    } else {
        println!("listlog: 本次会话共 {} 条命令", log.len());
        for (i, entry) in log.iter().enumerate() {
            println!("  {:>3}. {}", i + 1, entry);
        }
    }
    Ok(())
}
