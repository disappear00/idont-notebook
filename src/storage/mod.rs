pub mod error;
pub mod storage;

// 重导出，保持对外接口不变
pub use error::StorageError;
pub use storage::Storage;

