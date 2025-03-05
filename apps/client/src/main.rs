use bytes::{BytesMut, Buf, BufMut};
use tokio::{
    net::TcpStream,
    io::{AsyncReadExt, AsyncWriteExt},
    time::{sleep, Duration, timeout},
};
use std::{error::Error, time::Instant};

// 客户端配置
const SERVER_ADDR: &str = "127.0.0.1:8080";
const MAX_RETRIES: u32 = 5;
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(1);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(60);  // 心

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut retries = 0;
    loop {
        match connect_with_retry(SERVER_ADDR, retries).await {
            Ok(mut stream) => {
                println!("Connected to server");
                retries = 0;
                if let Err(e) = handle_client(&mut stream).await {
                    println!("Connection error: {}", e);
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
                retries = (retries + 1).min(MAX_RETRIES);
                let delay = INITIAL_RETRY_DELAY * 2u32.pow(retries);
                println!("Retrying in {:?}...", delay);
                sleep(delay).await;
            }
        }
    }
}

// 指数退避重连
async fn connect_with_retry(addr: &str, retries: u32) -> Result<TcpStream, Box<dyn Error>> {
    let delay = INITIAL_RETRY_DELAY * 2u32.pow(retries);
    for _ in 0..=retries {
        match TcpStream::connect(addr).await {
            Ok(stream) => return Ok(stream),
            Err(_e) => {
                if delay.as_secs() > 0 {
                    sleep(delay).await;
                }
            }
        }
    }
    Err("Max retries exceeded".into())
}

// 处理客户端逻辑
async fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = BytesMut::with_capacity(4096);
    let mut last_heartbeat = Instant::now();
    let ping_packet = BytesMut::from("PING");
    let data_packet = BytesMut::from("DATA");

    loop {
        // 发送心跳包前检查连接是否可写
        if stream.writable().await.is_err() {
            return Err("Connection closed".into());
        }
        // 发送心跳包
        if last_heartbeat.elapsed() > HEARTBEAT_INTERVAL {
            send_packet(stream, &ping_packet).await?;
            last_heartbeat = Instant::now();
        }

        send_packet(stream, &data_packet).await?;

        // 读取数据 (带超时)
        match timeout(HEARTBEAT_TIMEOUT, stream.read_buf(&mut buf)).await {
            Ok(Ok(0)) => return Ok(()), // 服务端关闭连接
            Ok(Ok(_)) => {
                while let Some(packet) = parse_packet(&mut buf) {
                    process_packet(packet).await?;
                }
            }
            Ok(Err(e)) => return Err(e.into()),
            Err(_) => {
                println!("Heartbeat timeout, retrying...");
                continue; // 重试读取
            }
        }
    }
}

// 解析数据包 (与服务端相同)
fn parse_packet(buf: &mut BytesMut) -> Option<BytesMut> {
    // ... 同服务端代码 ...
    if buf.len() < 4 {
        return None;
    }

    let len = {
        let len_bytes = &buf[..4];
        u32::from_be_bytes(len_bytes.try_into().unwrap()) as usize
    };

    if buf.len() < 4 + len {
        return None;
    }

    buf.advance(4);
    Some(buf.split_to(len))
}

// 处理服务端响应
async fn process_packet(packet: BytesMut) -> Result<(), Box<dyn Error>> {
    match &packet[..] {
        b"PONG" => println!("Received PONG"),
        _data => {
            println!("Received data: {:?}", _data)
        },
    }
    Ok(())
}

// 发送数据包 (与服务端相同)
async fn send_packet(stream: &mut TcpStream, data: &BytesMut) -> Result<(), Box<dyn Error>> {
    // ... 同服务端代码 ...
    let mut buf = BytesMut::with_capacity(4 + data.len());
    buf.put_u32(data.len() as u32);
    buf.extend_from_slice(&data);
    stream.write_all(&buf).await?;
    Ok(())
}