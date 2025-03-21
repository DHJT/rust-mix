/// 关联类型（Associated Types）关联类型是Rust中一个非常强大的概念，它允许你在Trait中定义类型参数，进而使得 Trait的实现更加灵活和简洁。
/// Container Trait定义了一个关联类型Item，并且BoxContainer实现了该 Trait，其中Item被定义为i32类型。关联类型使得代码更加简洁和可扩展。
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn remove(&mut self) -> Option<Self::Item>;
}

struct BoxContainer {
    items: Vec<i32>,
}

impl Container for BoxContainer {
    type Item = i32;
    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }
    fn remove(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let mut box_container = BoxContainer { items: Vec::new() };
        box_container.add(1);
        println!("{:?}", box_container.remove());
    }

}
