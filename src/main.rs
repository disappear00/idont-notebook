mod command;
mod core;
mod handler;
mod repl;
mod search;
mod storage;

fn main() {
    let mut storage = storage::Storage::new();
    if let Err(e) = repl::run(&mut storage) {
        eprintln!("致命错误: {}", e);
        std::process::exit(1);
    }
}
