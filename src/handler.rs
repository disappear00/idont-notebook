use crate::command::Command;
use crate::core::*;
use crate::storage::{StorageError, Storage};

pub fn handle_mknote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    mknote::mknote(storage, filename)
}

pub fn handle_initlib(storage: &mut Storage, path: &str) -> Result<(), StorageError> {
    initlib::initlib(storage, path, None)
}

pub fn handle_listlib(storage: &Storage) -> Result<(), StorageError> {
    listlib::listlib(storage)
}

pub fn handle_selectlib(storage: &mut Storage, selector: &str) -> Result<(), StorageError> {
    selectlib::selectlib(storage, selector)
}

pub fn handle_currentlib(storage: &Storage) -> Result<(), StorageError> {
    currentlib::currentlib(storage)
}

pub fn handle_listnote(storage: &Storage, show_all: bool) -> Result<(), StorageError> {
    listnote::listnote(storage, show_all)
}

pub fn handle_rmnote(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    rmnote::rmnote(storage, filename)
}

pub fn handle_editnote(storage: &Storage, filename: &str) -> Result<(), StorageError> {
    editnote::editnote(storage, filename)
}

pub fn handle_catnote(storage: &Storage, filename: &str, head: Option<usize>, tail: Option<usize>) -> Result<(), StorageError> {
    catnote::catnote(storage, filename, head, tail)
}

pub fn handle_track(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    track::track(storage, filename)
}

pub fn handle_untrack(storage: &mut Storage, filename: &str) -> Result<(), StorageError> {
    untrack::untrack(storage, filename)
}

pub fn handle_listlog(log: &[String]) -> Result<(), StorageError> {
    listlog::listlog(log)
}

pub fn handle_help() -> Result<(), StorageError> {
    help::help()
}

pub fn handle_chat(storage: &mut Storage, keyword: &str) -> Result<(), StorageError> {
    chat::chat(storage, keyword)
}

pub fn dispatch(storage: &mut Storage, log: &[String], cmd: Command) -> Result<bool, StorageError> {
    match cmd {
        Command::Mknote(filename) => handle_mknote(storage, &filename)?,
        Command::Initlib(path) => handle_initlib(storage, &path)?,
        Command::Listlib => handle_listlib(storage)?,
        Command::Selectlib(selector) => handle_selectlib(storage, &selector)?,
        Command::Currentlib => handle_currentlib(storage)?,
        Command::Listnote { show_all } => handle_listnote(storage, show_all)?,
        Command::Rmnote(filename) => handle_rmnote(storage, &filename)?,
        Command::Editnote(filename) => handle_editnote(storage, &filename)?,
        Command::Catnote { filename, head, tail } => handle_catnote(storage, &filename, head, tail)?,
        Command::Track(filename) => handle_track(storage, &filename)?,
        Command::Untrack(filename) => handle_untrack(storage, &filename)?,
        Command::Listlog => handle_listlog(log)?,
        Command::Chat(keyword) => handle_chat(storage, &keyword)?,
        Command::Help => handle_help()?,
        Command::Exit => return Ok(true),
    }
    Ok(false)
}
