use crate::command::Command;
use crate::core::*;
use crate::storage::{StorageError, Storage};

/// 处理 mknote 命令
pub fn handle_mknote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    mknote::mknote(storage, filename)
}

/// 处理 initlib 命令
pub fn handle_initlib(storage: &mut Storage, path: &str) -> Result<(), StorageError> {
    initlib::initlib(storage, path, None)
}

/// 处理 listlib 命令
pub fn handle_listlib(storage: &Storage) -> Result<(), StorageError> {
    listlib::listlib(storage)
}

/// 处理 selectlib 命令
pub fn handle_selectlib(storage: &mut Storage, selector: &str) -> Result<(), StorageError> {
    selectlib::selectlib(storage, selector)
}

/// 处理 currentlib 命令
pub fn handle_currentlib(storage: &Storage) -> Result<(), StorageError> {
    currentlib::currentlib(storage)
}

/// 处理 listnote 命令
pub fn handle_listnote(storage: &mut Storage) -> Result<(), StorageError> {
    listnote::listnote(storage)
}

/// 处理 rmnote 命令
pub fn handle_rmnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    rmnote::rmnote(storage, filename)
}

/// 处理 catnote 命令
pub fn handle_catnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    catnote::catnote(storage, filename)
}

/// 处理 editnote 命令
pub fn handle_editnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    editnote::editnote(storage, filename)
}

/// 处理 renote 命令
pub fn handle_renote(
    storage: &mut Storage,
    old: &str,
    new: &str,
) -> Result<(), StorageError> {
    renote::renote(storage, old, new)
}

/// 处理 searchnote 命令
pub fn handle_searchnote(storage: &mut Storage, keyword: &str) -> Result<(), StorageError> {
    searchnote::searchnote(storage, keyword)
}

/// 处理 listlog 命令
pub fn handle_listlog(log: &[String]) -> Result<(),StorageError>  {
    listlog::listlog(log)
}

/// 处理 help 命令
pub fn handle_help() -> Result<(), StorageError> {
    help::help()
}

/// 分发命令到对应处理函数
pub fn dispatch(storage: &mut Storage, log: &[String], cmd: Command) -> Result<bool, StorageError> {
    match cmd {
        Command::Mknote(filename) => handle_mknote(storage, &filename)?,
        Command::Initlib(path) => handle_initlib(storage, &path)?,
        Command::Listlib => handle_listlib(storage)?,
        Command::Selectlib(selector) => handle_selectlib(storage, &selector)?,
        Command::Currentlib => handle_currentlib(storage)?,
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
