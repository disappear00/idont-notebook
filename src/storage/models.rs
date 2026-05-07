// 数据模型：笔记仓库的持久化结构定义

use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// 全局配置文件名（存储在用户目录下）
pub const GLOBAL_CONFIG_FILENAME: &str = "idont-notebook-config.toml";
pub const META_FILENAME: &str = "notes.toml";
pub const NOTEBOOK_VERSION: u32 = 1;
pub const META_DIRECTORY: &str = ".notes";
pub const DATA_DIRECTORY: &str = "data";

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
pub struct GlobalConfig {
    pub notebooks: Vec<NotebookEntry>,
}
