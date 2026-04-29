/// REPL 命令枚举
pub enum Command {
    /// mknote <filename> — 创建笔记（支持 .md / .txt）
    Mknote(String),
    /// initlib <path> — 初始化笔记本目录
    Initlib(String),
    /// listnote — 列出所有笔记
    Listnote,
    /// rmnote <filename> — 删除笔记
    Rmnote(String),
    /// catnote <filename> — 查看笔记内容
    Catnote(String),
    /// editnote <filename> — 用系统编辑器打开笔记
    Editnote(String),
    /// renote <old> <new> — 重命名笔记
    Renote(String, String),
    /// searchnote <keyword> — 搜索笔记内容
    Searchnote(String),
    /// listlog — 显示本次会话的命令历史
    Listlog,
    /// help — 显示帮助
    Help,
    /// exit / quit — 退出 REPL
    Exit,
}

/// 将用户输入解析为 Command
pub fn parse(input: &str) -> Result<Command, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("请输入命令，输入 help 查看帮助".into());
    }

    let parts: Vec<&str> = input.splitn(4, ' ').collect();
    let cmd = parts[0];

    match cmd {
        "mknote" => {
            let name = parts.get(1).ok_or("用法: mknote <filename>")?;
            let name = name.trim();
            validate_note_filename(name)?;
            Ok(Command::Mknote(name.to_string()))
        }
        "initlib" => {
            let path = parts.get(1).ok_or("用法: initlib <path>")?;
            let path = path.trim();
            if path.is_empty() {
                return Err("路径不能为空".into());
            }
            Ok(Command::Initlib(path.to_string()))
        }
        "listnote" => Ok(Command::Listnote),
        "rmnote" => {
            let name = parts.get(1).ok_or("用法: rmnote <filename>")?;
            Ok(Command::Rmnote(name.trim().to_string()))
        }
        "catnote" => {
            let name = parts.get(1).ok_or("用法: catnote <filename>")?;
            Ok(Command::Catnote(name.trim().to_string()))
        }
        "editnote" => {
            let name = parts.get(1).ok_or("用法: editnote <filename>")?;
            Ok(Command::Editnote(name.trim().to_string()))
        }
        "renote" => {
            let old = parts.get(1).ok_or("用法: renote <old> <new>")?;
            let new = parts.get(2).ok_or("用法: renote <old> <new>")?;
            Ok(Command::Renote(old.trim().to_string(), new.trim().to_string()))
        }
        "searchnote" => {
            let keyword = parts.get(1).ok_or("用法: searchnote <keyword>")?;
            Ok(Command::Searchnote(keyword.trim().to_string()))
        }
        "listlog" => Ok(Command::Listlog),
        "help" => Ok(Command::Help),
        "exit" | "quit" => Ok(Command::Exit),
        _ => Err(format!("未知命令: {}，输入 help 查看帮助", cmd)),
    }
}

/// 校验笔记文件名，仅允许 .md 和 .txt 后缀
fn validate_note_filename(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("文件名不能为空".into());
    }
    if !name.ends_with(".md") && !name.ends_with(".txt") {
        return Err("仅支持 .md 和 .txt 格式的笔记".into());
    }
    Ok(())
}
