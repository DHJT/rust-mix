use backtrace::Backtrace;
use std::error::Error;
use std::fmt;

//捕获自定义错误时的调用栈
// 如果使用 Result<T, E> 处理错误，可以通过 backtrace 库手动捕获调用栈。
#[derive(Debug)]
pub struct CustomError {
    message: String,
    backtrace: Backtrace,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nBacktrace:\n{:?}", self.message, self.backtrace)
    }
}

impl Error for CustomError {}


// 测试函数
fn might_fail() -> Result<(), CustomError> {
    // if some_condition {
    if 1 > 0 {
        Err(CustomError {
            message: "something went wrong".to_string(),
            backtrace: Backtrace::new(),
        })
    } else {
        Ok(())
    }
}
