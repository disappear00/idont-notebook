pub enum Command {
    Mknote(String),
    Initlib(String),
    Listlib,
    Selectlib(String),
    Currentlib,
    Listnote { show_all: bool },
    Rmnote(String),
    Editnote(String),
    Catnote { filename: String, head: Option<usize>, tail: Option<usize> },
    Track(String),
    Untrack(String),
    Listlog,
    Chat(String),
    Help,
    Exit,
}

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
            if name.is_empty() {
                return Err("文件名不能为空".into());
            }
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
        "listnote" | "ls" => {
            let show_all = parts.get(1).map(|s| *s == "-a").unwrap_or(false);
            Ok(Command::Listnote { show_all })
        }
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
        "track" => {
            let name = parts.get(1).ok_or("用法: track <filename>")?;
            let name = name.trim();
            if name.is_empty() {
                return Err("文件名不能为空".into());
            }
            Ok(Command::Track(name.to_string()))
        }
        "untrack" => {
            let name = parts.get(1).ok_or("用法: untrack <filename>")?;
            let name = name.trim();
            if name.is_empty() {
                return Err("文件名不能为空".into());
            }
            Ok(Command::Untrack(name.to_string()))
        }
        "listlog" | "log" => Ok(Command::Listlog),
        "chat" => {
            let arg = parts.get(1).ok_or("用法: chat \"<搜索关键词>\"")?;
            let keyword = arg.trim_matches('"');
            if keyword.is_empty() {
                return Err("搜索关键词不能为空".into());
            }
            Ok(Command::Chat(keyword.to_string()))
        }
        "help" => Ok(Command::Help),
        "exit" | "quit" => Ok(Command::Exit),
        _ => Err(format!("未知命令: {}，输入 help 查看帮助", cmd)),
    }
}
