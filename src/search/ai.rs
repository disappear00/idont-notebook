use crate::storage::models::NotebookEntry;
use crate::storage::error::StorageError;
use super::{Searcher, SearchResult};

pub struct AiSearcher {
    _api_key: String,
}

impl AiSearcher {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl Searcher for AiSearcher {
    fn search(&self, _keyword: &str, _notebooks: &[NotebookEntry]) -> Result<Vec<SearchResult>, StorageError> {
        todo!("AiSearcher::search not yet implemented")
    }
}
