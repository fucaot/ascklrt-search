// create by goumang in 2022/11/27 16:24

use std::{fs::File, path::Path};


// 根据路径字符切片打开文件
fn open_file(path: &str) -> File{
    let path = Path::new(path);
    // let display = path.display();
    open_file_use_path(path)
}

// 根据path打开文件
fn open_file_use_path(path: &Path) -> File{
    match File::open(path) {
        Ok(file) => file,
        Err(way) => panic!("why: {:?}", way)
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use super::*;

    #[test]
    fn open_file () {
        let file = super::open_file("/Users/wangjiawei/Develop/CodeSpace/Rust");
        println!("open file: {:?}", file)
    }
}
