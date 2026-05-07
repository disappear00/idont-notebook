use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

use crate::command;
use crate::handler;
use crate::storage::Storage;

/// 命令补全器（支持上下文感知）
struct IdontCompleter {
    commands: Vec<&'static str>,
    notebook_names: Vec<String>,
    note_filenames: Vec<String>,
}

impl IdontCompleter {
    fn new() -> Self {
        Self {
            commands: vec![
                "mknote ",
                "mk ",
                "initlib ",
                "il ",
                "listlib",
                "ll",
                "selectlib ",
                "sl ",
                "currentlib",
                "cl",
                "listnote",
                "ls",
                "rmnote ",
                "rm ",
                "editnote ",
                "ed ",
                "listlog",
                "log",
                "help",
                "exit",
                "quit",
            ],
            notebook_names: Vec::new(),
            note_filenames: Vec::new(),
        }
    }

    /// 从 Storage 刷新补全数据
    fn refresh(&mut self, storage: &Storage) {
        self.notebook_names = storage.list_notebooks().iter()
            .map(|e| e.name.clone())
            .collect();
        // 使用当前选中的仓库索引来列出笔记
        let idx = storage.current_notebook_index();
        if let Ok(notes) = storage.list_notes(idx) {
            self.note_filenames = notes.into_iter()
                .map(|m| m.filename)
                .collect();
        }
    }

    /// 判断输入是否匹配指定命令（含别名）
    fn is_command(input: &str, cmd_name: &str, alias: &str) -> bool {
        input == cmd_name || input == alias
    }
}

impl Completer for IdontCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let input = &line[..pos];

        // 解析输入：分离已输入的命令和当前参数
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.len() {
            0 | 1 => {
                // 正在输入命令名 → 补全命令列表
                let matches: Vec<String> = self.commands.iter()
                    .filter(|cmd| cmd.starts_with(input))
                    .map(|s| s.to_string())
                    .collect();
                Ok((0, matches))
            }
            _ => {
                // 已有命令前缀 → 上下文感知参数补全
                let cmd = parts[0];
                let arg_input = parts.last().unwrap().to_string();
                let start_pos = pos - arg_input.len();

                let candidates = if IdontCompleter::is_command(cmd, "selectlib", "sl")
                    || IdontCompleter::is_command(cmd, "initlib", "il")
                {
                    // 补全仓库名
                    self.notebook_names.iter()
                        .filter(|name| name.starts_with(&arg_input))
                        .cloned()
                        .collect()
                } else if IdontCompleter::is_command(cmd, "rmnote", "rm")
                    || IdontCompleter::is_command(cmd, "editnote", "ed")
                {
                    // 补全笔记文件名
                    self.note_filenames.iter()
                        .filter(|name| name.starts_with(&arg_input))
                        .cloned()
                        .collect()
                } else {
                    Vec::new()
                };

                Ok((start_pos, candidates))
            }
        }
    }
}

impl Hinter for IdontCompleter {
    type Hint = String;
}

impl Highlighter for IdontCompleter {}
impl Validator for IdontCompleter {}
impl Helper for IdontCompleter {}

/// 构建 REPL 提示符（显示当前仓库名）
fn build_prompt(storage: &Storage) -> String {
    match storage.current_notebook_index() {
        Some(idx) => {
            if let Ok(entry) = storage.get_notebook(idx) {
                format!("idont({})> ", entry.name)
            } else {
                "idontnote> ".to_string()
            }
        }
        None => "idontnote> ".to_string(),
    }
}

/// 启动 REPL 主循环
pub fn run(storage: &mut Storage) -> Result<(), Box<dyn std::error::Error>> {
    let completer = IdontCompleter::new();
    let mut rl = Editor::new()?;
    rl.set_helper(Some(completer));

    println!("idontnote v0.1.0 — 输入 help 查看帮助，exit 退出");

    let mut log: Vec<String> = Vec::new();

    loop {
        // 刷新补全数据（同步仓库/笔记列表）
        if let Some(helper) = rl.helper_mut() {
            helper.refresh(storage);
        }

        let prompt = build_prompt(storage);
        let line = rl.readline(&prompt);
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed)?;
                log.push(trimmed.to_string());

                match command::parse(trimmed) {
                    Ok(cmd) => match handler::dispatch(storage, &log, cmd) {
                        Ok(should_exit) => {
                            if should_exit {
                                println!("再见！");
                                break;
                            }
                        }
                        Err(e) => eprintln!("错误: {}", e),
                    },
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("（按 Ctrl+D 或输入 exit 退出）");
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("再见！");
                break;
            }
            Err(e) => {
                eprintln!("读取输入错误: {}", e);
                break;
            }
        }
    }

    Ok(())
}
