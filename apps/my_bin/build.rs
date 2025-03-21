extern crate cc;

fn main() {
    // 编译 C 代码为静态库
    cc::Build::new()
        .file("src/mylib.c")
        .include("include") // 头文件目录
        .compile("mylib"); // 输出库名称为 libmylib.a
}