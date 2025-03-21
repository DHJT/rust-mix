/// 闭包可以作为函数参数
/// 例如迭代器的 .map()、.filter() 方法：
fn apply_to_value<F>(val: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(val)
}
///  闭包可以作为返回值：使用 impl Fn 返回闭包
fn make_adder_1(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
///  闭包可以作为返回值：使用 Box<dyn Fn> 返回闭包
fn make_adder_2(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}
fn call_closure<F>(f: F)
where
    F: FnOnce(),
{
    f(); // 只调用一次
}

/// Rust 中的闭包是一种匿名函数，它们可以捕获并存储其环境中的变量。
// 闭包允许在其定义的作用域之外访问变量，并且可以在需要时将其移动或借用给闭包。
// 闭包在 Rust 中被广泛应用于函数式编程、并发编程和事件驱动编程等领域。
// 闭包在 Rust 中非常有用，因为它们提供了一种简洁的方式来编写和使用函数。
// 闭包在 Rust 中非常灵活，可以存储在变量中、作为参数传递，甚至作为返回值。
// 闭包通常用于需要短小的自定义逻辑的场景，例如迭代器、回调函数等。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_to_value() {
        let double = |x| x * 2;
        let result = apply_to_value(5, double);
        println!("Result: {}", result); // 输出: Result: 10
    }

    #[test]
    fn test_make_adder_1() {
        let add_five = make_adder_1(5);
        println!("5 + 3 = {}", add_five(3)); // 输出: 5 + 3 = 8
    }

    #[test]
    fn test_make_adder_2() {
        let add_ten = make_adder_2(10);
        println!("10 + 2 = {}", add_ten(2)); // 输出: 10 + 2 = 12
    }

    #[test]
    fn test_call_closure() {
        let name = String::from("Rust");

        // 使用 move 强制捕获所有权
        let print_name = move || println!("Hello, {}!", name);

        call_closure(print_name);
        // println!("{}", name); // 若取消注释，将报错，name 的所有权已被移动
    }

    #[test]
    fn test_closure_life_cycle() {
        let add_one = |x: i32| x + 1;
        let mut calculate = |a, b, c| a * b + c;
        let _result = calculate(1, 2, 3);
        // let _result = calculate(1.0, 2.0, 3.0);

        let add = |a, b| a + b;
        println!("{}", add(2, 3)); // 输出: 5
    }

    /// __捕获外部变量__
    ///
    /// 可以通过三种方式捕获外部变量：
    ///
    /// - 按引用捕获（默认行为，类似 `&T`）
    /// - 按值捕获（类似 T）
    /// - 可变借用捕获（类似 &mut T）
    #[test]
    fn test_closure_life_cycle_2() {
        let x = 5;
        let square = |num| num * x;
        println!("x: {}, square: {}", x, square(3)); // 输出: 15

        let mut num = 5;

        // 按引用捕获
        let print_num = || println!("num = {:?}", num);
        print_num(); // 输出: num = 5

        // 按值捕获
        let take_num = move || println!("num taken = {}", num);
        take_num(); // 输出: num taken = 5
        println!("{}", num); // 若取消注释，不会报错，num 是基本类型，实现了Copy 特性，优先进行赋值后然后再进行所有权转移

        let s = String::from("hello");
        let print_s = move || println!("{}", s);
        print_s(); // 输出 "hello"
                   // println!("{}", s); // 这行代码将会报错，因为 s 的所有权已经被转移给了闭包

        // 可变借用捕获
        // let mut change_num: dyn FnMut() = || num += 1;
        let mut change_num = || num += 1;
        change_num();
        println!("num after closure = {}", num); // 输出: num after closure = 6
    }
}
