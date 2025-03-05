use std::io;
use std::io::Read;
use std::fs::File;

use anyhow::{Context, Result};

pub fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_contents(file_path: &str) -> Result<String> {
    std::fs::read_to_string(file_path).context("Failed to read file")
}