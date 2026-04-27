use crate::storage::{StorageError};
pub fn help()-> Result<(), StorageError> {
    println!("idontnote — 可用命令:");
    println!();
    println!("  mknote <filename>            创建笔记（支持 .md / .txt）");
    println!("  initnote <path>               初始化笔记本目录");
    println!("  listnote                     列出所有笔记");
    println!("  rmnote <filename>            删除笔记");
    println!("  catnote <filename>           查看笔记内容");
    println!("  editnote <filename>          用编辑器打开笔记");
    println!("  renote <old> <new>           重命名笔记");
    println!("  searchnote <keyword>         搜索笔记内容");
    println!("  listlog                      显示本次会话的命令历史");
    println!("  help                         显示此帮助");
    println!("  exit / quit                  退出程序");
    Ok(())
}
