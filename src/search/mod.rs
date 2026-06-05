pub mod simple;
pub mod ai;

use crate::storage::models::NotebookEntry;
use crate::storage::error::StorageError;

#[derive(Debug)]
pub struct SearchResult {
    pub notebook_name: String,
    pub notebook_index: usize,
    pub filename: String,
    pub matched_lines: Vec<MatchedLine>,
}

#[derive(Debug)]
pub struct MatchedLine {
    pub line_number: usize,
    pub content: String,
}

pub trait Searcher {
    fn search(&self, keyword: &str, notebooks: &[NotebookEntry]) -> Result<Vec<SearchResult>, StorageError>;
}

pub fn create_searcher() -> Box<dyn Searcher> {
    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        Box::new(ai::AiSearcher::new(api_key))
    } else {
        Box::new(simple::SimpleSearcher)
    }
}
