use std::fs;
use super::{Searcher, SearchResult, MatchedLine};
use crate::storage::models::NotebookEntry;
use crate::storage::error::StorageError;
use crate::storage::models::DATA_DIRECTORY;

pub struct SimpleSearcher;

impl Searcher for SimpleSearcher {
    fn search(&self, keyword: &str, notebooks: &[NotebookEntry]) -> Result<Vec<SearchResult>, StorageError> {
        let mut results = Vec::new();
        
        for (notebook_index, notebook) in notebooks.iter().enumerate() {
            let data_dir = notebook.path.join(DATA_DIRECTORY);
            if !data_dir.exists() {
                continue;
            }
            
            let entries = fs::read_dir(&data_dir)
                .map_err(|e| StorageError::Other(format!("读取目录失败: {}", e)))?;
            
            for entry in entries {
                let entry = entry.map_err(|e| StorageError::Other(format!("读取目录项失败: {}", e)))?;
                let path = entry.path();
                
                if !path.is_file() {
                    continue;
                }
                
                let filename = path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                
                let mut matched_lines = Vec::new();
                
                if filename.contains(keyword) {
                    matched_lines.push(MatchedLine {
                        line_number: 0,
                        content: format!("文件名匹配: {}", filename),
                    });
                }
                
                if let Ok(content) = fs::read_to_string(&path) {
                    for (line_number, line) in content.lines().enumerate() {
                        if line.contains(keyword) {
                            matched_lines.push(MatchedLine {
                                line_number: line_number + 1,
                                content: line.to_string(),
                            });
                        }
                    }
                }
                
                if !matched_lines.is_empty() {
                    results.push(SearchResult {
                        notebook_name: notebook.name.clone(),
                        notebook_index,
                        filename,
                        matched_lines,
                    });
                }
            }
        }
        
        Ok(results)
    }
}
