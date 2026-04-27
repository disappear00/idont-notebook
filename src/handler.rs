use crate::command::Command;
use crate::core::*;
use crate::storage::{NotebookError, Storage};

/// 处理 mknote 命令
pub fn handle_mknote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    mknote::mknote(storage, filename)
}

/// 处理 initnote 命令
pub fn handle_initnote(storage: &mut Storage, path: &str) -> Result<(), NotebookError> {
    initnote::initnote(storage, path)
}

/// 处理 listnote 命令
pub fn handle_listnote(storage: &mut Storage) -> Result<(), NotebookError> {
    listnote::listnote(storage)
}

/// 处理 rmnote 命令
pub fn handle_rmnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    rmnote::rmnote(storage, filename)
}

/// 处理 catnote 命令
pub fn handle_catnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    catnote::catnote(storage, filename)
}

/// 处理 editnote 命令
pub fn handle_editnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    editnote::editnote(storage, filename)
}

/// 处理 renote 命令
pub fn handle_renote(
    storage: &mut Storage,
    old: &str,
    new: &str,
) -> Result<(), NotebookError> {
    renote::renote(storage, old, new)
}

/// 处理 searchnote 命令
pub fn handle_searchnote(storage: &mut Storage, keyword: &str) -> Result<(), NotebookError> {
    searchnote::searchnote(storage, keyword)
}

/// 处理 listlog 命令
pub fn handle_listlog(log: &[String]) -> Result<(),NotebookError>  {
    listlog::listlog(log)
}

/// 处理 help 命令
pub fn handle_help() -> Result<(), NotebookError> {
    help::help()
}

/// 分发命令到对应处理函数
pub fn dispatch(storage: &mut Storage, log: &[String], cmd: Command) -> Result<bool, NotebookError> {
    match cmd {
        Command::Mknote(filename) => handle_mknote(storage, &filename)?,
        Command::Initnote(path) => handle_initnote(storage, &path)?,
        Command::Listnote => handle_listnote(storage)?,
        Command::Rmnote(filename) => handle_rmnote(storage, &filename)?,
        Command::Catnote(filename) => handle_catnote(storage, &filename)?,
        Command::Editnote(filename) => handle_editnote(storage, &filename)?,
        Command::Renote(old, new) => handle_renote(storage, &old, &new)?,
        Command::Searchnote(keyword) => handle_searchnote(storage, &keyword)?,
        Command::Listlog => handle_listlog(log)?,
        Command::Help => handle_help()?,
        Command::Exit => return Ok(true),
    }
    Ok(false)
}
