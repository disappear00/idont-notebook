use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

use crate::command;
use crate::handler;
use crate::storage::Storage;

/// 命令补全器
struct IdontCompleter {
    commands: Vec<&'static str>,
}

impl IdontCompleter {
    fn new() -> Self {
        Self {
            commands: vec![
                "mknote ",
                "initnote ",
                "listnote",
                "rmnote ",
                "catnote ",
                "editnote ",
                "renote ",
                "searchnote ",
                "listlog",
                "help",
                "exit",
                "quit",
            ],
        }
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
        let matches: Vec<String> = self
            .commands
            .iter()
            .filter(|cmd| cmd.starts_with(input))
            .map(|s| s.to_string())
            .collect();
        Ok((0, matches))
    }
}

impl Hinter for IdontCompleter {
    type Hint = String;
}

impl Highlighter for IdontCompleter {}
impl Validator for IdontCompleter {}
impl Helper for IdontCompleter {}

/// 启动 REPL 主循环
pub fn run(storage: &mut Storage) -> Result<(), Box<dyn std::error::Error>> {
    let completer = IdontCompleter::new();
    let mut rl = Editor::new()?;
    rl.set_helper(Some(completer));

    println!("idontnote v0.1.0 — 输入 help 查看帮助，exit 退出");

    let mut log: Vec<String> = Vec::new();

    loop {
        let line = rl.readline("idontnote> ");
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
