use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub const GLOBAL_CONFIG_FILENAME: &str = "idont-notebook-config.toml";
pub const META_FILENAME: &str = "notes.toml";
pub const NOTEBOOK_VERSION: u32 = 1;
pub const META_DIRECTORY: &str = ".notes";
pub const DATA_DIRECTORY: &str = "data";

#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookMeta {
    pub notebook: NotebookInfo,
    #[serde(default)]
    pub tracking: TrackingInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookInfo {
    pub version: u32,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TrackingInfo {
    #[serde(default)]
    pub hidden: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookEntry {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub notebooks: Vec<NotebookEntry>,
}
