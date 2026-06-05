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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_simple_search_filename_match() {
        let dir = tempdir().unwrap();
        let notebook_path = dir.path().join(".notes").join("data");
        fs::create_dir_all(&notebook_path).unwrap();
        
        fs::write(notebook_path.join("rust学习笔记.md"), "这是测试内容").unwrap();
        
        let notebooks = vec![NotebookEntry {
            name: "测试仓库".to_string(),
            path: dir.path().join(".notes"),
        }];
        
        let searcher = SimpleSearcher;
        let results = searcher.search("rust", &notebooks).unwrap();
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].filename, "rust学习笔记.md");
    }

    #[test]
    fn test_simple_search_content_match() {
        let dir = tempdir().unwrap();
        let notebook_path = dir.path().join(".notes").join("data");
        fs::create_dir_all(&notebook_path).unwrap();
        
        fs::write(notebook_path.join("test.md"), "这是rust学习内容\n第二行").unwrap();
        
        let notebooks = vec![NotebookEntry {
            name: "测试仓库".to_string(),
            path: dir.path().join(".notes"),
        }];
        
        let searcher = SimpleSearcher;
        let results = searcher.search("rust", &notebooks).unwrap();
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].matched_lines.len(), 1);
        assert_eq!(results[0].matched_lines[0].line_number, 1);
    }
}
