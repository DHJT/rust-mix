
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    pub fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old.", self.name, self.age)
    }
}

/// 自定义宏，用于打印调试信息
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        println!("DEBUG: {}", format!($($arg)*));
    };
}

pub mod utils;
mod protocol;
pub mod arithmetic;