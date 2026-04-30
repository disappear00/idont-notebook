use std::fs;
use std::path::PathBuf;
use chrono::Local;
use serde::{Serialize, Deserialize};
use super::error::StorageError;

/// 全局配置文件名（存储在用户目录下）
const GLOBAL_CONFIG_FILENAME: &str = "idont-notebook-config.toml";
const META_FILENAME: &str = "notes.toml";
const NOTEBOOK_VERSION: u32 = 1;
const META_DIRECTORY: &str = ".notes";

// ============================================================
// 数据结构
// ============================================================

/// 笔记本元数据（对应 notes.toml 内容）
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteMeta {
    pub filename: String,
    pub created_at: String,
}

/// 单个仓库条目（内存中的注册信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookEntry {
    /// 仓库名称
    pub name: String,
    /// .notes 目录的完整路径
    pub path: PathBuf,
}

/// 全局配置（持久化到用户目录，跨会话保存仓库注册列表）
#[derive(Debug, Serialize, Deserialize)]
struct GlobalConfig {
    notebooks: Vec<NotebookEntry>,
}

impl GlobalConfig {
    fn new() -> Self {
        Self { notebooks: vec![] }
    }

    fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|p| p.join(GLOBAL_CONFIG_FILENAME))
    }

    fn load() -> Self {
        match Self::config_path() {
            Some(path) if path.exists() => {
                let content = fs::read_to_string(&path).unwrap_or_default();
                toml::from_str(&content).unwrap_or_else(|_| Self::new())
            }
            _ => Self::new(),
        }
    }

    fn save(&self) -> Result<(), StorageError> {
        let path = Self::config_path()
            .ok_or_else(|| StorageError::Other("无法确定用户目录".to_string()))?;
        let content = toml::to_string_pretty(self)
            .map_err(|e| StorageError::Other(format!("序列化全局配置失败: {}", e)))?;
        fs::write(&path, content)?;
        Ok(())
    }
}

// ============================================================
// Storage 核心结构体
// ============================================================

/// 笔记本存储管理（支持多仓库）
pub struct Storage {
    /// 已注册的仓库列表
    notebooks: Vec<NotebookEntry>,
}

impl Storage {
    /// 创建新的 Storage 实例（从持久化的全局配置加载）
    pub fn new() -> Self {
        let config = GlobalConfig::load();
        Self {
            notebooks: config.notebooks,
        }
    }

    // ---- 仓库管理 ----

    /// 初始化/注册一个新仓库
    pub fn init_notebook(&mut self, path: &str, name: Option<&str>) -> Result<usize, StorageError> {
        let abs_path = PathBuf::from(path).canonicalize()
            .map_err(|e| StorageError::Other(format!("路径无效: {}", e)))?;
        let notes_dir = abs_path.join(META_DIRECTORY);
        let meta_path = notes_dir.join(META_FILENAME);

        // 检查是否已注册过该路径
        if self.notebooks.iter().any(|n| n.path == notes_dir) {
            return Err(StorageError::Other(
                format!("该目录已注册为仓库: {}", path),
            ));
        }

        // 检查磁盘状态：.notes/notes.toml 是否已存在
        if meta_path.exists() {
            return Err(StorageError::NoteAlreadyExists(
                format!("目录 {} 已包含笔记本", path),
            ));
        }

        // 创建 .notes 目录
        fs::create_dir_all(&notes_dir)?;

        // 生成初始元数据
        let repo_name = name
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                abs_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unnamed")
                    .to_string()
            });

        let meta = NotebookMeta {
            notebook: NotebookInfo {
                version: NOTEBOOK_VERSION,
                created_at: Local::now().to_rfc3339(),
            },
            notes: vec![],
        };

        // 写入 notes.toml
        let content = toml::to_string_pretty(&meta)
            .map_err(|e| StorageError::Other(format!("序列化元数据失败: {}", e)))?;
        fs::write(&meta_path, content)?;

        // 注册到内存列表
        let entry = NotebookEntry {
            name: repo_name.clone(),
            path: notes_dir.clone(),
        };
        self.notebooks.push(entry);

        // 持久化到全局配置
        GlobalConfig { notebooks: self.notebooks.clone() }.save()?;

        Ok(self.notebooks.len() - 1)
    }

    /// 获取所有已注册的仓库（不可变引用）
    pub fn list_notebooks(&self) -> &[NotebookEntry] {
        &self.notebooks
    }

    /// 按索引获取仓库
    pub fn get_notebook(&self, index: usize) -> Result<&NotebookEntry, StorageError> {
        self.notebooks.get(index)
            .ok_or(StorageError::InvalidSelection(format!(
                "索引 {} 超出范围 (共 {} 个仓库)", index, self.notebooks.len()
            )))
    }

    /// 获取仓库数量
    pub fn notebook_count(&self) -> usize {
        self.notebooks.len()
    }

    // ---- 向后兼容方法 ----

    /// 检查是否有任何已初始化的仓库
    pub fn is_initialized(&self) -> bool {
        !self.notebooks.is_empty()
    }

    /// 兼容旧接口：获取第一个仓库的 .notes 路径
    pub fn notebook_path(&self) -> Option<&PathBuf> {
        self.notebooks.first().map(|n| &n.path)
    }

    /// 兼容旧接口：获取第一个仓库的内容根目录
    pub fn content_path(&self) -> Option<PathBuf> {
        self.notebooks.first().map(|n| n.path.parent().map(|p| p.to_path_buf())).flatten()
    }

    /// 兼容旧接口：获取元数据文件路径
    pub fn meta_path(&self) -> Option<PathBuf> {
        self.notebook_path().map(|p| p.join(META_FILENAME))
    }

    // ---- 笔记操作 ----

    /// 在指定仓库中创建笔记文件并更新 notes.toml
    pub fn create_note(&mut self, notebook_index: usize, filename: &str) -> Result<PathBuf, StorageError> {
        let entry = self.get_notebook(notebook_index)?;

        // 确保文件名以 .md 结尾
        let normalized_name = if filename.ends_with(".md") {
            filename.to_string()
        } else {
            format!("{}.md", filename)
        };

        // 笔记内容放在 .notes 的上级目录
        let content_dir = entry.path.parent()
            .ok_or_else(|| StorageError::Other("无效的仓库路径".to_string()))?;
        let note_full_path = content_dir.join(&normalized_name);

        // 检查是否已存在
        if note_full_path.exists() {
            return Err(StorageError::NoteAlreadyExists(format!("笔记已存在: {}", normalized_name)));
        }

        // 创建空笔记文件
        fs::write(&note_full_path, "")?;

        // 更新 notes.toml
        self.append_note_meta(notebook_index, &normalized_name)?;

        Ok(note_full_path)
    }

    /// 在指定仓库的 notes.toml 中追加一条 NoteMeta
    fn append_note_meta(&mut self, notebook_index: usize, filename: &str) -> Result<(), StorageError> {
        let entry = self.get_notebook(notebook_index)?;
        let meta_path = entry.path.join(META_FILENAME);

        // 读取现有元数据
        let mut meta: NotebookMeta = if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            toml::from_str(&content)
                .map_err(|e| StorageError::Other(format!("解析 notes.toml 失败: {}", e)))?
        } else {
            NotebookMeta {
                notebook: NotebookInfo {
                    version: NOTEBOOK_VERSION,
                    created_at: Local::now().to_rfc3339(),
                },
                notes: vec![],
            }
        };

        // 追加新笔记记录
        meta.notes.push(NoteMeta {
            filename: filename.to_string(),
            created_at: Local::now().to_rfc3339(),
        });

        // 写回
        let content = toml::to_string_pretty(&meta)
            .map_err(|e| StorageError::Other(format!("序列化元数据失败: {}", e)))?;
        fs::write(&meta_path, content)?;

        Ok(())
    }

    /// 列出指定仓库的所有笔记（从 notes.toml 读取）
    pub fn list_notes(&self, notebook_index: Option<usize>) -> Result<Vec<NoteMeta>, StorageError> {
        let idx = notebook_index.unwrap_or(0);
        let entry = self.get_notebook(idx)?;
        let meta_path = entry.path.join(META_FILENAME);

        if !meta_path.exists() {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(&meta_path)?;
        let meta: NotebookMeta = toml::from_str(&content)
            .map_err(|e| StorageError::Other(format!("解析 notes.toml 失败: {}", e)))?;

        Ok(meta.notes)
    }

    /// 获取指定笔记的完整路径
    pub fn get_note_path(&self, notebook_index: usize, filename: &str) -> Result<PathBuf, StorageError> {
        let entry = self.get_notebook(notebook_index)?;
        let content_dir = entry.path.parent()
            .ok_or_else(|| StorageError::Other("无效的仓库路径".to_string()))?;

        let normalized = if filename.ends_with(".md") {
            filename.to_string()
        } else {
            format!("{}.md", filename)
        };

        Ok(content_dir.join(normalized))
    }
}

/// 默认实现 Default trait
impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
