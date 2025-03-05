use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, Result};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    is_active: bool,
    roles: Vec<String>,
}

pub fn serialize() -> Result<()> {
    // JSON 字符串
    let json_str = r#"
        {
          "id": 1001,
          "name": "Alice",
          "email": "alice@example.com",
          "is_active": true,
          "roles": ["admin", "user"]
        }
    "#;

    // 反序列化
    let user: User = serde_json::from_str(json_str)?;

    // 验证结果
    println!("Deserialized User: {:#?}", user);
    Ok(())
}

pub fn deserialize() -> Result<()> {
    // 创建结构体实例
    let user = User {
        id: 1001,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        is_active: true,
        roles: vec!["admin".to_string(), "user".to_string()],
    };

    // 序列化为格式化的 JSON 字符串
    let json_str = to_string_pretty(&user)?;

    // 输出到控制台
    println!("Serialized User:\n{}", json_str);

    Ok(())
}