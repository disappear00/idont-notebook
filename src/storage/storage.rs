use std::fs;
use std::path::PathBuf;
use chrono::Local;

use super::error::StorageError;
use super::models::{
    NotebookEntry, NotebookMeta, NotebookInfo, TrackingInfo,
    NOTEBOOK_VERSION, META_FILENAME, META_DIRECTORY, DATA_DIRECTORY,
    GlobalConfig,
};

pub struct Storage {
    notebooks: Vec<NotebookEntry>,
    current_notebook: Option<usize>,
}

impl Storage {
    pub fn new() -> Self {
        let config = GlobalConfig::load();
        Self {
            notebooks: config.notebooks,
            current_notebook: None,
        }
    }

    // ---- 仓库管理 ----

    pub fn init_notebook(&mut self, path: &str, name: Option<&str>) -> Result<usize, StorageError> {
        let abs_path = PathBuf::from(path).canonicalize()
            .map_err(|e| StorageError::Other(format!("路径无效: {}", e)))?;
        let notes_dir = abs_path.join(META_DIRECTORY);
        let meta_path = notes_dir.join(META_FILENAME);

        if self.notebooks.iter().any(|n| n.path == notes_dir) {
            return Err(StorageError::Other(
                format!("该目录已注册为仓库: {}", path),
            ));
        }

        if meta_path.exists() {
            return Err(StorageError::NoteAlreadyExists(
                format!("目录 {} 已包含笔记本", path),
            ));
        }

        fs::create_dir_all(notes_dir.join(DATA_DIRECTORY))?;

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
            tracking: TrackingInfo::default(),
        };

        let content = toml::to_string_pretty(&meta)
            .map_err(|e| StorageError::Other(format!("序列化元数据失败: {}", e)))?;
        fs::write(&meta_path, content)?;

        let entry = NotebookEntry {
            name: repo_name.clone(),
            path: notes_dir.clone(),
        };
        self.notebooks.push(entry);

        GlobalConfig { notebooks: self.notebooks.clone() }.save()?;

        Ok(self.notebooks.len() - 1)
    }

    pub fn list_notebooks(&self) -> &[NotebookEntry] {
        &self.notebooks
    }

    pub fn get_notebook(&self, index: usize) -> Result<&NotebookEntry, StorageError> {
        self.notebooks.get(index)
            .ok_or(StorageError::InvalidSelection(format!(
                "索引 {} 超出范围 (共 {} 个仓库)", index, self.notebooks.len()
            )))
    }

    pub fn notebook_count(&self) -> usize {
        self.notebooks.len()
    }

    pub fn select_notebook(&mut self, selector: &str) -> Result<usize, StorageError> {
        if let Ok(index) = selector.parse::<usize>() {
            if index < self.notebooks.len() {
                self.current_notebook = Some(index);
                return Ok(index);
            }
            return Err(StorageError::InvalidSelection(
                format!("索引 {} 超出范围 (共 {} 个仓库)", index, self.notebooks.len()),
            ));
        }

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

    pub fn current_notebook_index(&self) -> Option<usize> {
        self.current_notebook
    }

    pub fn require_current_notebook(&self) -> Result<&NotebookEntry, StorageError> {
        let idx = self.current_notebook
            .ok_or(StorageError::NoNotebookSelected)?;
        self.get_notebook(idx)
    }

    pub fn require_current_index(&self) -> Result<usize, StorageError> {
        self.current_notebook.ok_or(StorageError::NoNotebookSelected)
    }

    pub fn is_initialized(&self) -> bool {
        !self.notebooks.is_empty()
    }

    // ---- 笔记操作 ----

    pub fn create_note(&mut self, notebook_index: usize, filename: &str) -> Result<PathBuf, StorageError> {
        let entry = self.get_notebook(notebook_index)?;
        let data_dir = entry.path.join(DATA_DIRECTORY);
        let note_full_path = data_dir.join(filename);

        if note_full_path.exists() {
            return Err(StorageError::NoteAlreadyExists(format!("笔记已存在: {}", filename)));
        }

        fs::write(&note_full_path, "")?;
        Ok(note_full_path)
    }

    pub fn get_note_path(&self, notebook_index: usize, filename: &str) -> Result<PathBuf, StorageError> {
        let entry = self.get_notebook(notebook_index)?;
        Ok(entry.path.join(DATA_DIRECTORY).join(filename))
    }

    /// 扫描 .notes/data/ 目录，返回所有文件名（按修改时间倒序）
    pub fn scan_notes(&self, notebook_index: usize) -> Result<Vec<String>, StorageError> {
        let entry = self.get_notebook(notebook_index)?;
        let data_dir = entry.path.join(DATA_DIRECTORY);

        if !data_dir.exists() {
            return Ok(vec![]);
        }

        let mut entries: Vec<(String, std::time::SystemTime)> = fs::read_dir(&data_dir)
            .map_err(|e| StorageError::Other(format!("读取目录失败: {}", e)))?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                let modified = e.metadata().ok()?.modified().ok()?;
                Some((name, modified))
            })
            .collect();

        entries.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(entries.into_iter().map(|(name, _)| name).collect())
    }

    /// 获取隐藏文件列表
    pub fn get_hidden_files(&self, notebook_index: usize) -> Result<Vec<String>, StorageError> {
        let entry = self.get_notebook(notebook_index)?;
        let meta_path = entry.path.join(META_FILENAME);

        if !meta_path.exists() {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(&meta_path)?;
        let meta: NotebookMeta = toml::from_str(&content)
            .map_err(|e| StorageError::Other(format!("解析 notes.toml 失败: {}", e)))?;

        Ok(meta.tracking.hidden)
    }

    /// 将文件加入隐藏列表
    pub fn hide_file(&mut self, notebook_index: usize, filename: &str) -> Result<(), StorageError> {
        self.with_tracking_mut(notebook_index, |tracking| {
            if tracking.hidden.iter().any(|f| f == filename) {
                return Err(StorageError::Other(format!("文件已隐藏: {}", filename)));
            }
            tracking.hidden.push(filename.to_string());
            Ok(())
        })
    }

    /// 将文件从隐藏列表移除
    pub fn unhide_file(&mut self, notebook_index: usize, filename: &str) -> Result<(), StorageError> {
        let filename_owned = filename.to_string();
        self.with_tracking_mut(notebook_index, |tracking| {
            let before = tracking.hidden.len();
            tracking.hidden.retain(|f| f != &filename_owned);
            if tracking.hidden.len() == before {
                return Err(StorageError::Other(format!("文件未被隐藏: {}", filename)));
            }
            Ok(())
        })
    }

    /// 从隐藏列表中移除指定文件（用于 rmnote 清理）
    pub fn remove_from_hidden(&mut self, notebook_index: usize, filename: &str) -> Result<(), StorageError> {
        let filename_owned = filename.to_string();
        self.with_tracking_mut(notebook_index, |tracking| {
            tracking.hidden.retain(|f| f != &filename_owned);
            Ok(())
        })
    }

    fn with_tracking_mut<F>(&mut self, notebook_index: usize, op: F) -> Result<(), StorageError>
    where
        F: FnOnce(&mut TrackingInfo) -> Result<(), StorageError>,
    {
        let entry = self.get_notebook(notebook_index)?;
        let meta_path = entry.path.join(META_FILENAME);

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
                tracking: TrackingInfo::default(),
            }
        };

        op(&mut meta.tracking)?;

        let content = toml::to_string_pretty(&meta)
            .map_err(|e| StorageError::Other(format!("序列化元数据失败: {}", e)))?;
        fs::write(&meta_path, content)?;

        Ok(())
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
