trait Animal {
    fn sound(&self) -> &'static str;
}
trait Pet: Animal {
    fn name(&self) -> &'static str;
}

trait Speak {
    fn speak(&self) {
        println!("Hello!");
    }
}

struct Dog;

impl Animal for Dog {
    fn sound(&self) -> &'static str {
        "Woof!"
    }
}
impl Dog {
    fn new() -> Self {
        Dog
    }
}
impl Pet for Dog {
    fn name(&self) -> &'static str {
        "Buddy"
    }
}
impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn print_sound<T: Animal>(animal: T) {
    println!("The animal says: {}", animal.sound());
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) -> &'static str {
        "Meow!"
    }
}
impl Pet for Cat {
    fn name(&self) -> &'static str {
        "Tom"
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

/// 接受一个dyn Speak类型的Trait对象，表示可以传入任何实现了Speak Trait的类型。动态分发让我们能够在运行时调用不同类型的speak方法。
fn make_speak(animal: &dyn Speak) {
    animal.speak();
}

mod tests {
    use super::*;

    #[test]
    fn test_dog() {
        let dog = Dog::new();
        assert_eq!(dog.sound(), "Woof!");
        assert_eq!(dog.name(), "Buddy");
    }

    #[test]
    fn test_cat() {
        let cat = Cat;
        assert_eq!(cat.sound(), "Meow!");
    }
    #[test]
    fn test_print_sound() {
        let cat = Cat;
        print_sound(cat);
    }

    #[test]
    fn test_make_speak() {
        let dog = Dog::new();
        make_speak(&dog);
    }

    #[test]
    fn test_trait_object() {
        let dog = Dog::new();
        let cat = Cat;
        let animals = vec![&dog as &dyn Pet, &cat as &dyn Pet];
        for animal in animals {
            println!("{} says: {}", animal.name(), animal.sound());
        }
    }

    #[test]
    fn test_trait_object_as_any() {
        let dog = Dog;
        let cat = Cat;
        make_speak(&dog);
        make_speak(&cat);
    }
}
