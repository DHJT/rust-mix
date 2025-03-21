// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum SqliteError {
//     #[error("Database error: {0}")]
//     DatabaseError(#[from] sqlx::Error),
//     #[error("Connection pool error: {0}")]
//     PoolError(String),
// }
//
// use sqlx::{Sqlite, SqlitePool, Row};
// use std::path::Path;
// use std::sync::Arc;
// use tokio::sync::Mutex;
//
// /// 异步 SQLite 连接池工具类
// #[derive(Clone)]
// pub struct AsyncSqliteDb {
//     pool: SqlitePool,
//     // transaction: Option<Arc<Mutex<Transaction>>>,
//     transaction: Option<Arc<Mutex<sqlx::Transaction<'static, Sqlite>>>>,
// }
//
// impl AsyncSqliteDb {
//     /// 创建连接池（支持内存数据库）
//     pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self, SqliteError> {
//         let conn_str = format!("sqlite:{}", path.as_ref().display());
//         let pool = SqlitePool::connect(&conn_str)
//             .await
//             .map_err(|e| SqliteError::PoolError(e.to_string()))?;
//
//         Ok(Self {
//             pool,
//             transaction: None,
//         })
//     }
//
//     /// 执行 SQL 语句（CREATE/INSERT/UPDATE）
//     pub async fn execute(&self, sql: &str, params: &[&(dyn sqlx::Encode<'_, Sqlite> + Sync)]) -> Result<u64, SqliteError> {
//         let result = sqlx::query(sql)
//             .bind_all(params)
//             .execute(&self.pool)
//             .await?;
//
//         Ok(result.rows_affected())
//     }
//
//     /// 查询数据并映射到结构体
//     pub async fn query<T: From<sqlx::sqlite::SqliteRow>>(&self, sql: &str, params: &[&(dyn sqlx::Encode<'_, Sqlite> + Sync)]) -> Result<Vec<T>, SqliteError> {
//         let rows = sqlx::query(sql)
//             .bind_all(params)
//             .fetch_all(&self.pool)
//             .await?;
//
//         Ok(rows.into_iter().map(|row| T::from(row)).collect())
//     }
//
//     /// 开启事务
//     pub async fn begin_transaction(&mut self) -> Result<(), SqliteError> {
//         let tx = self.pool.begin().await?;
//         self.transaction = Some(Mutex::new(tx));
//         Ok(())
//     }
//
//     /// 提交事务
//     pub async fn commit(&mut self) -> Result<(), SqliteError> {
//         if let Some(tx) = self.transaction.take() {
//             // let mut tx = tx.lock().await;
//             let mut tx = tx.lock().await;
//             tx.commit().await?;
//         }
//         Ok(())
//     }
//
//     /// 回滚事务
//     pub async fn rollback(&mut self) -> Result<(), SqliteError> {
//         if let Some(tx) = self.transaction.take() {
//             let mut tx = tx.lock().await;
//             tx.rollback().await?;
//         }
//         Ok(())
//     }
//
//     /// 检查表是否存在
//     pub async fn table_exists(&self, table_name: &str) -> Result<bool, SqliteError> {
//         let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name=?";
//         let result: Option<String> = sqlx::query_scalar(sql)
//             .bind(table_name)
//             .fetch_optional(&self.pool)
//             .await?;
//
//         Ok(result.is_some())
//     }
// }
//
// // 示例数据模型
// #[derive(Debug, sqlx::FromRow, serde::Serialize, PartialEq)]
// pub struct User {
//     pub id: i64,
//     pub name: String,
//     pub email: String,
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tempfile::NamedTempFile;
//
//     #[tokio::test]
//     async fn test_connect_and_create_table() -> Result<(), SqliteError> {
//         let temp_db = NamedTempFile::new()?;
//         let db = AsyncSqliteDb::connect(temp_db.path()).await?;
//
//         // 创建测试表
//         let sql = "CREATE TABLE IF NOT EXISTS users (
//             id INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             email TEXT NOT NULL
//         )";
//         db.execute(sql, &[]).await?;
//
//         // 验证表存在
//         assert!(db.table_exists("users").await?);
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_insert_and_query() -> Result<(), SqliteError> {
//         let temp_db = NamedTempFile::new()?;
//         let db = AsyncSqliteDb::connect(temp_db.path()).await?;
//
//         // 创建表
//         db.execute(
//             "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)",
//             &[],
//         )
//             .await?;
//
//         // 插入数据
//         db.execute(
//             "INSERT INTO users (name, email) VALUES (?, ?)",
//             &[&"Alice".to_string(), &"alice@example.com".to_string()],
//         )
//             .await?;
//
//         // 查询数据
//         let users: Vec<User> = db.query("SELECT * FROM users", &[]).await?;
//         assert_eq!(users.len(), 1);
//         assert_eq!(users[0].name, "Alice");
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_transaction_rollback() -> Result<(), SqliteError> {
//         let temp_db = NamedTempFile::new()?;
//         let mut db = AsyncSqliteDb::connect(temp_db.path()).await?;
//
//         // 创建表
//         db.execute(
//             "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)",
//             &[],
//         )
//             .await?;
//
//         // 开启事务
//         db.begin_transaction().await?;
//
//         // 插入数据（未提交）
//         db.execute(
//             "INSERT INTO users (name, email) VALUES (?, ?)",
//             &[&"Bob".to_string(), &"bob@example.com".to_string()],
//         )
//             .await?;
//
//         // 回滚事务
//         db.rollback().await?;
//
//         // 验证数据未持久化
//         let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
//             .fetch_one(&db.pool)
//             .await?;
//         assert_eq!(count, 0);
//         Ok(())
//     }
// }
