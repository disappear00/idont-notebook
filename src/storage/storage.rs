use std::fs;
use std::path::PathBuf;
use chrono::Local;
use serde::{Serialize, Deserialize};
use super::error::StorageError;

/// 笔记本元数据
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookMeta {
    pub notebook: NotebookInfo,
    #[serde(default)]
    pub notes: Vec<NoteMeta>,
}

/// 笔记本基本信息
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookInfo {
    pub version: u32,
    pub created_at: String,
}

/// 单条笔记的元数据
#[derive(Serialize, Deserialize, Debug)]
pub struct NoteMeta {
    pub filename: String,
    pub created_at: String,
}

const META_FILENAME: &str = "notes.toml";
const NOTEBOOK_VERSION: u32 = 1;
const META_DIRECTORY : &str = ".notes";
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
    pub fn init(&mut self, path: &str) -> Result<(), StorageError> {
        let p = PathBuf::from(path);
        let notes_dir = p.join(META_DIRECTORY);

        // 检查内存状态：是否已经初始化
        if self.notebook_path.is_some() {
            return Err(StorageError::Other("笔记本已初始化".to_string()));
        }

        // 检查磁盘状态：.notes/notes.toml 是否已存在
        let meta_path = notes_dir.join(META_FILENAME);
        if meta_path.exists() {
            return Err(StorageError::NoteAlreadyExists(
                format!("目录 {} 已包含笔记本", path),
            ));
        }

        // 创建 .notes 目录（递归）
        fs::create_dir_all(&notes_dir)?;

        // 生成初始元数据
        let meta = NotebookMeta {
            notebook: NotebookInfo {
                version: NOTEBOOK_VERSION,
                created_at: Local::now().to_rfc3339(),
            },
            notes: vec![],
        };

        // 序列化并写入 .notes/notes.toml
        let content = toml::to_string_pretty(&meta)
            .map_err(|e| StorageError::Other(format!("序列化元数据失败: {}", e)))?;
        fs::write(&meta_path, content)?;

        // notebook_path 存储的是 .notes 目录的路径
        self.notebook_path = Some(notes_dir);
        Ok(())
    }

    /// 获取 notes.toml 的完整路径
    pub fn meta_path(&self) -> Option<PathBuf> {
        self.notebook_path.as_ref().map(|p| p.join(META_FILENAME))
    }

    /// 获取笔记本 .notes 内部目录路径（元数据所在位置）
    pub fn notebook_path(&self) -> Option<&PathBuf> {
        self.notebook_path.as_ref()
    }

    /// 获取用户笔记内容根目录（.notes 的父目录）
    pub fn content_path(&self) -> Option<PathBuf> {
        self.notebook_path.as_ref().map(|p| p.parent().map(|parent| parent.to_path_buf())).flatten()
    }

    /// 检查笔记本是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.notebook_path.is_some()
    }

    /// 获取笔记的完整路径（笔记存放在 .notes 的父目录）
    pub fn get_note_path(&self, filename: &str) -> Result<PathBuf, StorageError> {
        let notes_dir = self
            .notebook_path
            .as_ref()
            .ok_or(StorageError::NotInitialized)?;
        // 笔记内容放在 .notes 的上级目录
        let base = notes_dir.parent().ok_or_else(|| StorageError::Other("无效的笔记本路径".to_string()))?;
        Ok(base.join(filename))
    }

    /// 列出所有笔记（TODO: 实现）
    pub fn list_notes(&self) -> Result<Vec<String>, StorageError> {
        if !self.is_initialized() {
            return Err(StorageError::NotInitialized);
        }
        // TODO: 扫描目录或读取 notes.toml
        Ok(vec![])
    }
}
