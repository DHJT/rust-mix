// 定义两个有同名方法的 trait_1
trait Driver {
    fn drive(&self) {
        println!("Driving a car");
    }
}

trait Pilot {
    fn drive(&self) {
        println!("Flying a plane");
    }
}

// 结构体同时实现两个 trait_1
struct Person;

impl Driver for Person {
    fn drive(&self) {
        println!("Person is driving a car");
    }
}

impl Pilot for Person {
    fn drive(&self) {
        println!("Person is flying a plane");
    }
}

trait Factory {
    fn create() -> String {
        String::from("Product")
    }
}

trait Lab {
    fn create() -> String {
        String::from("Experiment")
    }
}

struct Facility;

impl Factory for Facility {}
impl Lab for Facility {}


mod tests {
    use super::*;

    #[test]
    fn test_same_name() {
        let person = Person;

        // 方式 1：通过作用域指定调用哪个 trait_1 的方法
        <Person as Driver>::drive(&person); // 输出：Person is driving a car
        <Person as Pilot>::drive(&person);  // 输出：Person is flying a plane
    }

    /// 如果多个 trait_1 中存在同名的关联函数（无 self 参数），必须通过完全限定语法调用。
    #[test]
    fn test_associated_functions() {
        // 必须使用完全限定语法
        let product = <Facility as Factory>::create();
        let experiment = <Facility as Lab>::create();
        println!("{} and {}", product, experiment); // 输出：Product and Experiment
    }
}
