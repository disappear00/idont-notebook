use std::fs;
use serde_json::{json, Value};
use super::{Searcher, SearchResult, MatchedLine};
use crate::storage::models::NotebookEntry;
use crate::storage::error::StorageError;
use crate::storage::models::DATA_DIRECTORY;

pub struct AiSearcher {
    api_key: String,
}

impl AiSearcher {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
    
    fn call_openai_api(&self, keyword: &str, content: &str) -> Result<Vec<MatchedLine>, StorageError> {
        let client = reqwest::blocking::Client::new();
        
        let prompt = format!(
            "请分析以下文本内容，找出与关键词\"{}\"语义相关的行。\
             返回JSON数组，每个元素包含line_number(行号)和content(行内容)。\
             只返回JSON，不要其他文字。\n\n文本内容:\n{}",
            keyword, content
        );
        
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [{"role": "user", "content": prompt}],
                "temperature": 0.3
            }))
            .send()
            .map_err(|e| StorageError::Other(format!("API请求失败: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(StorageError::Other(format!("API返回错误: {}", response.status())));
        }
        
        let response_body: Value = response.json()
            .map_err(|e| StorageError::Other(format!("解析API响应失败: {}", e)))?;
        
        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("[]");
        
        let parsed: Vec<Value> = serde_json::from_str(content)
            .unwrap_or_default();
        
        let matched_lines = parsed.iter().filter_map(|item| {
            Some(MatchedLine {
                line_number: item["line_number"].as_u64()? as usize,
                content: item["content"].as_str()?.to_string(),
            })
        }).collect();
        
        Ok(matched_lines)
    }
}

impl Searcher for AiSearcher {
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
                
                // 检查文件名是否匹配
                let mut matched_lines = Vec::new();
                if filename.contains(keyword) {
                    matched_lines.push(MatchedLine {
                        line_number: 0,
                        content: format!("文件名匹配: {}", filename),
                    });
                }
                
                // 读取文件内容并调用AI API
                if let Ok(content) = fs::read_to_string(&path) {
                    match self.call_openai_api(keyword, &content) {
                        Ok(ai_matches) => matched_lines.extend(ai_matches),
                        Err(e) => {
                            eprintln!("AI搜索失败，回退到简单匹配: {}", e);
                            // 回退到简单匹配
                            for (line_number, line) in content.lines().enumerate() {
                                if line.contains(keyword) {
                                    matched_lines.push(MatchedLine {
                                        line_number: line_number + 1,
                                        content: line.to_string(),
                                    });
                                }
                            }
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
