use crate::storage::StorageError;

pub fn help() -> Result<(), StorageError> {
    println!("idontnote — 可用命令:");
    println!();
    println!("  仓库管理:");
    println!("    initlib <path>         (il)  初始化笔记本目录");
    println!("    listlib                (ll)  列出所有仓库");
    println!("    selectlib <编号或名称>  (sl)  选中一个仓库");
    println!("    currentlib             (cl)  显示当前选中的仓库");
    println!();
    println!("  笔记操作（需先 selectlib）:");
    println!("    mknote <filename>      (mk)  创建笔记（任意类型）");
    println!("    listnote [-a]          (ls)  列出笔记（-a 显示全部含已隐藏）");
    println!("    editnote <filename>    (ed)  用系统默认程序打开");
    println!("    catnote <filename>     (ca)  打印笔记内容 [-n 行数] [-t 行数]");
    println!("    rmnote <filename>      (rm)  删除笔记");
    println!();
    println!("  跟踪管理:");
    println!("    untrack <filename>           隐藏文件（不删除）");
    println!("    track <filename>             恢复显示被隐藏的文件");
    println!();
    println!("  其他:");
    println!("    listlog                (log) 显示本次会话的命令历史");
    println!("    help                         显示此帮助");
    println!("    exit / quit                  退出程序");
    Ok(())
}
