struct MyStruct1;

impl MyStruct1 {
    // 方法 1：不可变引用
    fn method_ref(&self) {
        println!("不可变方法调用");
    }

    // 方法 2：可变引用
    fn method_mut_ref(&mut self) {
        println!("可变方法调用");
    }
}

struct MyStruct2;

impl MyStruct2 {
    // 方法消耗所有权，但返回 Self 以便继续使用
    fn take_ownership(self) -> Self {
        println!("消耗所有权的方法");
        self // 返回所有权
    }
}

struct MyStruct3;

impl MyStruct3 {
    fn method_box(self: Box<Self>) -> Box<Self> {
        println!("通过 Box 调用方法");
        self // 返回 Box，保留所有权
    }
}

struct MyStruct4 {
    data: u32,
}

impl MyStruct4 {
    fn modify(&mut self) {
        self.data += 1;
    }
}

/// 生命周期管理
///
/// | 场景 |	方案	 | 优点 |	限制   |
/// | ---- | ---- | ---- | ---- |
/// | 方法无需修改结构体 |	不可变引用 (&self) |	可无限次调用，无副作用 |	只能读取数据，不能修改 |
/// | 方法需要修改结构体 |	可变引用 (&mut self)	| 允许修改数据	| 同一作用域内只能存在一个可变引用 |
/// | 方法必须消耗所有权 |	链式调用返回 Self |	可复用所有权 |	每次调用需重新绑定变量 |
/// | 需要跨作用域多次修改 |	作用域隔离	|灵活控制生命周期	| 代码结构稍复杂 |
#[cfg(test)]
mod tests {
    use super::*;

    /// 1. 方法不消耗所有权（推荐）
    // 若方法通过 引用（&self 或 &mut self）操作，可直接多次调用：
    #[test]
    fn test_life_cycle1() {
        let mut s = MyStruct1;

        // 多次调用不可变方法
        s.method_ref(); // 第一次
        s.method_ref(); // 第二次

        // 多次调用可变方法（需确保作用域不重叠）
        s.method_mut_ref(); // 第一次
        s.method_mut_ref(); // 第二次（同一作用域下允许，因前一次可变引用已释放）
    }

    /// 2. 方法消耗所有权但需复用
    /// 若方法需要所有权（参数为 self），需通过 链式调用 或 返回所有权 实现复用：
    #[test]
    fn test_life_cycle2() {
        let s = MyStruct2;

        // 链式调用两次
        s.take_ownership().take_ownership();

        let s1 = MyStruct2;
        // 分步调用（需重新绑定变量）
        let s1 = s1.take_ownership(); // 第一次
        s1.take_ownership();         // 第二次
    }

    /// 3. 使用 Box 智能指针
    // 通过 Box 将结构体分配到堆上，避免直接移动：
    #[test]
    fn test_life_cycle3() {
        let s = Box::new(MyStruct3);

        // 多次调用
        let s = s.method_box(); // 第一次
        s.method_box();         // 第二次
    }

    #[test]
    fn test_life_cycle4() {
        let mut s = MyStruct4 { data: 0 };
        // 第一次可变借用（隔离作用域）
        {
            s.modify();
        }
        // 第二次可变借用（作用域已结束，允许再次借用）
        {
            s.modify();
        }
        println!("最终值: {}", s.data); // 输出: 2
    }
 }
