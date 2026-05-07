use std::fs;
use std::path::PathBuf;
use chrono::Local;

use super::error::StorageError;
use super::models::{
    NotebookEntry, NotebookMeta, NotebookInfo, NoteMeta,
    NOTEBOOK_VERSION, META_FILENAME, META_DIRECTORY, DATA_DIRECTORY,
    GlobalConfig,
};

/// 笔记本存储管理（支持多仓库）
pub struct Storage {
    /// 已注册的仓库列表
    notebooks: Vec<NotebookEntry>,
    /// 当前选中的仓库索引
    current_notebook: Option<usize>,
}

impl Storage {
    /// 创建新的 Storage 实例（从持久化的全局配置加载）
    pub fn new() -> Self {
        let config = GlobalConfig::load();
        Self {
            notebooks: config.notebooks,
            current_notebook: None,
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

        // 创建 .notes 目录（含 data 子目录）
        fs::create_dir_all(notes_dir.join(DATA_DIRECTORY))?;

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

    /// 选中一个仓库（按索引或名称）
    pub fn select_notebook(&mut self, selector: &str) -> Result<usize, StorageError> {
        // 先尝试按索引选择
        if let Ok(index) = selector.parse::<usize>() {
            if index < self.notebooks.len() {
                self.current_notebook = Some(index);
                return Ok(index);
            }
            return Err(StorageError::InvalidSelection(
                format!("索引 {} 超出范围 (共 {} 个仓库)", index, self.notebooks.len()),
            ));
        }

        // 按名称选择
        for (i, nb) in self.notebooks.iter().enumerate() {
            if nb.name == selector {
                self.current_notebook = Some(i);
                return Ok(i);
            }
        }

        Err(StorageError::InvalidSelection(
            format!("未找到名为 '{}' 的仓库", selector),
        ))
    }

    /// 获取当前选中的仓库索引
    pub fn current_notebook_index(&self) -> Option<usize> {
        self.current_notebook
    }

    /// 获取当前选中的仓库（返回错误如果未选中）
    pub fn require_current_notebook(&self) -> Result<&NotebookEntry, StorageError> {
        let idx = self.current_notebook
            .ok_or(StorageError::NoNotebookSelected)?;
        self.get_notebook(idx)
    }

    /// 获取当前选中的仓库索引（返回错误如果未选中）
    pub fn require_current_index(&self) -> Result<usize, StorageError> {
        self.current_notebook.ok_or(StorageError::NoNotebookSelected)
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

        // 要求文件名必须包含扩展名
        if std::path::Path::new(filename).extension().is_none() {
            return Err(StorageError::Other(
                format!("请为文件名添加后缀，例如: {}.md", filename),
            ));
        }
        let normalized_name = filename.to_string();

        // 笔记内容放在 .notes/data 目录内部
        let data_dir = entry.path.join(DATA_DIRECTORY);
        let note_full_path = data_dir.join(&normalized_name);

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
        self.with_notebook_meta_mut(notebook_index, |meta| {
            meta.notes.push(NoteMeta {
                filename: filename.to_string(),
                created_at: Local::now().to_rfc3339(),
            });
            Ok(())
        })
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

        if std::path::Path::new(filename).extension().is_none() {
            return Err(StorageError::Other(
                format!("请为文件名添加后缀，例如: {}.md", filename),
            ));
        }

        Ok(entry.path.join(DATA_DIRECTORY).join(filename))
    }

    /// 对指定仓库的 notes.toml 执行读-改-写操作（消除重复代码）
    fn with_notebook_meta_mut<F>(&mut self, notebook_index: usize, op: F) -> Result<(), StorageError>
    where
        F: FnOnce(&mut NotebookMeta) -> Result<(), StorageError>,
    {
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

        // 执行修改回调
        op(&mut meta)?;

        // 写回磁盘
        let content = toml::to_string_pretty(&meta)
            .map_err(|e| StorageError::Other(format!("序列化元数据失败: {}", e)))?;
        fs::write(&meta_path, content)?;

        Ok(())
    }

    /// 从指定仓库的 notes.toml 中移除一条 NoteMeta
    pub fn remove_note_meta(&mut self, notebook_index: usize, filename: &str) -> Result<(), StorageError> {
        let filename_owned = filename.to_string();
        self.with_notebook_meta_mut(notebook_index, |meta| {
            let before = meta.notes.len();
            meta.notes.retain(|n| n.filename != filename_owned);
            if meta.notes.len() == before {
                return Err(StorageError::NoteNotFound(filename_owned));
            }
            Ok(())
        })
    }

}

/// 默认实现 Default trait
impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
