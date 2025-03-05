mod http;
mod write_to_file;
mod serialize;

use serde_json::{to_string_pretty};
use crate::serialize::{deserialize, serialize};

fn main() {
    match write_to_file::write_to_file("temp/output.txt", "Hello, Rust1----!") {
        Ok(_) => println!("File written successfully."),
        Err(e) => println!("Failed to write to file: {}", e),
    }

    match write_to_file::read_from_file("temp/output.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => println!("Failed to read file: {}", e),
    }

    serialize().expect("something is wrong.");
    deserialize().expect("something is wrong.");

    let url = "https://jsonplaceholder.typicode.com/todos/1";
    match http::fetch_data(url) {
        Ok(data) => {
            println!("Fetched data: {:#?}", data);
            // 序列化为格式化的 JSON 字符串
            let json_str = to_string_pretty(&data);

            // 输出到控制台
            println!("Serialized response:\n{:#?}", json_str);
        },
        Err(e) => println!("Error fetching data: {}", e),
    }
}
