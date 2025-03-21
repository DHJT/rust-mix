use redis::{Client, RedisResult};
use deadpool_redis::{Config, Runtime};
use futures::StreamExt;
use mockall::automock;
use redis::aio::MultiplexedConnection;

async fn connect_redis() -> redis::RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut _con = client.get_multiplexed_async_connection().await?;
    // 连接成功后可执行操作
    Ok(())
}

async fn set_value(con: &mut MultiplexedConnection, key: &str, value: &str) -> RedisResult<()> {
    redis::cmd("SET")
        .arg(key)
        .arg(value)
        .query_async(con)
        .await
}

async fn get_value(con: &mut MultiplexedConnection, key: &str) -> RedisResult<String> {
    redis::cmd("GET")
        .arg(key)
        .query_async(con)
        .await
}

/// 事务处理
async fn transaction(con: &mut MultiplexedConnection) -> RedisResult<()> {
    let mut pipe = redis::pipe();
    pipe.atomic()
        .cmd("SET").arg("counter").arg(0).ignore()
        .cmd("INCR").arg("counter")
        .cmd("GET").arg("counter");

    let (_, incr_result, get_result): ((), i32, i32) = pipe.query_async(con).await?;
    println!("Counter after increment: {}", get_result);
    Ok(())
}

/// 发布订阅
async fn pubsub() -> redis::RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut pubcon = client.get_multiplexed_async_connection().await?;
    let mut subcon = client.get_async_connection().await?.into_pubsub();

    subcon.subscribe("news").await?;

    tokio::spawn(async move {
        let _: () = redis::cmd("PUBLISH")
            .arg("news")
            .arg("Important Message!")
            .query_async(&mut pubcon)
            .await
            .unwrap();
    });

    let msg = subcon.on_message().next().await;
    // println!("Received: {:?}", msg.unwrap().get_payload()::<String>().unwrap());
    Ok(())
}

/// 重试操作
async fn retry_operation<F, T>(mut op: F, max_retries: usize) -> RedisResult<T>
where
    F: FnMut() -> futures::future::BoxFuture<'static, RedisResult<T>>,
{
    let mut attempts = 0;
    loop {
        match op().await {
            Ok(val) => return Ok(val),
            Err(_e) if attempts < max_retries => {
                attempts += 1;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

/// 创建连接池 create_pool("redis://127.0.0.1/")
async fn create_pool(url: &str) -> deadpool_redis::Pool {
    let cfg = Config::from_url(url);
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}

/// 管道操作
async fn pipeline(con: &mut MultiplexedConnection) -> RedisResult<()> {
    let mut pipe = redis::pipe();
    pipe.cmd("SET")
        .arg("key1")
        .arg("value1")
        .ignore()
        .cmd("GET")
        .arg("key1");

    let (_, value): ((), String) = pipe.query_async(con).await?;
    println!("Pipelined result: {}", value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use redis::aio::MultiplexedConnection;
    use redis::Client;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[automock]
    trait RedisConnection {
        async fn get_multiplexed_async_connection(&self) -> RedisResult<MultiplexedConnection>;
        async fn get_async_connection(&self) -> RedisResult<redis::aio::Connection>;
    }

    #[tokio::test]
    async fn connect_redis_SuccessfulConnection_ReturnsOk() {
        // let mut mock_client = MockRedisConnection::new();
        // mock_client.expect_get_multiplexed_async_connection()
        //     .returning(|| Ok(MultiplexedConnection::new()));
        //
        // let client = Arc::new(Mutex::new(mock_client));
        // let result = connect_redis().await;
        // assert!(result.is_ok());
    }

}
