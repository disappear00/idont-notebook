pub mod error;
pub mod storage;

// 重导出
pub use error::StorageError;
pub use storage::{
    Storage, NotebookMeta, NotebookInfo, NoteMeta, NotebookEntry,
};
