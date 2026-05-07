use thiserror::Error;


/// 笔记本错误类型
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("笔记本未初始化，请先运行 initlib <path>")]
    NotInitialized,

    #[error("没有已注册的仓库，请先运行 initlib <path>")]
    NoNotebooksRegistered,

    #[error("未选中任何仓库，请先运行 selectlib <编号或名称>")]
    NoNotebookSelected,

    #[error("无效的选择索引: {0}")]
    InvalidSelection(String),

    #[error("笔记不存在: {0}")]
    NoteNotFound(String),

    #[error("笔记已存在: {0}")]
    NoteAlreadyExists(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}
