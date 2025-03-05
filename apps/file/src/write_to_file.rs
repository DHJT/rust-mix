use std::fs::File;
use std::io::{self, Write};

use std::fs::read_to_string;

pub fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// 读取文件
pub fn read_from_file(filename: &str) -> io::Result<String> {
    let content = read_to_string(filename)?;
    Ok(content)
}
