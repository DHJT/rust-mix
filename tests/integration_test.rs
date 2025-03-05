// 集成测试
use my_lib::arithmetic::add::{add, add_two};

#[test]
fn it_adds_two() {
    assert_eq!(add_two(2), 4);
    assert_eq!(add(2, 4), 6);
}