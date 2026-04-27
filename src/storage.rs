use std::path::PathBuf;
use thiserror::Error;

/// 笔记本错误类型
#[derive(Error, Debug)]
pub enum NotebookError {
    #[error("笔记本未初始化，请先运行 initnote path <path>")]
    NotInitialized,

    #[error("笔记不存在: {0}")]
    NoteNotFound(String),

    #[error("笔记已存在: {0}")]
    NoteAlreadyExists(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

/// 笔记本存储管理
pub struct Storage {
    /// 笔记本根目录，None 表示尚未初始化
    notebook_path: Option<PathBuf>,
}

impl Storage {
    /// 创建新的 Storage 实例（未初始化状态）
    pub fn new() -> Self {
        Self {
            notebook_path: None,
        }
    }

    /// 初始化笔记本目录
    pub fn init(&mut self, path: &str) -> Result<(), NotebookError> {
        let p = PathBuf::from(path);
        // TODO: 创建目录、写入 notes.toml 元数据文件
        self.notebook_path = Some(p);
        Ok(())
    }

    /// 获取笔记本根路径
    pub fn notebook_path(&self) -> Option<&PathBuf> {
        self.notebook_path.as_ref()
    }

    /// 检查笔记本是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.notebook_path.is_some()
    }

    /// 获取笔记的完整路径（TODO: 实现）
    pub fn get_note_path(&self, filename: &str) -> Result<PathBuf, NotebookError> {
        let base = self
            .notebook_path
            .as_ref()
            .ok_or(NotebookError::NotInitialized)?;
        Ok(base.join(filename))
    }

    /// 列出所有笔记（TODO: 实现）
    pub fn list_notes(&self) -> Result<Vec<String>, NotebookError> {
        if !self.is_initialized() {
            return Err(NotebookError::NotInitialized);
        }
        // TODO: 扫描目录或读取 notes.toml
        Ok(vec![])
    }
}
