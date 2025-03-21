#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn with_age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }
}

fn print_user(user: &User) {
    println!("User: {:?}", user);
}

fn update_age(user: &mut User) {
    user.age += 1;
}


#[derive(Debug, Clone)] // 必须实现 Clone trait
struct Data {
    content: String,
}

fn process_data(data: Data) {
    println!("Processing: {}", data.content);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 仅需读取数据（不修改）
    // 通过 不可变引用（&T）共享访问权，避免所有权转移：
    #[test]
    fn test_life_cycle1() -> Result<(), ()> {
        let user = User {
            name: String::from("Alice"),
            age: 30,
        };

        // 第一次使用（传引用）
        print_user(&user);

        // 第二次使用（仍有效）
        print_user(&user);
        Ok(())
    }

    /// 需要修改数据
    // 通过 可变引用（&mut T）临时独占访问权，注意同一作用域内只能存在一个可变引用：
    #[test]
    fn test_life_cycle2() -> Result<(), ()> {
        let mut user = User {
            name: String::from("Bob"),
            age: 25,
        };

        // 第一次修改
        update_age(&mut user);

        // 第二次修改（需确保前一次可变引用已释放）
        update_age(&mut user);
        Ok(())
    }

    /// 必须转移所有权但需重复使用
    // 通过 克隆（Clone）创建数据的完整副本：
    #[test]
    fn test_life_cycle3() -> Result<(), ()> {
        let data = Data {
            content: String::from("Important info"),
        };

        // 第一次使用（转移所有权）
        process_data(data.clone());

        // 第二次使用克隆副本
        process_data(data);
        Ok(())
    }

    /// 函数链式调用后仍需使用
    // 通过 函数返回所有权 实现链式操作后回收变量：
    #[test]
    fn test_life_cycle4() -> Result<(), ()> {
        let user = User {
            name: String::from("Charlie"),
            age: 40,
        };

        // 链式调用后返回所有权
        let user = user.with_age(41);

        // 继续使用
        println!("Updated age: {}", user.age);
        Ok(())
    }
}
