use std::fmt;
use std::error;
use std::io::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum CustomError {
    IoError(Error),
    ParseError(ParseIntError),
    // 其他自定义错误类型
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::IoError(err) => write!(f, "IO error: {}", err),
            CustomError::ParseError(err) => write!(f, "Parse error: {}", err),
            // 其他自定义错误类型的格式化
        }
    }
}

impl error::Error for CustomError {}

// 为其他错误类型实现 From trait
impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for CustomError {
    fn from(err: std::num::ParseIntError) -> CustomError {
        CustomError::ParseError(err)
    }
}
