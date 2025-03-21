mod trait_1;
use my_lib::Person;
use my_lib::debug_print;
use my_lib::utils::{time, file};
// use my_lib::utils::file::read_text_from_file;
use neural_network::neural_network::NeuralNetwork;
use neural_network::layer::mean_squared_error;

use std::io;

// 引入自动生成的绑定
mod bindings;
use bindings::{add, print_point, Point};

fn main() {
    // 调用 C 函数（需在 unsafe 块中）
    unsafe {
        // 示例 1：调用简单函数
        let result = add(2, 3);
        println!("2 + 3 = {}", result);

        // 示例 2：传递结构体
        let p = Point { x: 10, y: 20 };
        print_point(p);
    }

    let person = Person::new("Alice".to_string(), 30);
    println!("{}", person.greet());

    // test_neural_network();
    test_debug_print();
    time::sleep_for_ms(10000);

    if_test();

}

fn if_test() {
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}, {}", number, a);
}

fn test_neural_network() {
    let mut network = NeuralNetwork::new(&[2, 3, 1]);
    let data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];

    let learning_rate = 0.1;
    for epoch in 0..5000 {
        let mut loss = 0.0;
        for (input, target) in &data {
            let prediction = network.forward(input);
            loss += mean_squared_error(&prediction, target);
            network.backward(input, target, learning_rate);
        }

        if epoch % 1000 == 0 {
            println!("Epoch {}: Loss = {}", epoch, loss / data.len() as f64);
        }
    }
}

fn test_debug_print() {
    let value = 42;
    debug_print!("The value is {}", value);
}
fn test_util_file() {
    let str_file = my_lib::utils::read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}
