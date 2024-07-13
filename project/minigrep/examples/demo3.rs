// 文件读取

use std::env;
use std::fs;

fn main() {
    let file_path = "poem.txt";
    // --省略之前的内容--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}