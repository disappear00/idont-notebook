/// REPL 命令枚举
pub enum Command {
    /// mknote <filename> — 创建笔记（支持 .md / .txt）
    Mknote(String),
    /// initlib <path> — 初始化笔记本目录
    Initlib(String),
    /// listlib — 列出所有已注册的仓库
    Listlib,
    /// selectlib <索引或名称> — 选中一个仓库
    Selectlib(String),
    /// currentlib — 显示当前选中的仓库
    Currentlib,
    /// listnote — 列出所有笔记
    Listnote,
    /// rmnote <filename> — 删除笔记
    Rmnote(String),
    /// editnote <filename> — 查看/编辑笔记（用系统编辑器打开）
    Editnote(String),
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
        "listlib" => Ok(Command::Listlib),
        "selectlib" => {
            let selector = parts.get(1).ok_or("用法: selectlib <编号或名称>")?;
            let selector = selector.trim();
            if selector.is_empty() {
                return Err("请指定仓库编号或名称".into());
            }
            Ok(Command::Selectlib(selector.to_string()))
        }
        "currentlib" => Ok(Command::Currentlib),
        "listnote" => Ok(Command::Listnote),
        "rmnote" => {
            let name = parts.get(1).ok_or("用法: rmnote <filename>")?;
            Ok(Command::Rmnote(name.trim().to_string()))
        }
        "editnote" => {
            let name = parts.get(1).ok_or("用法: editnote <filename>")?;
            Ok(Command::Editnote(name.trim().to_string()))
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
