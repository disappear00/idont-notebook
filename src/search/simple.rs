use crate::storage::models::NotebookEntry;
use crate::storage::error::StorageError;
use super::{Searcher, SearchResult};

pub struct SimpleSearcher;

impl Searcher for SimpleSearcher {
    fn search(&self, _keyword: &str, _notebooks: &[NotebookEntry]) -> Result<Vec<SearchResult>, StorageError> {
        todo!("SimpleSearcher::search not yet implemented")
    }
}
