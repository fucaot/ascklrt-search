#[allow(dead_code)]
// create by goumang in 2022/11/27 16:42

use super::file;
use std::{fs::{File, self}, path::Path};

// 根据文件名搜索文件
fn search_use_filename(filename: &str) -> String {
    search_use_path_name(".", filename)
}

// 根据文件名搜索文件
fn search_use_path_name(root_path: &str, filename: &str) -> String {
    let this_dir = Path::new(root_path);
    match search_use_path(this_dir, filename) {
        None => "not found".to_string(),
        Some(p) => p
    }
}

fn search_use_path(path: &Path, filename: &str) -> Option<String> {
    println!("in {:?}", path.to_str());
    if path.is_dir() {
        let dir = path.read_dir().expect("read dir failed!");
        for entry in dir {
            let this_path = entry.unwrap().path();
            if let Some(res) = search_use_path(this_path.as_path(), filename) {
                return Some(res);
            }
        }
    } else {
        if let Some(n) = path.file_name() {
            let this_file = n.to_str().unwrap();
            if this_file == filename {
                return Some(String::from(this_file));
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_use_filename() {
        let resp = super::search_use_filename("lib.rs");
        println!("{}", resp)
    }
}