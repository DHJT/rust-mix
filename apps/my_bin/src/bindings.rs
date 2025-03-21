use libc::{c_int, c_void};// 如果需要C标准库的功能，比如printf等，可以包含此模块。通常不是必须的。
// use std::os::raw::c_int;

// 手动声明 C 函数和结构体
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: c_int,
    pub y: c_int,
}

// #[link(name = "example")] // 注意这里的名字要和动态库的文件名相对应（去掉前缀lib和后缀.so/.dll）
extern "C" {
    // 注意这里的函数签名要和C中的对应。对于Windows DLL，通常是去掉前缀lib和后缀.dll的名字。
    pub fn add(a: c_int, b: c_int) -> c_int;
    pub fn print_point(p: Point);
}