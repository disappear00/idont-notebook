use crate::command::Command;
use crate::storage::{NotebookError, Storage};

/// 处理 mknote 命令
pub fn handle_mknote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 创建笔记文件、更新 notes.toml
    println!("mknote: 创建笔记 {}", filename);
    Ok(())
}

/// 处理 initnote 命令
pub fn handle_initnote(storage: &mut Storage, path: &str) -> Result<(), NotebookError> {
    storage.init(path)?;
    println!("initnote: 笔记本已初始化于 {}", path);
    Ok(())
}

/// 处理 listnote 命令
pub fn handle_listnote(storage: &mut Storage) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    let notes = storage.list_notes()?;
    if notes.is_empty() {
        println!("listnote: 暂无笔记");
    } else {
        println!("listnote: 共 {} 篇笔记", notes.len());
        for note in &notes {
            println!("  - {}", note);
        }
    }
    Ok(())
}

/// 处理 rmnote 命令
pub fn handle_rmnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 删除笔记文件、更新 notes.toml
    println!("rmnote: 删除笔记 {}", filename);
    Ok(())
}

/// 处理 catnote 命令
pub fn handle_catnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 读取并输出笔记内容
    println!("catnote: 查看笔记 {}", filename);
    Ok(())
}

/// 处理 editnote 命令
pub fn handle_editnote(storage: &mut Storage, filename: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 用系统编辑器打开笔记
    println!("editnote: 编辑笔记 {}", filename);
    Ok(())
}

/// 处理 renote 命令
pub fn handle_renote(
    storage: &mut Storage,
    old: &str,
    new: &str,
) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 重命名笔记文件、更新 notes.toml
    println!("renote: 重命名 {} -> {}", old, new);
    Ok(())
}

/// 处理 searchnote 命令
pub fn handle_searchnote(storage: &mut Storage, keyword: &str) -> Result<(), NotebookError> {
    if !storage.is_initialized() {
        return Err(NotebookError::NotInitialized);
    }
    // TODO: 搜索笔记内容
    println!("searchnote: 搜索关键词 \"{}\"", keyword);
    Ok(())
}

/// 处理 help 命令
pub fn handle_help() {
    println!("idontnote — 可用命令:");
    println!();
    println!("  mknote <filename>            创建笔记（支持 .md / .txt）");
    println!("  initnote path <path>         初始化笔记本目录");
    println!("  listnote                     列出所有笔记");
    println!("  rmnote <filename>            删除笔记");
    println!("  catnote <filename>           查看笔记内容");
    println!("  editnote <filename>          用编辑器打开笔记");
    println!("  renote <old> <new>           重命名笔记");
    println!("  searchnote <keyword>         搜索笔记内容");
    println!("  help                         显示此帮助");
    println!("  exit / quit                  退出程序");
}

/// 分发命令到对应处理函数
pub fn dispatch(storage: &mut Storage, cmd: Command) -> Result<bool, NotebookError> {
    match cmd {
        Command::Mknote(filename) => handle_mknote(storage, &filename)?,
        Command::Initnote(path) => handle_initnote(storage, &path)?,
        Command::Listnote => handle_listnote(storage)?,
        Command::Rmnote(filename) => handle_rmnote(storage, &filename)?,
        Command::Catnote(filename) => handle_catnote(storage, &filename)?,
        Command::Editnote(filename) => handle_editnote(storage, &filename)?,
        Command::Renote(old, new) => handle_renote(storage, &old, &new)?,
        Command::Searchnote(keyword) => handle_searchnote(storage, &keyword)?,
        Command::Help => handle_help(),
        Command::Exit => return Ok(true),
    }
    Ok(false)
}
