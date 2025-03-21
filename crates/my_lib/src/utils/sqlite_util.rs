use rusqlite::{Connection, Result, Row, params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};

/// SQLite 数据库工具类
pub struct SqliteDb {
    conn: Connection,
}

impl SqliteDb {
    /// 连接到 SQLite 数据库（若不存在则创建）
    pub fn connect<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(SqliteDb { conn })
    }

    /// 执行 SQL 语句（如 CREATE, INSERT, UPDATE）
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        self.conn.execute(sql, params)
    }

    /// 查询数据并映射到结构体（需实现 From<Row>）
    pub fn query<T: for<'a> From<&'a Row<'a>> + FromSql>(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<Vec<T>> {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params, |row| Ok(T::from(row)))?;
        rows.collect()
    }

    /// 开启事务
    pub fn begin_transaction(&mut self) -> Result<usize> {
        self.conn.execute("BEGIN TRANSACTION;", [])
    }

    /// 提交事务
    pub fn commit(&mut self) -> Result<usize> {
        self.conn.execute("COMMIT;", [])
    }

    /// 回滚事务
    pub fn rollback(&mut self) -> Result<usize> {
        self.conn.execute("ROLLBACK;", [])
    }

    /// 检查表是否存在（工具方法）
    pub fn table_exists(&self, table_name: &str) -> Result<bool> {
        let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name=?1";
        self.conn
            .query_row(sql, [table_name], |row| row.get::<_, String>(0))
            .optional()
            .map(|opt| opt.is_some())
    }
}

// 示例数据模型（可选）
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    id: i64,
    name: String,
    email: String,
}

impl From<&Row<'_>> for User {
    fn from(row: &Row) -> Self {
        User {
            id: row.get("id").unwrap(),
            name: row.get("name").unwrap(),
            email: row.get("email").unwrap(),
        }
    }
}

impl FromSql for User {
    /// 将数据库中的 ValueRef 转换为 User 类型。
    /// 如果值的类型不匹配或解析失败，则返回错误。
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(text) => {
                // 假设 User 可以从字符串反序列化
                serde_json::from_slice::<User>(text).map_err(|_| FromSqlError::InvalidType)
            }
            ValueRef::Blob(blob) => {
                // 假设 User 可以从二进制数据反序列化
                serde_json::from_slice::<User>(blob).map_err(|_| FromSqlError::InvalidType)
            }
            _ => Err(FromSqlError::InvalidType), // 不支持的类型
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    /// 测试数据库连接和表创建
    #[test]
    fn test_connect_and_create_table() -> Result<()> {
        let temp_db = NamedTempFile::new().unwrap();
        let db = SqliteDb::connect(temp_db.path())?;

        // 创建测试表
        let sql = "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )";
        db.execute(sql, &[])?;

        // 验证表是否存在
        assert!(db.table_exists("users")?);
        Ok(())
    }

    /// 测试插入和查询数据
    #[test]
    fn test_insert_and_query() -> Result<()> {
        let temp_db = NamedTempFile::new().unwrap();
        let db = SqliteDb::connect(temp_db.path())?;

        // 创建表
        db.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)",
            &[],
        )?;

        // 插入数据
        db.execute(
            "INSERT INTO users (name, email) VALUES (?1, ?2)",
            &[&"Alice", &"alice@example.com"],
        )?;

        // 查询数据
        let users: Vec<User> = db.query("SELECT * FROM users", &[])?;
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "Alice");
        Ok(())
    }

    /// 测试事务回滚
    #[test]
    fn test_transaction_rollback() -> Result<()> {
        let temp_db = NamedTempFile::new().unwrap();
        let mut db = SqliteDb::connect(temp_db.path())?;

        // 创建表
        db.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)",
            &[],
        )?;

        // 开启事务
        db.begin_transaction()?;

        // 插入数据（未提交）
        db.execute(
            "INSERT INTO users (name, email) VALUES (?1, ?2)",
            &[&"Bob", &"bob@example.com"],
        )?;

        // 回滚事务
        db.rollback()?;

        // 验证数据未持久化
        let count: i64 = db.conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
        assert_eq!(count, 0);
        Ok(())
    }

}
