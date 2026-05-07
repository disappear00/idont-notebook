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
    /// catnote <filename> [-n <lines>] [-t <lines>] — 打印笔记内容
    Catnote { filename: String, head: Option<usize>, tail: Option<usize> },
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
        "mknote" | "mk" => {
            let name = parts.get(1).ok_or("用法: mknote <filename> (别名: mk)")?;
            let name = name.trim();
            validate_note_filename(name)?;
            Ok(Command::Mknote(name.to_string()))
        }
        "initlib" | "il" => {
            let path = parts.get(1).ok_or("用法: initlib <path> (别名: il)")?;
            let path = path.trim();
            if path.is_empty() {
                return Err("路径不能为空".into());
            }
            Ok(Command::Initlib(path.to_string()))
        }
        "listlib" | "ll" => Ok(Command::Listlib),
        "selectlib" | "sl" => {
            let selector = parts.get(1).ok_or("用法: selectlib <编号或名称> (别名: sl)")?;
            let selector = selector.trim();
            if selector.is_empty() {
                return Err("请指定仓库编号或名称".into());
            }
            Ok(Command::Selectlib(selector.to_string()))
        }
        "currentlib" | "cl" => Ok(Command::Currentlib),
        "listnote" | "ls" => Ok(Command::Listnote),
        "rmnote" | "rm" => {
            let name = parts.get(1).ok_or("用法: rmnote <filename> (别名: rm)")?;
            Ok(Command::Rmnote(name.trim().to_string()))
        }
        "editnote" | "ed" => {
            let name = parts.get(1).ok_or("用法: editnote <filename> (别名: ed)")?;
            Ok(Command::Editnote(name.trim().to_string()))
        }
        "catnote" | "ca" => {
            let all_parts: Vec<&str> = input.split_whitespace().collect();
            if all_parts.len() < 2 {
                return Err("用法: catnote <filename> [-n <lines>] [-t <lines>] (别名: ca)".into());
            }
            let name = all_parts[1];
            validate_note_filename(name)?;
            let mut head: Option<usize> = None;
            let mut tail: Option<usize> = None;
            let mut i = 2;
            while i < all_parts.len() {
                match all_parts[i] {
                    "-n" => {
                        i += 1;
                        let val = all_parts.get(i).ok_or("用法: -n <行数>")?;
                        head = Some(val.parse::<usize>()
                            .map_err(|_| format!("无效的行数: {}", val))?);
                    }
                    "-t" => {
                        i += 1;
                        let val = all_parts.get(i).ok_or("用法: -t <行数>")?;
                        tail = Some(val.parse::<usize>()
                            .map_err(|_| format!("无效的行数: {}", val))?);
                    }
                    other => return Err(format!("未知选项: {}", other)),
                }
                i += 1;
            }
            if head.is_some() && tail.is_some() {
                return Err("-n 和 -t 不能同时使用".into());
            }
            Ok(Command::Catnote { filename: name.to_string(), head, tail })
        }
        "listlog" | "log" => Ok(Command::Listlog),
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
