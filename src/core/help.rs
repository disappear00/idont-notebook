use crate::storage::{StorageError};
pub fn help()-> Result<(), StorageError> {
    println!("idontnote — 可用命令:");
    println!();
    println!("  仓库管理:");
    println!("    initlib <path>         (il)  初始化笔记本目录");
    println!("    listlib                (ll)  列出所有仓库");
    println!("    selectlib <编号或名称>  (sl)  选中一个仓库");
    println!("    currentlib             (cl)  显示当前选中的仓库");
    println!();
    println!("  笔记操作（需先 selectlib）:");
    println!("    mknote <filename>      (mk)  创建笔记（支持 .md / .txt）");
    println!("    listnote               (ls)  列出当前仓库所有笔记");
    println!("    editnote <filename>    (ed)  查看/编辑笔记");
    println!("    catnote <filename>     (ca)  打印笔记内容 [-n 行数] [-t 行数]");
    println!("    rmnote <filename>      (rm)  删除笔记");
    println!();
    println!("  其他:");
    println!("    listlog                (log) 显示本次会话的命令历史");
    println!("    help                         显示此帮助");
    println!("    exit / quit                  退出程序");
    Ok(())
}
