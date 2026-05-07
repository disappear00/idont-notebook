// GlobalConfig 持久化：跨会话保存仓库注册列表

use std::fs;
use std::path::PathBuf;

use super::error::StorageError;
use super::models::{GlobalConfig, GLOBAL_CONFIG_FILENAME};

impl GlobalConfig {
    pub fn new() -> Self {
        Self { notebooks: vec![] }
    }

    fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|p| p.join(".idont").join(GLOBAL_CONFIG_FILENAME))
    }

    pub fn load() -> Self {
        match Self::config_path() {
            Some(path) if path.exists() => {
                let content = fs::read_to_string(&path).unwrap_or_default();
                toml::from_str(&content).unwrap_or_else(|_| Self::new())
            }
            _ => Self::new(),
        }
    }

    pub fn save(&self) -> Result<(), StorageError> {
        let path = Self::config_path()
            .ok_or_else(|| StorageError::Other("无法确定用户目录".to_string()))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)
            .map_err(|e| StorageError::Other(format!("序列化全局配置失败: {}", e)))?;
        fs::write(&path, content)?;
        Ok(())
    }
}
