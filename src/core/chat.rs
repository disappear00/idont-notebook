use std::io::{self, Write};

use crate::search::{self, SearchResult};
use crate::storage::{Storage, StorageError};

const PAGE_SIZE: usize = 20;

pub fn chat(storage: &mut Storage, keyword: &str) -> Result<(), StorageError> {
    let notebooks = storage.list_notebooks();

    if notebooks.is_empty() {
        println!("没有已注册的仓库");
        return Ok(());
    }

    let searcher = search::create_searcher();
    let results = searcher.search(keyword, notebooks)?;

    if results.is_empty() {
        println!("未找到匹配结果");
        return Ok(());
    }

    display_results(&results, 0);

    let mut matched_notebooks: Vec<(usize, String)> = Vec::new();
    for result in &results {
        if !matched_notebooks.iter().any(|(idx, _)| *idx == result.notebook_index) {
            matched_notebooks.push((result.notebook_index, result.notebook_name.clone()));
        }
    }

    select_notebook(storage, &matched_notebooks)
}

fn display_results(results: &[SearchResult], start: usize) {
    let end = std::cmp::min(start + PAGE_SIZE, results.len());

    println!("搜索结果 (共 {} 条):\n", results.len());

    for (i, result) in results[start..end].iter().enumerate() {
        println!("{}. 仓库: {}", start + i + 1, result.notebook_name);
        println!("   文件: {}", result.filename);
        println!("   匹配行:");
        for matched_line in &result.matched_lines {
            if matched_line.line_number == 0 {
                println!("     {}", matched_line.content);
            } else {
                println!("     第{}行: {}", matched_line.line_number, matched_line.content);
            }
        }
        println!();
    }

    if end < results.len() {
        println!("还有 {} 条结果未显示", results.len() - end);
    }
}

fn select_notebook(storage: &mut Storage, notebooks: &[(usize, String)]) -> Result<(), StorageError> {
    loop {
        println!("请选择要进入的仓库 (输入编号，或输入 0 取消):");
        for (i, (_, name)) in notebooks.iter().enumerate() {
            println!("{}. {}", i + 1, name);
        }

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() || input == "0" {
            println!("已取消搜索");
            return Ok(());
        }

        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= notebooks.len() {
                let (notebook_index, notebook_name) = &notebooks[choice - 1];
                storage.select_notebook(&notebook_index.to_string())?;
                println!("已选中仓库: {}", notebook_name);
                return Ok(());
            }
        }

        println!("请输入有效的仓库编号");
    }
}
