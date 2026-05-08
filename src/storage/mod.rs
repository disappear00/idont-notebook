pub mod config;
pub mod error;
pub mod models;
pub mod storage;

// 重导出
pub use error::StorageError;
pub use storage::Storage;
// 允许 #[allow(unused_imports)] — 这些类型被 core/ 模块直接引用
#[allow(unused_imports)]
pub use models::{NotebookMeta, NotebookInfo, NotebookEntry};
